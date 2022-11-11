#!/bin/bash
set -exu
source ./scripts/k8s_env.sh

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

binary=note_storage
if [ $target == "musl" ]; then
cargo b --bin $binary --target x86_64-unknown-linux-musl
scp -C target/x86_64-unknown-linux-musl/debug/$binary $remote:/root/
else
cargo b --bin $binary
scp -C target/debug/$binary $remote:/root/
fi

ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl stop $binary"
ssh $remote "kubectl -n $namespace cp /root/$binary $pod:/tmp/$binary"
ssh $remote "kubectl -n $namespace exec $pod -- sudo mv /tmp/$binary /usr/local/bin/"
ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl restart $binary"
