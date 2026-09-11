import {
  ClientError,
  err,
  ok,
  type Result,
} from "./errors";

export interface ClientConfig {
  readonly baseUrl: string;
  readonly bearerToken?: string;
  readonly maxResponseBytes: number;
}

export type ConfigResult = Result<Readonly<ClientConfig>, "invalid_base">;

function ownedConfig(input: ClientConfig): Readonly<ClientConfig> {
  return Object.freeze({
    baseUrl: input.baseUrl,
    bearerToken: input.bearerToken,
    maxResponseBytes: input.maxResponseBytes,
  });
}

/**
 * Build an owned, frozen configuration snapshot without throwing.
 *
 * The client never retains the caller's object reference. This matters even
 * though today's fields are primitives: future nested configuration must be
 * reconstructed here rather than shallow-spread if it can contain mutable
 * arrays, maps, sets, or objects.
 */
export function tryCreateClientConfig(input: ClientConfig): ConfigResult {
  if (!input.baseUrl.trim()) {
    return err("invalid_base");
  }
  return ok(ownedConfig(input));
}

/** Compatibility wrapper for existing callers that use exception semantics. */
export function createClientConfig(input: ClientConfig): Readonly<ClientConfig> {
  const result = tryCreateClientConfig(input);
  if (!result.ok) {
    throw new ClientError(result.error);
  }
  return result.value;
}

export function tryConfigFromEnv(
  env: Readonly<Record<string, string | undefined>> = process.env,
): ConfigResult {
  const baseUrl = env["FANWAAVE_API_BASE"]?.trim();
  if (!baseUrl) {
    return err("invalid_base");
  }
  return tryCreateClientConfig({
    baseUrl,
    bearerToken: env["FANWAAVE_TOKEN"] || undefined,
    maxResponseBytes: 64 * 1024,
  });
}

/** Compatibility wrapper for existing callers that use exception semantics. */
export function configFromEnv(
  env: Readonly<Record<string, string | undefined>> = process.env,
): Readonly<ClientConfig> {
  const result = tryConfigFromEnv(env);
  if (!result.ok) {
    throw new ClientError(result.error);
  }
  return result.value;
}
