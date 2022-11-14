#!/bin/bash
set -exu
source ./scripts/k8s_env.sh

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

binary=note_storage
if [ $target == "musl" ]; then
cargo b --bin $binary --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/debug/$binary target/$binary
else
cargo b --bin $binary
cp target/debug/$binary target/$binary
fi
local_binary_md5=$(md5sum target/$binary | cut -f 1 -d " ")
remote_binary_md5=$(ssh $remote md5sum /$binary | cut -f 1 -d " ")
if [ $local_binary_md5 != $remote_binary_md5 ]; then
    scp -C target/$binary $remote:/$binary
fi

ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl stop $binary"
ssh $remote "kubectl -n $namespace cp /$binary $pod:/tmp/$binary"
ssh $remote "kubectl -n $namespace exec $pod -- sudo mv /tmp/$binary /usr/local/bin/"
ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl restart $binary"
