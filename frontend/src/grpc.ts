
import { createClient, Transport } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';

import { LogService } from './gen/log_pb';

const apiUrl = 'http://localhost:50051';

export const transport: Transport = createGrpcWebTransport({
  baseUrl: apiUrl,
});

export const LogServiceClient = createClient(LogService, transport);
