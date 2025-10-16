import { DtServiceClient } from './grpc';
import { getAllEvents, clearEvents, putPost, getPost, Post } from './idb';
import { ConnectError, Code } from '@connectrpc/connect';

const RETRY_TIMEOUT_MS: number = 3000;
let retryTimeoutId: number | null = null;

function getUserToken(): string | null {
    return localStorage.getItem('user_token');
}

async function submitEvents() {
  const userToken = getUserToken();
  if (!userToken || !navigator.onLine) {
    return;
  }

  const eventsWithKeys = await getAllEvents();
  if (eventsWithKeys.length === 0) {
    return;
  }

  console.log(`Attempting to sync ${eventsWithKeys.length} local events.`);

  for (const eventWithKey of eventsWithKeys) {
    try {
      const response = await DtServiceClient.submitEvent({
        userToken,
        event: {
          clientKey: eventWithKey.event.client_key,
          action: eventWithKey.event.action,
          payload: JSON.stringify(eventWithKey.event.payload),
          createdAt: BigInt(eventWithKey.event.created_at),
        },
      });
      if (response.success) {
        await clearEvents([eventWithKey.key]);
        console.log(`Successfully synced and cleared event.`);
      } else {
        console.error('Server indicated failure for a synced event.');
      }
    } catch (error) {
      handleSyncError(error, submitEvents);
      return;
    }
  }
}

export async function syncInitialState() {
    const userToken = getUserToken();
    if (!userToken || !navigator.onLine) {
        return;
    }

    try {
        console.log('Attempting to sync initial state...');
        const response = await DtServiceClient.getInitialState({ userToken });
        
        for (const post of response.posts) {
            await putPost(new Post({
                content_hash: post.postId,
                content: post.content,
                base_score: post.baseScore,
                user_score: post.userScore,
            }));
        }
        
        localStorage.setItem('last_sync_timestamp', response.serverTimestamp.toString());
        window.dispatchEvent(new CustomEvent('datachanged'));
        console.log('Initial state synced successfully.');
    } catch (error) {
        handleSyncError(error, syncInitialState);
    }
}

export async function syncEvents() {
    const userToken = getUserToken();
    if (!userToken || !navigator.onLine) {
        return;
    }

    await submitEvents();

    const lastSyncTimestamp = localStorage.getItem('last_sync_timestamp');
    if (!lastSyncTimestamp) {
        console.warn('No last sync timestamp found. Skipping delta sync.');
        return;
    }

    try {
        const response = await DtServiceClient.syncEvents({
            userToken,
            sinceTimestamp: BigInt(lastSyncTimestamp),
        });

        if (response.events.length > 0) {
            for (const event of response.events) {
                const payload = JSON.parse(event.payload);
                if (event.action === 'post') {
                    const post = new Post({
                        content_hash: payload.content_hash,
                        content: payload.content,
                        base_score: 0,
                        user_score: 0,
                    });
                    await putPost(post);
                } else if (event.action === 'score_update') {
                    const post = await getPost(payload.post_id);
                    if (post) {
                        post.base_score = Number(payload.total_score) - post.user_score;
                        await putPost(post);
                    } else {
                        console.warn(`Received score_update for unknown post: ${payload.post_id}`);
                    }
                }
            }
            window.dispatchEvent(new CustomEvent('datachanged'));
        }

        localStorage.setItem('last_sync_timestamp', response.serverTimestamp.toString());
    } catch (error) {
        handleSyncError(error, syncEvents);
    }
}

export function handleSyncError(error: unknown, retryFunction: () => void) {
    if (error instanceof ConnectError && error.code === Code.Unavailable) {
      console.info("Failed to sync: Server unavailable.");
    } else {
      let isNetworkError = false;
      if (error instanceof Error) {
        const errorMessage = error.message.toLowerCase();
        const errorCause = error.cause instanceof Error ? error.cause.message.toLowerCase() : "";

        if (errorMessage.includes('failed to fetch') || errorMessage.includes('load failed') || errorMessage.includes('http 404')) {
          isNetworkError = true;
        } else if (errorCause.includes('failed to fetch')) {
          isNetworkError = true;
        }
      }

      if (isNetworkError) {
        console.info("Failed to sync to the server due to a network error.");
        return;
      } else {
        console.error("Failed to sync to the server:", error);
      }
    }

    let isFetchError = false;
    if (error instanceof Error) {
        if (error.message.toLowerCase().includes('failed to fetch')) {
            isFetchError = true;
        }
        if (!isFetchError && error.cause instanceof Error) {
            if (error.cause.message.toLowerCase().includes('failed to fetch')) {
                isFetchError = true;
            }
        }
    }

    if (isFetchError) {
        console.info('Failed to sync to the server due to a network error.');
        return;
    } else {
        console.error('Failed to sync to the server:', error);
    }

    if (retryTimeoutId) {
        return;
    }

    console.info(`Retrying in ${RETRY_TIMEOUT_MS} ms...`);
    retryTimeoutId = setTimeout(() => {
        retryTimeoutId = null;
        retryFunction();
    }, RETRY_TIMEOUT_MS);
}

window.addEventListener('online', () => {
    syncEvents();
});

