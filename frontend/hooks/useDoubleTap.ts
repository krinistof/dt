import { useRef, useCallback, useEffect } from 'react';

interface DoubleTapOptions {
  onDoubleTap: () => void;
  delay?: number; // Milliseconds to wait for a second tap
}

export function useDoubleTap({ onDoubleTap, delay = 300 }: DoubleTapOptions): () => void {
  const clickTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);
  const clickCount = useRef(0);

  // Cleanup timeout on component unmount
  useEffect(() => {
    return () => {
      if (clickTimeout.current) {
        clearTimeout(clickTimeout.current);
      }
    };
  }, []);

  const handler = useCallback(() => {
    clickCount.current += 1;

    if (clickCount.current === 1) {
      // Start a timer for the second click
      clickTimeout.current = setTimeout(() => {
        clickCount.current = 0; // Reset count if an_action_is_not_performed_within_delay
      }, delay);
    } else if (clickCount.current === 2) {
      // Double tap detected
      if (clickTimeout.current) {
        clearTimeout(clickTimeout.current); // Clear the timeout for the first click
        clickTimeout.current = null;
      }
      clickCount.current = 0; // Reset count
      onDoubleTap(); // Execute the double tap action
    }
  }, [onDoubleTap, delay]);

  return handler;
}