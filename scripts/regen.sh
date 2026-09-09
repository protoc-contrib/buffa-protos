#!/usr/bin/env bash
#
# Regenerate src/generated from the vendored protos.
#
# Worth a script rather than a bare `buf generate`: the output is committed, so
# a package renamed or removed upstream would leave a stale file behind that
# still compiles. Removing the directory first makes the tree say what the
# protos say.
#
# With --check, fails if regenerating changes anything -- which means either the
# committed output is stale or the pinned plugin moved.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

check=false
if [[ "${1:-}" == "--check" ]]; then
  check=true
elif [[ $# -gt 0 ]]; then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

rm -rf src/generated
buf generate

if $check; then
  if ! git diff --quiet -- src/generated; then
    echo "regen: --check failed, regenerating changed committed output:" >&2
    git --no-pager diff --stat -- src/generated >&2
    exit 1
  fi
  echo "regen: --check passed, output byte-identical"
else
  echo "regen: done"
fi
