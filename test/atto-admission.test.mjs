import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { validate_atto_binding } from "../scripts/atto-admission.mjs";

const binding = JSON.parse(readFileSync(new URL("../clients/atto/contract-binding.v1.json", import.meta.url)));
const projection = JSON.parse(readFileSync(new URL("../rpc/public-projection.v1.json", import.meta.url)));
const policy = readFileSync(new URL("../clients/atto/health.atto", import.meta.url), "utf8");
const matrix = readFileSync(new URL("../clients/support-matrix.toml", import.meta.url), "utf8");

test("AttoLang binding preserves public health-only RPC authority", () => {
  assert.equal(validate_atto_binding(binding, projection), true);
});
test("AttoLang support is staged, not advertised as production transport", () => {
  assert.match(matrix, /\[languages\.atto\]\s*tier\s*=\s*3\s*status\s*=\s*"contract-only"/);
  assert.equal(binding.transportImplemented, false);
});
test("AttoLang module exposes only pure, bounded health-response predicates", () => {
  assert.match(policy, /pub fnc valid_health_status\(int status_code\): bool/);
  assert.match(policy, /pub fnc within_response_budget\(int payload_bytes, int max_response_bytes\): bool/);
  assert.match(policy, /pub fnc accept_health_response\(int status_code, int payload_bytes, int max_response_bytes\): bool/);
  assert.match(policy, /if payload_bytes < 0 then/);
  assert.match(policy, /if max_response_bytes < 1 then/);
  assert.doesNotMatch(policy, /\b(?:fnc main|fetch\(|http\.get\(|send\(|Authorization:)\b/);
});
test("reject admin scope and widened audience", () => {
  for (const patch of [{scope:"admin"}, {audience:"server"}, {operation:"fanwaave.admin.get"}]) {
    assert.throws(() => validate_atto_binding({...binding, ...patch}, projection), /rejected/);
  }
});
test("reject false transport or missing security guarantees", () => {
  assert.throws(() => validate_atto_binding({...binding, transportImplemented:true}, projection), /transport/);
  for (const key of ["httpsOnly","bearerHeader","boundedResponse"]) {
    assert.throws(() => validate_atto_binding({...binding, security:{...binding.security,[key]:false}}, projection), /security/);
  }
});
test("reject forged or absent public operation", () => {
  assert.throws(() => validate_atto_binding(binding,{...projection,operations:[]}), /operation/);
  assert.throws(() => validate_atto_binding(binding,{...projection,rules:{...projection.rules,allowAdminScope:true}}), /RPC policy/);
  assert.throws(() => validate_atto_binding(binding,{...projection,operations:[{key:binding.operation,requiredAudience:"server",requiredScope:"regular"}]}), /operation/);
});
