#! /bin/bash
set -euo pipefail

repo="$PWD"
tmp=$(mktemp -d)

cp -r derive/* "$tmp"/.
cp -r LICENSE-MIT LICENSE-APACHE README.md "$tmp"/.

cd "$tmp"
cargo publish

cd "$repo"
rm -rf "$tmp"
cargo publish
