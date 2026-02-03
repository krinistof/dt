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

export interface UnsignedEvent {
	topicPk: Uint8Array;
	payload: Uint8Array;
}

export async function signEvent(
	unsignedEvent: UnsignedEvent,
	author: KeyPair,
): Promise<Event> {
	const payloadHash = sha256(unsignedEvent.payload);
	const msg = new Uint8Array(payloadHash.length + unsignedEvent.topicPk.length);
	msg.set(payloadHash);
	msg.set(unsignedEvent.topicPk, payloadHash.length);

	const signature = await ed.signAsync(msg, author.privateKey);

	return new Event({
		topicPk: unsignedEvent.topicPk,
		authorPk: author.publicKey,
		signature: signature,
		payload: unsignedEvent.payload,
	});
}

export interface PushEventParams {
	topicPk: Uint8Array;
	author: KeyPair;
	payload: Uint8Array;
}

export async function sync(
	client: UqClient,
	sinceMs: bigint,
	userPk: Uint8Array,
	push?: PushEventParams,
): Promise<SyncResponse> {
	const events: Event[] = [];
	if (push) {
		const event = await signEvent(push, push.author);
		events.push(event);
	}

	const request = new SyncRequest({
		events,
		sinceTimestampMs: sinceMs,
		userPk,
	});

	return client.sync(request);
}

// Re-export proto types
export * from "./gen/uq/v1/uq_pb.js";
