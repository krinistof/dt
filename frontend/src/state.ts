import { createUqClient, type KeyPair } from "uq-client";

export const client = createUqClient("/");

let _identity: KeyPair | undefined;

export function getIdentity(): KeyPair | undefined {
	return _identity;
}

export function setIdentity(identity: KeyPair) {
	_identity = identity;
}
