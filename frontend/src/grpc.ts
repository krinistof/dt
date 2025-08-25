
import { createClient, Transport } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';

import { LogCollectorService } from './gen/log/v1/log_pb';

const apiUrl = '/grpc';

export const transport: Transport = createGrpcWebTransport({
  baseUrl: apiUrl,
});

export const LogServiceClient = createClient(LogCollectorService, transport);
