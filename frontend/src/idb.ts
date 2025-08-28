// --- Type Definitions ---

export interface Post {
  post_id: string;
  content: string;
  base_score: number;
  user_score: number;
}

export interface Event {
  client_key: string;
  user_token: string;
  action: 'post' | 'vote';
  payload: { [key: string]: unknown };
  created_at: number;
}

export interface Log {
  level: string;
  message: string;
  timestamp: string;
}


// --- Database Setup ---

const DB_NAME = 'dt-cache';
const DB_VERSION = 2;

let db: IDBDatabase | null = null;

function getDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    if (db) {
      return resolve(db);
    }

    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onerror = () => {
      reject('Error opening IndexedDB');
    };

    request.onsuccess = (event) => {
      db = (event.target as IDBOpenDBRequest).result;
      resolve(db);
    };

    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains('logs')) {
        db.createObjectStore('logs', { autoIncrement: true });
      }
      if (!db.objectStoreNames.contains('posts')) {
        db.createObjectStore('posts', { keyPath: 'post_id' });
      }
      if (!db.objectStoreNames.contains('events')) {
        db.createObjectStore('events', { autoIncrement: true });
      }
    };
  });
}

// --- Log Functions ---

export async function addLog(log: Log) {
  const db = await getDb();
  const transaction = db.transaction('logs', 'readwrite');
  const store = transaction.objectStore('logs');
  store.add(log);
}

export function getAllLogs(): Promise<{ key: IDBValidKey; log: Log }[]> {
    return new Promise((resolve, reject) => {
        getDb().then(db => {
            const transaction = db.transaction('logs', 'readonly');
            const store = transaction.objectStore('logs');
            const request = store.openCursor();
            const logs: { key: IDBValidKey; log: Log }[] = [];

            request.onerror = () => {
                reject('Error getting logs from IndexedDB');
            };

            request.onsuccess = () => {
                const cursor = request.result;
                if (cursor) {
                    logs.push({ key: cursor.key, log: cursor.value });
                    cursor.continue();
                } else {
                    resolve(logs);
                }
            };
        }).catch(reject);
    });
}

export async function clearLogs(keys: IDBValidKey[]) {
    const db = await getDb();
    const transaction = db.transaction('logs', 'readwrite');
    const store = transaction.objectStore('logs');
    for (const key of keys) {
        store.delete(key);
    }
}

// --- DT Functions ---

export async function addEvent(event: Event) {
  const db = await getDb();
  const transaction = db.transaction('events', 'readwrite');
  const store = transaction.objectStore('events');
  store.add(event);
}

export function getAllEvents(): Promise<{ key: IDBValidKey; event: Event }[]> {
    return new Promise((resolve, reject) => {
        getDb().then(db => {
            const transaction = db.transaction('events', 'readonly');
            const store = transaction.objectStore('events');
            const request = store.openCursor();
            const events: { key: IDBValidKey; event: Event }[] = [];

            request.onerror = () => {
                reject('Error getting events from IndexedDB');
            };

            request.onsuccess = () => {
                const cursor = request.result;
                if (cursor) {
                    events.push({ key: cursor.key, event: cursor.value });
                    cursor.continue();
                } else {
                    resolve(events);
                }
            };
        }).catch(reject);
    });
}

export async function clearEvents(keys: IDBValidKey[]) {
    const db = await getDb();
    const transaction = db.transaction('events', 'readwrite');
    const store = transaction.objectStore('events');
    for (const key of keys) {
        store.delete(key);
    }
}

export async function putPost(post: Post) {
  const db = await getDb();
  const transaction = db.transaction('posts', 'readwrite');
  const store = transaction.objectStore('posts');
  store.put(post);
}

export function getPost(post_id: string): Promise<Post | null> {
    return new Promise((resolve, reject) => {
        getDb().then(db => {
            const transaction = db.transaction('posts', 'readonly');
            const store = transaction.objectStore('posts');
            const request = store.get(post_id);

            request.onerror = () => {
                reject('Error getting post from IndexedDB');
            };

            request.onsuccess = () => {
                if (request.result) {
                    resolve(request.result);
                } else {
                    resolve(null);
                }
            };
        }).catch(reject);
    });
}

export function getAllPosts(): Promise<Post[]> {
    return new Promise((resolve, reject) => {
        getDb().then(db => {
            const transaction = db.transaction('posts', 'readonly');
            const store = transaction.objectStore('posts');
            const request = store.getAll();

            request.onerror = () => {
                reject('Error getting posts from IndexedDB');
            };

            request.onsuccess = () => {
                resolve(request.result);
            };
        }).catch(reject);
    });
}