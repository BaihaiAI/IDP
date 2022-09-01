#!/bin/bash
set -exu

script_dir=$(dirname -- $(readlink -f -- "$0"))
# get parent dir of ./scripts/ci.sh
repo_root=$(dirname $script_dir)
cd $repo_root

## typo/fmt checking
typos
cargo fmt --check
taplo fmt --check
$HOME/go/bin/license-eye header check
#cargo install --path ./crates/ci/cargo_toml_license_check
cargo_toml_license_check
if grep --perl-regexp '[\p{Han}]' --include "*.rs" --line-number ./crates -r; then
    echo "error: found chinese in rs source file break our code style" 1>&2
    exit 1
fi

## rust code checking
export RUSTFLAGS="-D warnings"
# cargo c --tests maybe duplicate to cargo clippy --tests
#cargo c --tests
cargo clippy --tests --workspace --all-targets --all-features
cargo test

cargo b --bin idp --bin idp_kernel
mkdir -p web/{terminal,dist}

commit_sha=$(git rev-parse --short HEAD)
#commit_sha=$CI_COMMIT_SHA
tag="$commit_sha-$(date +'%Y-%m-%d_%T')"
img=idp-note:$commit_sha
container=idp-note-$commit_sha

docker build -f Dockerfile --target debug -t $img .
echo $img
export GATEWAY_PORT=$(python3 -c "s=__import__('socket').socket();s.bind(('',0));print(s.getsockname()[1]);s.close()")
docker run --rm -dit --name $container -p $GATEWAY_PORT:3000 $img
INTEGRATION_TEST=1 cargo test
docker stop $container
