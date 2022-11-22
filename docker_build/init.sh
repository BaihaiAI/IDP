#!/usr/bin/env bash
set -exu
script_dir=$(dirname -- $(readlink -f -- "$0"))
cd $script_dir/store/1/projects/1/notebooks/
git init