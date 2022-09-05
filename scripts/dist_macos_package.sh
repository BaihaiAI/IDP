#!/bin/bash
set -exu

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

arch=${1:-arm64}
#arch=x64

dir=IDPStudio-macos-$arch-v$(date +'%y%m%d-%H%M%S')
mkdir -p $dir/{bin,lib}

cp -r docker_build/store $dir/store
cp docker_build/init.sh $dir/
cp docker_build/start.sh $dir/
# NOTE must run on conda base env to get CONDA_PREFIX without envs
for py_minor_version in 8 9 10; do
    #cp $CONDA_PREFIX/envs/py3$py_minor_version/lib/libpython3.$py_minor_version.dylib $dir/store/12345/miniconda3/envs/python38/lib/
    cp $CONDA_PREFIX/envs/py3$py_minor_version/lib/libpython3.$py_minor_version.dylib $dir/lib/
done
cargo b --bin idp
cp target/debug/idp $dir/

source $CONDA_PREFIX/bin/activate base
for env in py38 py39 py310; do
    conda activate $env
    cargo b --bin idp_kernel
    cp target/debug/idp_kernel $dir/bin/kernel_$env
done

if ! [ -f web/dist/index.html ]; then
    echo "web was not build, please yarn build in web dir"
    exit 1
fi
cp -r web/dist $dir/web
cp -r web/terminal $dir/terminal

pushd $dir
curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz
tar zxf python_packages.tgz
rm python_packages.tgz

curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/lsp_all.tgz
tar zxf lsp_all.tgz
rm lsp_all.tgz

pushd lsp
rm -rf node
NODEJS_VERSION=v16.17.0
curl -O -L https://registry.npmmirror.com/-/binary/node/latest-v16.x/node-$NODEJS_VERSION-darwin-$arch.tar.gz
tar zxf node-$NODEJS_VERSION-darwin-$arch.tar.gz
rm node-$NODEJS_VERSION-darwin-$arch.tar.gz
mv node-$NODEJS_VERSION-darwin-$arch node
popd

popd

tar zcf $dir.tar.gz $dir/
rm -rf $dir
echo "scp baihai@192.168.12.14:~/repos/idp-note/$dir.tar.gz ."
