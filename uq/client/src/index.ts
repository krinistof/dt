import { createClient, type PromiseClient } from "@connectrpc/connect";
import { createConnectTransport } from "@connectrpc/connect-web";
import { UqService } from "./gen/uq/v1/uq_connect.js";
import { Event, SyncRequest, SyncResponse } from "./gen/uq/v1/uq_pb.js";
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

export interface PushEventParams {
  topicPk: Uint8Array;
  author: KeyPair;
  payload: Uint8Array;
}

export async function sync(
  client: UqClient,
  sinceMs: bigint,
  push?: PushEventParams
): Promise<SyncResponse> {
  const events: Event[] = [];
  if (push) {
    // Construct signature
    // msg = sha256(payload) + topicPk
    const payloadHash = sha256(push.payload);
    const msg = new Uint8Array(payloadHash.length + push.topicPk.length);
    msg.set(payloadHash);
    msg.set(push.topicPk, payloadHash.length);

    const signature = await ed.signAsync(msg, push.author.privateKey);

    const event = new Event({
      topicPk: push.topicPk,
      authorPk: push.author.publicKey,
      signature: signature,
      payload: push.payload,
      // serverTimestampMs is set by server
    });
    events.push(event);
  }

  const request = new SyncRequest({
    events,
    sinceTimestampMs: sinceMs,
  });

  return client.sync(request);
}

// Re-export proto types
export * from "./gen/uq/v1/uq_pb.js";
