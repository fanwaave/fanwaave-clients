export type ClientErrorCode =
  | "invalid_base"
  | "empty_token"
  | "http"
  | "too_large"
  | "invalid_json";

export type Result<T, E = ClientErrorCode> =
  | Readonly<{ ok: true; value: T }>
  | Readonly<{ ok: false; error: E }>;

export function ok<T>(value: T): Result<T, never> {
  return Object.freeze({ ok: true as const, value });
}

export function err<E>(error: E): Result<never, E> {
  return Object.freeze({ ok: false as const, error });
}

export class ClientError extends Error {
  constructor(readonly code: ClientErrorCode) {
    super(code);
    this.name = "ClientError";
  }
}
