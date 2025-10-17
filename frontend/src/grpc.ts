import { createClient, type Transport } from "@connectrpc/connect";
import { createGrpcWebTransport } from "@connectrpc/connect-web";

import { LogCollectorService } from "./gen/log/v1/log_pb";
import { Dt } from "./gen/dt/v1/dt_pb";

const logApiUrl = "/grpc/log";
const dtApiUrl = "/grpc/dt";

export const logTransport: Transport = createGrpcWebTransport({
	baseUrl: logApiUrl,
});

export const dtTransport: Transport = createGrpcWebTransport({
	baseUrl: dtApiUrl,
});

export const LogServiceClient = createClient(LogCollectorService, logTransport);
export const DtServiceClient = createClient(Dt, dtTransport);
