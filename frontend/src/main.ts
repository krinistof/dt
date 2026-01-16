import { createClient } from "@connectrpc/connect";
import { createConnectTransport } from "@connectrpc/connect-web";
import { LogCollectorService } from "./gen/log/v1/log_pb.js";

const transport = createConnectTransport({
  baseUrl: "/",
});

const client = createClient(LogCollectorService, transport);

async function sendLog(msg: string) {
  const output = document.getElementById("output")!;
  try {
    // Note: No need for new LogRequest(), just pass the object literal
    const response = await client.log({ message: msg });
    output.innerText += `\n[Success] Logged: ${msg}`;
    console.log("RPC Response:", response);
  } catch (err) {
    output.innerText += `\n[Error] ${err}`;
  }
}

document.getElementById("call-btn")?.addEventListener("click", () => {
  const input = (document.getElementById("log-input") as HTMLInputElement).value;
  sendLog(input || "Default cynic message");
});
