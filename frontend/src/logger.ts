
import { LogServiceClient } from './grpc';
import { addLog, getAllLogs, clearLogs } from './idb';

const RETRY_TIMEOUT_MS: number = 3000;
let retryTimeoutId: number | null = null;

/**
 * Sends a log message to the backend log collector.
 * @param level The log level (e.g., 'ERROR', 'WARN', 'INFO').
 * @param message The message to log.
 */
async function logToServer(level: string, message: string) {
  const log = { level, message, timestamp: new Date().toISOString() };
  await addLog(log);
  syncLogs();
}

async function syncLogs() {
  if (!navigator.onLine) {
    return;
  }

  const logsWithKeys = await getAllLogs();
  if (logsWithKeys.length === 0) {
    return;
  }

  console.log(`Attempting to sync ${logsWithKeys.length} logs.`);
  const syncedLogKeys: IDBValidKey[] = [];

  try {
    for (const { key, log } of logsWithKeys) {
      const response = await LogServiceClient.log({ message: `[${log.level}] ${log.message}` });
      if (response.success) {
        syncedLogKeys.push(key);
      } else {
        console.error("Log collector reported a failure for synced logs.");
      }
    }

    if (syncedLogKeys.length > 0) {
      await clearLogs(syncedLogKeys);
      console.log(`Successfully synced and cleared ${syncedLogKeys.length} logs.`);
    }

    if (retryTimeoutId) {
      clearTimeout(retryTimeoutId);
      retryTimeoutId = null;
    }
  } catch (error) {
    let isNetworkError = false;
    if (error instanceof Error) {
      const errorMessage = error.message.toLowerCase();
      const errorCause = error.cause instanceof Error ? error.cause.message.toLowerCase() : "";

      if (errorMessage.includes('failed to fetch') || errorMessage.includes('load failed')) {
        isNetworkError = true;
      } else if (errorCause.includes('failed to fetch')) {
        isNetworkError = true;
      }
    }

    if (isNetworkError) {
      console.info("Failed to sync logs to the collector due to a network error.");
    } else {
      console.error("Failed to sync logs to the collector:", error);
    }
    
    if (retryTimeoutId) {
      return;
    }

    console.info(`Retrying in ${RETRY_TIMEOUT_MS} ms...`);
    retryTimeoutId = setTimeout(() => {
      retryTimeoutId = null;
      syncLogs();
    }, RETRY_TIMEOUT_MS);
  }
}

/**
 * Formats an error object into a string for logging.
 * @param error The error object to format.
 * @returns A string representation of the error.
 */
function formatError(error: unknown): string {
  if (error instanceof Error && error.stack) {
    return error.stack;
  }
  return String(error);
}

// --- Global Error Handlers ---

// Catch all uncaught synchronous errors and script errors
window.addEventListener('error', (event: ErrorEvent) => {
  console.log("Global error handler caught:", event.error);
  logToServer('ERROR', formatError(event.error));
});

// Catch all unhandled promise rejections
window.addEventListener('unhandledrejection', (event: PromiseRejectionEvent) => {
  console.log("Global unhandled rejection handler caught:", event.reason);
  logToServer('ERROR', formatError(event.reason));
});

// --- Global Log/Warn Overrides ---

// Override console.warn
const originalWarn = console.warn;
console.warn = (...args: unknown[]) => {
  originalWarn.apply(console, args);
  logToServer('WARN', args.map(arg => String(arg)).join(' '));
};

// Override console.error (optional, as uncaught errors are already handled)
// This can be useful for logging errors that are caught but still logged with console.error
const originalError = console.error;
console.error = (...args: unknown[]) => {
  originalError.apply(console, args);
  // Avoid double-logging errors caught by the global error handler
  if (args[0] && !(args[0] instanceof Error)) {
     logToServer('ERROR', args.map(arg => String(arg)).join(' '));
  }
};

// --- Network Status Handling ---

window.addEventListener('online', () => {
  syncLogs();
});

// --- Initial Sync ---

// Attempt to sync logs when the application starts
syncLogs();
console.log("Global loggers and error handlers initialized.");
