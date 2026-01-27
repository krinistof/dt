import { 
  createUqClient, 
  generateKeyPair, 
  sync, 
  type KeyPair,
  type Event
} from "uq-client";
import * as ed from "@noble/ed25519";
import { sha256 } from "@noble/hashes/sha2.js";

const client = createUqClient("/");
let identity: KeyPair;
let currentTopic: Uint8Array;

// Helpers
const toHex = (b: Uint8Array) => ed.etc.bytesToHex(b);
const fromHex = (s: string) => ed.etc.hexToBytes(s);
const enc = new TextEncoder();
const dec = new TextDecoder();

// State
async function initIdentity() {
  const stored = localStorage.getItem("uq-identity");
  if (stored) {
    const parsed = JSON.parse(stored);
    identity = {
      publicKey: fromHex(parsed.publicKey),
      privateKey: fromHex(parsed.privateKey),
    };
  } else {
    identity = await generateKeyPair();
    localStorage.setItem("uq-identity", JSON.stringify({
      publicKey: toHex(identity.publicKey),
      privateKey: toHex(identity.privateKey),
    }));
  }
  document.getElementById("user-pk")!.innerText = toHex(identity.publicKey);
}

function setTopic(input: string) {
  try {
    if (!input) {
      // Default topic: all zeros (public channel?) or hash of "default"
      // Let's use 32 bytes of zeros for "global"
      currentTopic = new Uint8Array(32);
    } else {
      currentTopic = sha256(enc.encode(input));
    }
    console.log("Topic set:", toHex(currentTopic));
    refreshFeed();
  } catch (e) {
    alert("Error setting topic");
  }
}

function renderEvents(events: Event[]) {
  const feed = document.getElementById("feed")!;
  const filtered = events.filter(e => {
      // Compare topicPk
      if (e.topicPk.length !== currentTopic.length) return false;
      for(let i=0; i<e.topicPk.length; i++) if (e.topicPk[i] !== currentTopic[i]) return false;
      return true;
  });

  feed.innerHTML = filtered.map(e => `
    <div style="margin-bottom: 5px; padding: 5px; border-bottom: 1px solid #eee;">
      <small style="color: #666;">${toHex(e.authorPk).slice(0, 8)}...</small>: 
      <span>${dec.decode(e.payload)}</span>
    </div>
  `).join("");
}

async function sendMessage() {
  const input = document.getElementById("msg-input") as HTMLInputElement;
  const msg = input.value;
  if (!msg) return;

  const payload = enc.encode(msg);
  try {
    const response = await sync(client, 0n /* TODO Pull since 1970 for now */, { author: identity, topicPk: currentTopic, payload });
    input.value = "";
    renderEvents(response.events);
  } catch (e) {
    console.error(e);
    alert("Send failed");
  }
}

async function refreshFeed() {
  try {
    const response = await sync(client, 0n); // TODO Pull since 1970 for now
    renderEvents(response.events);
  } catch (e) {
    console.error(e);
  }
}

// Bindings
document.getElementById("regen-id-btn")?.addEventListener("click", async () => {
  localStorage.removeItem("uq-identity");
  await initIdentity();
});

document.getElementById("set-topic-btn")?.addEventListener("click", () => {
  const input = document.getElementById("topic-input") as HTMLInputElement;
  setTopic(input.value);
});

document.getElementById("send-btn")?.addEventListener("click", sendMessage);

// Init
await initIdentity();
setTopic(""); // Default topic
setInterval(refreshFeed, 2000); // Poll every 2s
