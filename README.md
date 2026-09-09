# buffa-protos

Third-party protos compiled for the [buffa] Rust protobuf runtime.

[`buffa-types`] carries the protobuf **well-known types** — the ones that ship
with protoc. This carries the other ones a service ends up needing. Today that
is `google.rpc`.

[buffa]: https://github.com/anthropics/buffa
[`buffa-types`]: https://docs.rs/buffa-types

## What is here

| Package | Holds |
| --- | --- |
| `google.rpc` | `Status`, the `Code` enum, and the `error_details.proto` payloads that go in `Status.details` — `BadRequest`, `RetryInfo`, `ErrorInfo`, `QuotaFailure`, `PreconditionFailure`, `ResourceInfo`, `RequestInfo`, `Help`, `LocalizedMessage`, `DebugInfo` |

Vendored from [googleapis], generated with the pinned plugin in
`buf.gen.yaml`, and **committed** — so a consumer needs neither `buf` nor
`protoc` to build.

[googleapis]: https://github.com/googleapis/googleapis

## Why not generate it per service

Because `google.rpc.Status` is a *shared* type, and generating it into each
service makes it a different Rust type in each one. A `Status` built by a
library cannot then be assigned to a `Status` field on a message generated
somewhere else, and the compiler is right to refuse — they are unrelated types
that happen to share a name.

That is the same reason `google.protobuf` lives in `buffa-types` rather than
being emitted into every crate that mentions a `Timestamp`.

It is also 8,000-odd lines a service does not have to carry or regenerate.

## Using it

Point the buffa codegen at this crate instead of generating the package again:

```yaml
- remote: buf.build/anthropics/buffa:v0.9.1
  out: src/proto
  include_imports: true
  opt:
    - file_per_package=true
    # Do not generate the package...
    - exclude_package=.google.rpc
    # ...name it here instead.
    - extern_path=.google.rpc=::buffa_protos::google::rpc
```

```toml
buffa-protos = { git = "https://github.com/protoc-contrib/buffa-protos", rev = "..." }
```

Pin a `rev`. This is a git dependency with no published releases, so a branch
would mean the crate can change under a build that did not ask it to.

The module path mirrors the proto package — `google.rpc.Status` is
`buffa_protos::google::rpc::Status` — which is what lets the `extern_path` be a
single package-prefix mapping rather than one entry per type.

## Adding a package

1. Vendor the `.proto` under `proto/`.
2. Run `scripts/regen.sh`.
3. Mount the emitted `<package>.rs` in `src/lib.rs`, and add it to the `ignore`
   list in `rustfmt.toml`.

Anything the new package imports from `google.protobuf` resolves to
`buffa-types` with no further configuration — that is the `extern_path` already
in `buf.gen.yaml`, and it is what stops this crate shipping a second `Any`.
Anything it imports from a package this crate does not carry has to be vendored
alongside it.

`scripts/regen.sh --check` fails if regenerating changes anything, which means
either the committed output is stale or the pinned plugin moved. CI runs it.

## Development

```bash
nix develop        # cargo, rustc, clippy, rustfmt, buf
cargo test
./scripts/regen.sh --check
```

`buf` is for regenerating only. The generated code is committed, so building
this crate — or anything depending on it — needs neither `buf` nor `protoc`.

## License

The vendored protos are Google's, Apache-2.0. This repository is Apache-2.0;
see [LICENSE](LICENSE).
