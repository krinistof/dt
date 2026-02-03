import { sync } from "uq-client";
import { client, getIdentity } from "./state.js";

/**
 * Sends a log message to the backend log collector.
 * @param level The log level (e.g., 'ERROR', 'WARN', 'INFO').
 * @param message The message to log.
 */
async function logToServer(level: string, message: string) {
	const log = { level, message, timestamp: new Date().toISOString() };
	sendLog(log);
}

const enc = new TextEncoder();

async function sendLog(log: object) {
	const identity = getIdentity();
	if (!navigator.onLine || !identity) {
		return;
	}

	try {
		const topicPk = enc.encode("client_logs");
		const payload = enc.encode(JSON.stringify(log));

		await sync(client, 9223372036854775807n, identity.publicKey, {
			topicPk,
			payload,
			author: identity,
		});
	} catch (error) {
		console.error("Failed to send log:", error);
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
window.addEventListener("error", (event: ErrorEvent) => {
	console.log("Global error handler caught:", event.error);
	logToServer("ERROR", formatError(event.error));
});

// Catch all unhandled promise rejections
window.addEventListener(
	"unhandledrejection",
	(event: PromiseRejectionEvent) => {
		console.log("Global unhandled rejection handler caught:", event.reason);
		logToServer("ERROR", formatError(event.reason));
	},
);

// --- Global Log/Warn Overrides ---

// Override console.warn
const originalWarn = console.warn;
console.warn = (...args: unknown[]) => {
	originalWarn.apply(console, args);
	logToServer("WARN", args.map((arg) => String(arg)).join(" "));
};

// Override console.error (optional, as uncaught errors are already handled)
// This can be useful for logging errors that are caught but still logged with console.error
const originalError = console.error;
console.error = (...args: unknown[]) => {
	originalError.apply(console, args);
	// Avoid double-logging errors caught by the global error handler
	if (args[0] && !(args[0] instanceof Error)) {
		logToServer("ERROR", args.map((arg) => String(arg)).join(" "));
	}
};

console.log("Global loggers and error handlers initialized.");
