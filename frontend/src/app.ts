// Import the logger to activate the global handlers
import './logger';

const logButton = document.getElementById('logButton');
const warnButton = document.getElementById('warnButton');
const errorButton = document.getElementById('errorButton');

if (logButton) {
  logButton.addEventListener('click', () => {
    console.log("This is a test log message.");
  });
}

if (warnButton) {
  warnButton.addEventListener('click', () => {
    console.warn("This is a test warning message.");
  });
}

if (errorButton) {
  errorButton.addEventListener('click', () => {
    console.log('Throwing a test error...');
    // This error will be caught by the global error handler in logger.ts
    throw new Error("This is a test error to be caught by the global handler.");
  });
}

// Example of an unhandled promise rejection
Promise.reject("This is a test of an unhandled promise rejection.");