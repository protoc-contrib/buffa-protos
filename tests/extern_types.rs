//! The property this crate exists for: the `google.protobuf` types its protos
//! import are *the* buffa-types ones, not a second copy generated here.
//!
//! If the `extern_path` in buf.gen.yaml were dropped, everything below would
//! still compile against a locally generated `Any` — and then fail at the one
//! place it matters, when a consumer hands over a value built from
//! `buffa_types`. So these assign across the boundary rather than merely
//! constructing.

use buffa_protos::google::rpc::{BadRequest, Code, RetryInfo, Status, bad_request};
use buffa_types::google::protobuf::{Any, Duration};

#[test]
fn status_details_takes_a_buffa_types_any() {
    let detail = Any {
        type_url: "type.googleapis.com/google.rpc.RetryInfo".to_owned(),
        ..Default::default()
    };

    // The assignment is the test: `details` is `Vec<Any>` where `Any` is
    // buffa-types', so a locally generated one would not type-check here.
    let status = Status {
        code: Code::UNAVAILABLE as i32,
        message: "try again".to_owned(),
        details: vec![detail],
        ..Default::default()
    };

    assert_eq!(status.details.len(), 1);
}

#[test]
fn retry_info_takes_a_buffa_types_duration() {
    let retry = RetryInfo {
        retry_delay: buffa::MessageField::some(Duration {
            seconds: 5,
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_eq!(retry.retry_delay.as_option().map(|d| d.seconds), Some(5));
}

#[test]
fn the_error_details_payloads_are_all_here() {
    // `Status.details` is only useful if what goes in it exists, so the whole
    // package is generated rather than status.proto alone.
    let violation = bad_request::FieldViolation {
        field: "update_mask.paths[1]".to_owned(),
        description: "not a permitted field path".to_owned(),
        ..Default::default()
    };
    let bad = BadRequest {
        field_violations: vec![violation],
        ..Default::default()
    };

    assert_eq!(bad.field_violations[0].field, "update_mask.paths[1]");
}

#[test]
fn code_is_the_grpc_code_enum() {
    assert_eq!(Code::OK as i32, 0);
    assert_eq!(Code::NOT_FOUND as i32, 5);
    assert_eq!(Code::UNAVAILABLE as i32, 14);
}
