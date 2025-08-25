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
  const syncedEventKeys: IDBValidKey[] = [];

  try {
    const events = eventsWithKeys.map(ek => ek.event);
    const response = await DtServiceClient.sync({ events: events.map(e => ({...e, payload: JSON.stringify(e.payload)})) });
    if (response.success) {
        syncedEventKeys.push(...eventsWithKeys.map(ek => ek.key));
    } else {
        console.error("Event sync reported a failure for synced events.");
    }

    if (syncedEventKeys.length > 0) {
      await clearEvents(syncedEventKeys);
      console.log(`Successfully synced and cleared ${syncedEventKeys.length} events.`);
    }

    if (retryTimeoutId) {
      clearTimeout(retryTimeoutId);
      retryTimeoutId = null;
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
      console.info("Failed to sync events to the server due to a network error.");
    } else {
      console.error("Failed to sync events to the server:", error);
    }
    
    if (retryTimeoutId) {
      return;
    }

    console.info(`Retrying in ${RETRY_TIMEOUT_MS} ms...`);
    retryTimeoutId = setTimeout(() => {
      retryTimeoutId = null;
      syncEvents();
    }, RETRY_TIMEOUT_MS);
  }
}

window.addEventListener('online', () => {
    syncEvents();
});

syncEvents();
