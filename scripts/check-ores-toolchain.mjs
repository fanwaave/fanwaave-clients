#!/usr/bin/env node
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const zpkg = readFileSync(".zpkg.toml", "utf8");
assert.match(zpkg, /"fanwaave\/fanwaave-pub-lib-core"\s*=\s*"\^0\.1\.0"/);
assert.doesNotMatch(zpkg, /"fanwaave\/fanwaave-lib-core"/);
assert.match(zpkg, /"oresoftware\/api-docs"\s*=\s*"=2\.0\.3"/);
assert.match(zpkg, /"oresoftware\/typespec-json-schema-validator"\s*=\s*"=0\.1\.1"/);

const toolchain = JSON.parse(readFileSync("toolchain/ores-toolchain.v1.json", "utf8"));
assert.equal(toolchain.schema, "fanwaave.ores-toolchain/v1");
assert.equal(toolchain.repository, "fanwaave/fanwaave-clients");
assert.equal(toolchain.authorities.contracts, "fanwaave/fanwaave-interfaces");
assert.equal(toolchain.authorities.publicCore, "fanwaave/fanwaave-pub-lib-core");

const tools = new Map(toolchain.tools.map((tool) => [tool.repository, tool]));
assert.equal(tools.size, toolchain.tools.length, "toolchain repository entries must be unique");
const admissions = new Set(["active", "blocked-upstream", "transitional"]);
for (const tool of toolchain.tools) {
  assert.match(tool.repository, /^[A-Za-z0-9._-]+\/[A-Za-z0-9._-]+$/, "invalid repository identity");
  assert.match(tool.commit, /^[0-9a-f]{40}$/, `${tool.repository} must be SHA pinned`);
  assert.ok(admissions.has(tool.admission), `unknown admission: ${tool.repository}`);
  if (tool.admission === "blocked-upstream") {
    assert.ok(tool.blocker?.trim(), `missing blocker: ${tool.repository}`);
  }
  if (tool.admission === "transitional") {
    assert.ok(tool.sunset?.trim(), `transitional tool lacks sunset: ${tool.repository}`);
  }
}

for (const repository of [
  "ORESoftware/api-docs",
  "ORESoftware/typespec-json-schema-validator",
  "ORESoftware/ores-wit",
  "ores-stack/ores-stack-cli",
  "ORESoftware/ores-stack",
]) {
  const tool = tools.get(repository);
  assert.ok(tool, `missing toolchain entry for ${repository}`);
  assert.match(tool.commit, /^[0-9a-f]{40}$/, `${repository} must use an exact commit`);
}
assert.equal(tools.get("ORESoftware/api-docs").admission, "active");
assert.equal(tools.get("ORESoftware/typespec-json-schema-validator").admission, "active");
assert.equal(tools.get("ORESoftware/ores-wit").admission, "blocked-upstream");
assert.match(tools.get("ORESoftware/ores-wit").blocker, /Cargo\.lock/);
assert.equal(tools.get("ores-stack/ores-stack-cli").admission, "blocked-upstream");
assert.equal(tools.get("ORESoftware/ores-stack").admission, "transitional");

const projection = JSON.parse(readFileSync("rpc/public-projection.v1.json", "utf8"));
assert.equal(projection.schema, "fanwaave.rpc-public-projection/v1");
assert.equal(projection.sourceRepository, "fanwaave/fanwaave-api-server.rs");
assert.equal(projection.contractAuthority, "fanwaave/fanwaave-interfaces");
assert.equal(projection.audience, "public");
assert.equal(projection.scope, "regular");
assert.equal(projection.rules.allowAdminScope, false);
assert.equal(projection.rules.allowAudienceWidening, false);
assert.equal(projection.rules.allowHandAuthoredGeneratedSources, false);
assert.ok(projection.operations.length > 0, "at least one public operation must be declared");
assert.equal(
  new Set(projection.operations.map((operation) => operation.key)).size,
  projection.operations.length,
  "public operation keys must be unique",
);
for (const operation of projection.operations) {
  assert.match(operation.key, /^[a-z0-9]+(?:[._-][a-z0-9]+)+$/);
  assert.equal(operation.requiredAudience, "browser");
  assert.equal(operation.requiredScope, "regular");
}

console.log("Fanwaave ORES toolchain/public RPC policy is internally consistent");
