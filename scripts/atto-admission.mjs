// Admission policy only. The contract authority remains fanwaave-interfaces.
export function validate_atto_binding(binding, projection) {
  const reject = (reason) => { throw new Error(`AttoLang binding rejected: ${reason}`); };
  if (!binding || typeof binding !== "object" || Array.isArray(binding)) reject("missing binding");
  if (!projection || typeof projection !== "object") reject("missing projection");
  if (binding.schema !== "fanwaave.atto-client-binding/v1" ||
      binding.language !== "atto" || binding.admission !== "contract-only" ||
      binding.contractAuthority !== "fanwaave/fanwaave-interfaces") reject("unrecognized binding authority");
  if (binding.transportImplemented !== false) reject("unverified transport admission");
  if (binding.operation !== "fanwaave.health.get" ||
      binding.audience !== "browser" || binding.scope !== "regular") reject("non-public operation or scope");
  if (!binding.security || binding.security.httpsOnly !== true ||
      binding.security.bearerHeader !== true || binding.security.boundedResponse !== true) reject("security contract not complete");
  if (projection.schema !== "fanwaave.rpc-public-projection/v1" ||
      projection.contractAuthority !== binding.contractAuthority ||
      projection.audience !== "public" || projection.scope !== "regular" ||
      projection.rules?.allowAdminScope !== false ||
      projection.rules?.allowAudienceWidening !== false ||
      projection.rules?.allowHandAuthoredGeneratedSources !== false) reject("RPC policy mismatch");
  if (!Array.isArray(projection.operations) || !projection.operations.some((operation) =>
    operation.key === binding.operation &&
    operation.requiredAudience === binding.audience &&
    operation.requiredScope === binding.scope)) reject("operation missing from admitted public projection");
  return true;
}
