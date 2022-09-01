set -xue

git submodule update --init --recursive
pushd web
git submodule update --init --recursive
git checkout dev
cd extension
git checkout open
cd ..
# In mac M1 if nodejs version > 16, then require CXXFLAGS set to C++ 17
yarn install && CXXFLAGS="--std=c++17" yarn install:terminal && yarn build:open
popd

cargo b --release
