#!/usr/bin/env bash
set -exu

pushd web
# In mac M1 if nodejs version > 16, then require CXXFLAGS set to C++ 17
yarn install && CXXFLAGS="--std=c++17" yarn install:terminal && yarn build:open
popd

cd /opt/
curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/lsp_all.tgz
tar zxf lsp_all.tgz
rm lsp_all.tgz
cd -

cargo b --bin idp --bin idp_kernel && ./target/debug/idp
