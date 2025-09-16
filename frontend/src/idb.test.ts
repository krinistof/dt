import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { addLog, getAllLogs, clearLogs } from './idb';
import 'fake-indexeddb/auto';

describe('idb', () => {
  beforeAll(() => {
    // Set up a fake IndexedDB environment
  });

  afterAll(async () => {
    // Clean up the database
    const allLogs = await getAllLogs();
    const allKeys = allLogs.map(log => log.key);
    await clearLogs(allKeys);
  });

  it('should add and retrieve a log', async () => {
    const testLog = {
      level: 'info',
      message: 'This is a test log',
      timestamp: new Date().toISOString(),
    };

    await addLog(testLog);

    const logs = await getAllLogs();
    expect(logs).toHaveLength(1);
    expect(logs[0].log).toEqual(testLog);
  });
});
