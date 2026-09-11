import { ClientError } from "./errors";

export interface ClientConfig {
  readonly baseUrl: string;
  readonly bearerToken?: string;
  readonly maxResponseBytes: number;
}

/**
 * Build an owned, frozen configuration snapshot.
 *
 * The client never retains the caller's object reference. This matters even
 * though today's fields are primitives: future nested configuration must be
 * reconstructed here rather than shallow-spread if it can contain mutable
 * arrays, maps, sets, or objects.
 */
export function createClientConfig(input: ClientConfig): Readonly<ClientConfig> {
  if (!input.baseUrl.trim()) {
    throw new ClientError("invalid_base");
  }

  return Object.freeze({
    baseUrl: input.baseUrl,
    bearerToken: input.bearerToken,
    maxResponseBytes: input.maxResponseBytes,
  });
}

export function configFromEnv(
  env: Readonly<Record<string, string | undefined>> = process.env,
): Readonly<ClientConfig> {
  const baseUrl = env["FANWAAVE_API_BASE"]?.trim();
  if (!baseUrl) {
    throw new ClientError("invalid_base");
  }
  return createClientConfig({
    baseUrl,
    bearerToken: env["FANWAAVE_TOKEN"] || undefined,
    maxResponseBytes: 64 * 1024,
  });
}
