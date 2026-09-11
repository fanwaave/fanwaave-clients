import assert from "node:assert/strict";
import test from "node:test";

import { Client } from "../src/client.ts";
import {
  createClientConfig,
  tryCreateClientConfig,
} from "../src/config.ts";

test("createClientConfig returns a fresh frozen snapshot", () => {
  const input = {
    baseUrl: "https://api.example.test/",
    bearerToken: "token-a",
    maxResponseBytes: 1024,
  };

  const snapshot = createClientConfig(input);

  assert.notStrictEqual(snapshot, input);
  assert.equal(Object.isFrozen(snapshot), true);

  input.baseUrl = "https://mutated.example.test/";
  input.bearerToken = "token-b";
  input.maxResponseBytes = 1;

  assert.deepEqual(snapshot, {
    baseUrl: "https://api.example.test/",
    bearerToken: "token-a",
    maxResponseBytes: 1024,
  });
});

test("typed config construction returns an explicit immutable result", () => {
  const input = {
    baseUrl: "https://api.example.test/",
    bearerToken: "token-a",
    maxResponseBytes: 1024,
  };

  const result = tryCreateClientConfig(input);
  assert.equal(result.ok, true);
  if (!result.ok) return;

  assert.notStrictEqual(result.value, input);
  assert.equal(Object.isFrozen(result), true);
  assert.equal(Object.isFrozen(result.value), true);

  input.baseUrl = "https://mutated.example.test/";
  assert.equal(result.value.baseUrl, "https://api.example.test/");

  assert.deepEqual(
    tryCreateClientConfig({ ...input, baseUrl: "   " }),
    { ok: false, error: "invalid_base" },
  );
});

test("Client detaches behavior from later caller mutation", () => {
  const input = {
    baseUrl: "https://api.example.test/",
    maxResponseBytes: 1024,
  };
  const client = new Client(input);

  input.baseUrl = "https://mutated.example.test/";
  input.maxResponseBytes = 1;

  assert.equal(client.healthUrl(), "https://api.example.test/v1/health");
  assert.doesNotThrow(() =>
    client.decodeHealth(new TextEncoder().encode('{"status":"ok"}')),
  );
});

test("typed client APIs encode construction and decode failures as values", () => {
  const created = Client.tryCreate({
    baseUrl: "https://api.example.test/",
    maxResponseBytes: 16,
  });
  assert.equal(created.ok, true);
  if (!created.ok) return;

  const decoded = created.value.tryDecodeHealth(
    new TextEncoder().encode('{"status":"ok"}'),
  );
  assert.deepEqual(decoded, { ok: true, value: { status: "ok" } });
  assert.deepEqual(
    created.value.tryDecodeHealth(new Uint8Array(17)),
    { ok: false, error: "too_large" },
  );
  assert.deepEqual(
    created.value.tryDecodeHealth(new TextEncoder().encode("{")),
    { ok: false, error: "invalid_json" },
  );
});
