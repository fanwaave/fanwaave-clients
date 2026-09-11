import type { ClientConfig } from "./config";
import { createClientConfig } from "./config";
import { ClientError } from "./errors";
import type { Health } from "./types";

export class Client {
  private readonly config: Readonly<ClientConfig>;

  constructor(config: ClientConfig) {
    this.config = createClientConfig(config);
  }

  healthUrl(): string {
    return `${this.config.baseUrl.replace(/\/$/, "")}/v1/health`;
  }

  decodeHealth(body: Uint8Array): Health {
    if (body.byteLength > this.config.maxResponseBytes) {
      throw new ClientError("too_large");
    }
    try {
      return JSON.parse(new TextDecoder().decode(body)) as Health;
    } catch {
      throw new ClientError("invalid_json");
    }
  }
}
