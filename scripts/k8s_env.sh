#!/bin/bash
set -exu
# generate team_id.sh by go script
go run scripts/get_nightly_k8s_env_team_id.go
source ./scripts/team_id.sh

remote_hostname=ucloud
namespace=nightly
node_name=ray-idp-raycluster-b-$team_id-head
pod=$(ssh $remote_hostname "kubectl -n $namespace get pod -l ray-node-name=$node_name -o custom-columns=:metadata.name --no-headers")
if [ -z "${pod}" ]; then
    echo "pod not found"
    exit 1
fi
