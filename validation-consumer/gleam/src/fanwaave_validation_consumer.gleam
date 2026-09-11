import fanwaave_validation
import gleam/dynamic.{type Dynamic}

pub fn validate_request_meta(value: Dynamic) {
  fanwaave_validation.decode_request_meta(value)
}
