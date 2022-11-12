@echo on

set arch=x64
set dir=IDPStudio-win-%arch%-v%DATE:~0,4%-%date:~5,2%-%date:~8,2%

mkdir %dir%\bin
mkdir %dir%\store

xcopy docker_build\store %dir%\store /s /e /y

cargo b --bin idp
copy target\debug\idp.exe %dir% /y

call activate base

for %%i in (py38 py39 py310) do (
  call conda activate %%i
  cargo b --bin idp_kernel
  copy target\debug\idp_kernel.exe %dir%\bin\kernel_%%i.exe
)

if not exist web\dist\index.html (
  echo "web was not build, please yarn build in web dir"
  exit
)

xcopy web\dist %dir%\web\ /s /e /y
xcopy web\terminal %dir%\terminal\ /s /e /y

pushd %dir%
curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz
tar zxf python_packages.tgz
del python_packages.tgz

curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/lsp_all.tgz
tar zxf lsp_all.tgz
del lsp_all.tgz

pushd lsp
set NODEJS_VERSION=v16.17.0
rmdir node /s /q
curl -O -L https://registry.npmmirror.com/-/binary/node/latest-v16.x/node-%NODEJS_VERSION%-win-%arch%.zip
tar zxf node-%NODEJS_VERSION%-win-x64-%arch%.zip
del node-%NODEJS_VERSION%-win-x64-%arch%.zip
ren node-%NODEJS_VERSION%-win-x64-%arch% node
popd
popd

tar -acf %dir%.zip %dir%
rmdir %dir% /s /q

exit