import type { ClientConfig } from "./config";
import { createClientConfig, tryCreateClientConfig } from "./config";
import {
  ClientError,
  err,
  ok,
  type Result,
} from "./errors";
import type { Health } from "./types";

export type HealthDecodeResult = Result<Health, "too_large" | "invalid_json">;
export type ClientCreateResult = Result<Client, "invalid_base">;

export class Client {
  private readonly config: Readonly<ClientConfig>;

  constructor(config: ClientConfig) {
    this.config = createClientConfig(config);
  }

  static tryCreate(config: ClientConfig): ClientCreateResult {
    const result = tryCreateClientConfig(config);
    return result.ok ? ok(new Client(result.value)) : err(result.error);
  }

  healthUrl(): string {
    return `${this.config.baseUrl.replace(/\/$/, "")}/v1/health`;
  }

  tryDecodeHealth(body: Uint8Array): HealthDecodeResult {
    if (body.byteLength > this.config.maxResponseBytes) {
      return err("too_large");
    }
    try {
      return ok(JSON.parse(new TextDecoder().decode(body)) as Health);
    } catch {
      return err("invalid_json");
    }
  }

  /** Compatibility wrapper for existing callers that use exception semantics. */
  decodeHealth(body: Uint8Array): Health {
    const result = this.tryDecodeHealth(body);
    if (!result.ok) {
      throw new ClientError(result.error);
    }
    return result.value;
  }
}
