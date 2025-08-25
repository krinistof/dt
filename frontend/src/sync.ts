import { DtServiceClient } from './grpc';
import { getAllEvents, clearEvents } from './idb';

const RETRY_TIMEOUT_MS: number = 3000;
let retryTimeoutId: number | null = null;

export async function syncEvents() {
  if (!navigator.onLine) {
    return;
  }

  const eventsWithKeys = await getAllEvents();
  if (eventsWithKeys.length === 0) {
    return;
  }

  console.log(`Attempting to sync ${eventsWithKeys.length} events.`);

  for (const eventWithKey of eventsWithKeys) {
    try {
      const response = await DtServiceClient.submitEvent({
        userToken: 'dev', //TODO
        event: {
          ...eventWithKey.event,
          payload: JSON.stringify(eventWithKey.event.payload),
        },
      });
      if (response.success) {
        await clearEvents([eventWithKey.key]);
        console.log(`Successfully synced and cleared event.`);
      } else {
        console.error('Event sync reported a failure for synced event.');
      }
    } catch (error) {
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
        console.info(
          'Failed to sync events to the server due to a network error.',
        );
      } else {
        console.error('Failed to sync events to the server:', error);
      }

      if (retryTimeoutId) {
        return;
      }

      console.info(`Retrying in ${RETRY_TIMEOUT_MS} ms...`);
      retryTimeoutId = setTimeout(() => {
        retryTimeoutId = null;
        syncEvents();
      }, RETRY_TIMEOUT_MS);
      return;
    }
  }

  if (retryTimeoutId) {
    clearTimeout(retryTimeoutId);
    retryTimeoutId = null;
  }
}

window.addEventListener('online', () => {
    syncEvents();
});

syncEvents();
