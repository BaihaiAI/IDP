#!/bin/bash
aim=/tmp/pyright.tgz

npm ci && pushd packages/vscode-pyright && npm run package && mv dist pyright && tar czf ${aim} pyright && echo "package pyright at ${aim}" && rm -rf pyright &&  popd
