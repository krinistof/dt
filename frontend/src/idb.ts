// --- Type Definitions ---

export class Post {
  content_hash: string;
  content: string;
  base_score: number;
  user_score: number;

  constructor(data: { content_hash: string; content: string; base_score: number; user_score: number; }) {
    this.content_hash = data.content_hash;
    this.content = data.content;
    this.base_score = data.base_score;
    this.user_score = data.user_score;
  }

  get totalScore(): number {
    return this.base_score + this.user_score;
  }
}

export interface Event {
  user_token: string;
  action: 'post' | 'vote';
  payload: { [key: string]: any };
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
        db.createObjectStore('posts', { keyPath: 'content_hash' });
      }
      if (!db.objectStoreNames.contains('events')) {
        db.createObjectStore('events', { autoIncrement: true });
      }
    };
  });
}

// --- Log Functions ---

export async function addLog(log: any) {
  const db = await getDb();
  const transaction = db.transaction('logs', 'readwrite');
  const store = transaction.objectStore('logs');
  store.add(log);
}

export async function getAllLogs(): Promise<{ key: IDBValidKey; log: any }[]> {
    return new Promise(async (resolve, reject) => {
        const db = await getDb();
        const transaction = db.transaction('logs', 'readonly');
        const store = transaction.objectStore('logs');
        const request = store.openCursor();
        const logs: { key: IDBValidKey; log: any }[] = [];

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

export async function getAllEvents(): Promise<{ key: IDBValidKey; event: any }[]> {
    return new Promise(async (resolve, reject) => {
        const db = await getDb();
        const transaction = db.transaction('events', 'readonly');
        const store = transaction.objectStore('events');
        const request = store.openCursor();
        const events: { key: IDBValidKey; event: any }[] = [];

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
  // IndexedDB stores plain objects, so we convert the class instance back
  store.put(JSON.parse(JSON.stringify(post)));
}

export async function getAllPosts(): Promise<Post[]> {
    return new Promise(async (resolve, reject) => {
        const db = await getDb();
        const transaction = db.transaction('posts', 'readonly');
        const store = transaction.objectStore('posts');
        const request = store.getAll();

        request.onerror = () => {
            reject('Error getting posts from IndexedDB');
        };

        request.onsuccess = () => {
            const postsData = request.result;
            // Convert plain objects from DB to Post class instances
            const postInstances = postsData.map(data => new Post(data));
            resolve(postInstances);
        };
    });
}
