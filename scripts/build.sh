#!/usr/bin/env bash
set -exu

# readlink -f not support in macOS
#script_dir=$(dirname -- $(readlink -f -- "$0"))
#repo_root=$(dirname $script_dir)
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd .. && pwd)
cd $repo_root


pushd web
# In mac M1 if nodejs version > 16, then require CXXFLAGS set to C++ 17
yarn install && CXXFLAGS="--std=c++17" yarn install:terminal && yarn build:open
popd

#mkdir -p target/release/lsp
pushd typescript-lsp
#./build.sh
pushd packages/vscode-pyright
npm run package && mv dist pyright
popd

popd
#tar xf /tmp/pyright.tgz --directory target/release/lsp/

cargo b --release --bin idp --bin idp_kernel

pushd target/release
curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz
tar zxf python_packages.tgz
rm python_packages.tgz
popd
