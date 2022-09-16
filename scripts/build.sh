#!/usr/bin/env bash
set -exu

script_dir=$(dirname -- $(readlink -f -- "$0"))
# get parent dir of ./scripts/ci.sh
repo_root=$(dirname $script_dir)
cd $repo_root

mkdir -p target/release/lsp

pushd web
# In mac M1 if nodejs version > 16, then require CXXFLAGS set to C++ 17
yarn install && CXXFLAGS="--std=c++17" yarn install:terminal && yarn build:open
popd

pushd typescript-lsp
./build.sh
popd
tar xf /tmp/pyright.tgz --directory target/release/lsp/

cargo b --release --bin idp --bin idp_kernel
