import fanwaave_validation_consumer
import gleam/dynamic
import gleeunit/should

pub fn validates_before_transport_test() {
  dynamic.properties([
    #(dynamic.string("requestId"), dynamic.string("req-1")),
    #(dynamic.string("traceId"), dynamic.string("trace-1")),
  ])
  |> fanwaave_validation_consumer.validate_request_meta
  |> should.be_ok
}
