import assert from "node:assert/strict";
import test from "node:test";

import { Client } from "../src/client.ts";
import { createClientConfig } from "../src/config.ts";

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
