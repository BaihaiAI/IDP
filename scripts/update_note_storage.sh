#!/bin/bash
set -exu

remote=ucloud
namespace=nightly
#binary=kernel_manage
binary=note_storage
region=b
team_id=$(ssh $remote "kubectl -n $namespace exec svc/postgres-inner -- psql -U postgres -d idp_prod_saas -c \"select team_id from user_info where username='hr@baihai.ai'\" --no-align  --tuples-only")
pod=$(ssh ucloud "kubectl -n $namespace get pod -l app=idp-develop-$region-$team_id -o custom-columns=:metadata.name --no-headers")
if [ -z "${pod}" ]; then
    echo "pod not found" && exit 1
fi
target=${1:-glibc}

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

if [ $target == "musl" ]; then
cargo b --bin $binary --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/debug/$binary target/$binary
else
cargo b --bin $binary
cp target/debug/$binary target/$binary
fi
local_binary_md5=$(md5sum target/$binary | cut -f 1 -d " ")
remote_binary_md5=$(ssh $remote md5sum /$binary | cut -f 1 -d " ")
if [ "$local_binary_md5" != "$remote_binary_md5" ]; then
    scp -C target/$binary $remote:/$binary
fi

ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl stop $binary"
ssh $remote "kubectl -n $namespace cp /$binary $pod:/tmp/$binary"
ssh $remote "kubectl -n $namespace exec $pod -- sudo mv /tmp/$binary /usr/local/bin/"
ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl restart $binary"
