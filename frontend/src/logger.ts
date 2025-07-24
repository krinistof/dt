
import { LogServiceClient } from './grpc';
import { addLog, getAllLogs, clearLogs } from './idb';

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
    console.log("Offline, skipping log sync.");
    return;
  }

  const logs = await getAllLogs();
  if (logs.length === 0) {
    return;
  }

  console.log(`Attempting to sync ${logs.length} logs.`);

  try {
    const logMessages = logs.map(log => `[${log.level}] ${log.message}`);
    // This is a simplification. In a real-world scenario, you would
    // likely send the logs in batches and handle partial failures.
    const response = await LogServiceClient.log({ messages: logMessages });

    if (response.success) {
      console.log("Successfully synced logs.");
      await clearLogs();
    } else {
      console.error("Log collector reported a failure for synced logs.");
    }
  } catch (error) {
    console.error("Failed to sync logs to the collector:", error);
  }
}

/**
 * Formats an error object into a string for logging.
 * @param error The error object to format.
 * @returns A string representation of the error.
 */
function formatError(error: any): string {
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
console.warn = (...args: any[]) => {
  originalWarn.apply(console, args);
  logToServer('WARN', args.map(arg => String(arg)).join(' '));
};

// Override console.error (optional, as uncaught errors are already handled)
// This can be useful for logging errors that are caught but still logged with console.error
const originalError = console.error;
console.error = (...args: any[]) => {
  originalError.apply(console, args);
  // Avoid double-logging errors caught by the global error handler
  if (args[0] && !(args[0] instanceof Error)) {
     logToServer('ERROR', args.map(arg => String(arg)).join(' '));
  }
};

// --- Network Status Handling ---

window.addEventListener('online', () => {
  console.log("Network connection restored. Attempting to sync logs.");
  syncLogs();
});

window.addEventListener('offline', () => {
  console.log("Network connection lost. Logs will be stored locally.");
});

// --- Initial Sync ---

// Attempt to sync logs when the application starts
syncLogs();

console.log("Global loggers and error handlers initialized.");
