
import { createClient, Transport } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';

import { Greeter } from './gen/greeter_pb';

const apiUrl = 'http://localhost:50051';

export const transport: Transport = createGrpcWebTransport({
  baseUrl: apiUrl,
});

export const GreeterClient = createClient(Greeter, transport);
