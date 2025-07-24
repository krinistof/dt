const DB_NAME = 'log-cache';
const STORE_NAME = 'logs';
const DB_VERSION = 1;

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
      db.createObjectStore(STORE_NAME, { autoIncrement: true });
    };
  });
}

export async function addLog(log: any) {
  const db = await getDb();
  const transaction = db.transaction(STORE_NAME, 'readwrite');
  const store = transaction.objectStore(STORE_NAME);
  store.add(log);
}

export async function getAllLogs(): Promise<any[]> {
    return new Promise(async (resolve, reject) => {
        const db = await getDb();
        const transaction = db.transaction(STORE_NAME, 'readonly');
        const store = transaction.objectStore(STORE_NAME);
        const request = store.getAll();

        request.onerror = () => {
            reject('Error getting logs from IndexedDB');
        };

        request.onsuccess = () => {
            resolve(request.result);
        };
    });
}

export async function clearLogs() {
    const db = await getDb();
    const transaction = db.transaction(STORE_NAME, 'readwrite');
    const store = transaction.objectStore(STORE_NAME);
    store.clear();
}
