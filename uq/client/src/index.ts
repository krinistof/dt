import { createClient, type PromiseClient } from "@connectrpc/connect";
import { createConnectTransport } from "@connectrpc/connect-web";
import { UqService } from "./gen/uq/v1/uq_connect.js";
import { Event, PullRequest, PushRequest } from "./gen/uq/v1/uq_pb.js";
import * as ed from "@noble/ed25519";
import { sha512 } from "@noble/hashes/sha512";
import { sha256 } from "@noble/hashes/sha256";

// Fix for noble-ed25519 usage with syncsha256
ed.etc.sha512Sync = (...m) => sha512(ed.etc.concatBytes(...m));

export type UqClient = PromiseClient<typeof UqService>;

export interface KeyPair {
  publicKey: Uint8Array;
  privateKey: Uint8Array;
}

export async function generateKeyPair(): Promise<KeyPair> {
  const privateKey = ed.utils.randomPrivateKey();
  const publicKey = await ed.getPublicKeyAsync(privateKey);
  return { publicKey, privateKey };
}

export function createUqClient(baseUrl: string): UqClient {
  const transport = createConnectTransport({
    baseUrl,
  });
  return createClient(UqService, transport);
}

export async function pushEvent(
  client: UqClient,
  author: KeyPair,
  topicPk: Uint8Array,
  payload: Uint8Array
): Promise<void> {
  // Construct signature
  // msg = sha256(payload) + topicPk
  const payloadHash = sha256(payload);
  const msg = new Uint8Array(payloadHash.length + topicPk.length);
  msg.set(payloadHash);
  msg.set(topicPk, payloadHash.length);

  const signature = await ed.signAsync(msg, author.privateKey);

  const event = new Event({
    topicPk: topicPk as any,
    authorPk: author.publicKey as any,
    signature: signature as any,
    payload: payload as any,
    // serverTimestampMs is set by server
  });

  const request = new PushRequest({
    events: [event],
  });

  await client.push(request);
}

export async function pullEvents(
  client: UqClient,
  sinceMs: bigint,
  filterTopics: Uint8Array[] = []
): Promise<Event[]> {
  const request = new PullRequest({
    sinceTimestampMs: sinceMs,
    filterTopics,
  });
  
  const response = await client.pull(request);
  return response.events;
}

// Re-export proto types
export * from "./gen/uq/v1/uq_pb.js";
