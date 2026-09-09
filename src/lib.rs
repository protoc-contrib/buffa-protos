//! Third-party protos compiled for [buffa].
//!
//! [`buffa-types`] carries the protobuf *well-known* types — the ones that ship
//! with protoc. This carries the other ones a service ends up needing, which
//! today means [`google::rpc`]: `Status`, the `Code` enum, and the
//! `error_details.proto` payloads that go in `Status.details`.
//!
//! Those are googleapis, not well-known types, so they are nobody's to ship by
//! default — and generating them into each service instead means every service
//! carries its own `Status`, which is a different Rust type in each one.
//!
//! # Usage
//!
//! Point the buffa codegen at this crate rather than generating the package
//! again, which is what makes a `Status` built here assignable to a `Status`
//! field there:
//!
//! ```yaml
//! - remote: buf.build/anthropics/buffa:v0.9.1
//!   out: src/proto
//!   include_imports: true
//!   opt:
//!     - exclude_package=.google.rpc
//!     - extern_path=.google.rpc=::buffa_protos::google::rpc
//! ```
//!
//! ```
//! use buffa_protos::google::rpc::{Code, Status};
//!
//! let status = Status {
//!     //  is an i32 that "should be" a Code, per the proto -- so
//!     // the enum is a source of the right numbers, not the field's type.
//!     code: Code::NOT_FOUND as i32,
//!     message: "collections/9c7ecc05 does not exist".to_owned(),
//!     ..Default::default()
//! };
//! assert_eq!(status.code, 5);
//! ```
//!
//! # Adding a package
//!
//! Vendor the `.proto` under `proto/`, run `scripts/regen.sh`, and mount the
//! emitted `<package>.rs` below. Anything it imports from `google.protobuf`
//! resolves to `buffa-types` without further configuration; anything it imports
//! from a package this crate does not carry has to be vendored too.
//!
//! [buffa]: https://github.com/anthropics/buffa
//! [`buffa-types`]: https://docs.rs/buffa-types

/// Mirrors the proto package path, so a type is where its fully-qualified proto
/// name says it is — `google.rpc.Status` at `buffa_protos::google::rpc::Status`.
///
/// That is also what lets the `extern_path` in a consumer's `buf.gen.yaml` be a
/// single package-prefix mapping rather than one entry per type.
pub mod google {
    /// `google/rpc/status.proto`, `code.proto` and `error_details.proto`.
    pub mod rpc {
        #![allow(
            non_camel_case_types,
            dead_code,
            unused_imports,
            unused_qualifications,
            clippy::derivable_impls,
            clippy::match_single_binding,
            clippy::uninlined_format_args,
            clippy::doc_lazy_continuation,
            clippy::module_inception,
            clippy::pedantic,
            reason = "buffa codegen output -- upstream codegen style, not this crate's to police"
        )]

        include!("generated/google.rpc.rs");
    }
}
