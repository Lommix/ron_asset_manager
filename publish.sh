#! /bin/bash
tmp=$(mktemp -d)
cp -r derive/* "$tmp"/.
cp -r LICENSE-MIT LICENSE-APACHE README.md "$tmp"/.
cd $tmp && cargo publish
rm -rf "$tmp"
cd
cargo publish
