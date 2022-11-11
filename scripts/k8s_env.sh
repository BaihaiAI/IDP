#!/bin/bash
set -exu
remote=ucloud
namespace=idp #kubectl config set-context --current --namespace=$namespace
region=a
team_id=executor
#node_name=ray-idp-raycluster-a-executor-head
pod=$(ssh ucloud "kubectl -n $namespace get pod -l app=idp-develop-$region-$team_id -o custom-columns=:metadata.name --no-headers")
if [ -z "${pod}" ]; then
    echo "pod not found" && exit 1
fi
target=${1:-glibc}