# reference: [rustc builder dockerfile](https://github.com/rust-lang/rust/blob/master/src/ci/docker/host-x86_64/dist-x86_64-linux/Dockerfile)
# builder_base image used for develop or build release binary or CI
FROM centos:7 AS builder_base
# centos GNU g++ version is too old not support C++14 so we use clang's g++
#yum groupinstall -y 'Development Tools' && \
RUN --mount=type=cache,target=/var/cache/yum \
    --mount=type=cache,target=/var/cache/dnf \
    yum install -y epel-release centos-release-scl && \
    yum install -y \
        llvm-toolset-7.0-lld \
        devtoolset-7-llvm \
        openssl-devel \
        make \
        strace \
        git \
        curl \
        # miniconda3 install: tar (child): bzip2: Cannot exec: No such file or directory
        bzip2 \
        vim \
        which \
        openssh-server
#yum clean all
#RUN ssh-keygen -t rsa -N "" -f ~/.ssh/id_rsa
RUN set -x && \
    NODEJS_VERSION=v16.17.0 && \
    curl -O -L https://registry.npmmirror.com/-/binary/node/latest-v16.x/node-$NODEJS_VERSION-linux-x64.tar.gz && \
    tar zxf node-$NODEJS_VERSION-linux-x64.tar.gz && \
    rm node-$NODEJS_VERSION-linux-x64.tar.gz && \
    mv /node-$NODEJS_VERSION-linux-x64/ /nodejs && \
    PATH=$PATH:/nodejs/bin && \
    npm install -g yarn && \
    curl -O -L https://registry.npmmirror.com/-/binary/node/latest-v16.x/node-$NODEJS_VERSION-headers.tar.gz && \
    # terminal(node-pty) build require nodejs-headers, we need to keep header source files
    npm config set tarball /node-$NODEJS_VERSION-headers.tar.gz
    ##17 19.99 npm ERR! gyp ERR! stack Error: ENOENT: no such file or directory, stat 'node-v16.17.0-headers.tar.gz'
    #rm node-$NODEJS_VERSION-headers.tar.gz
ENV PATH $PATH:/nodejs/bin

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn" \
    RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
RUN curl -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- -y \
        --no-modify-path \
        --profile minimal \
        --default-toolchain nightly-2022-08-11
COPY docker_build/centos7_cargo_config.toml /root/.cargo/config.toml

RUN set -x && \
    curl -L https://mirrors.tuna.tsinghua.edu.cn/anaconda/miniconda/Miniconda3-latest-Linux-x86_64.sh -o /tmp/miniconda.sh && \
    bash /tmp/miniconda.sh -bfp /root/miniconda3/ && \
    rm -rf /tmp/miniconda.sh && \
    source /root/miniconda3/bin/activate base && \
    conda create --name py38 python=3.8 -y && \
    conda create --name py39 python=3.9 -y && \
    conda create --name py310 python=3.10

RUN echo "source ~/.cargo/env" >> /root/.bashrc && \
    # enable lld on PATH
    echo "source scl_source enable llvm-toolset-7.0" >> /root/.bashrc && \
    # enable clang on PATH
    echo "source scl_source enable llvm-toolset-7" >> /root/.bashrc && \
    # enable g++/gcc from LLVM on PATH
    echo "source scl_source enable devtoolset-7" >> /root/.bashrc && \
    echo "source /root/miniconda3/bin/activate base" >> /root/.bashrc 

# docker build -t centos7 -f Dockerfile --target builder_base .
# docker run --restart=always --name centos7 -dit centos7 bash
FROM builder_base AS builder
#ENV BASH_ENV "/root/.bashrc"
#SHELL ["/bin/bash", "--login", "-c"]
WORKDIR idp-note

COPY web ./web
# build terminal
RUN --mount=type=cache,target=/root/.npm \
    set -x && \
    source /root/.bashrc && \
    cd web/terminal && \
    # npm ci
    rm -rf node_modules && \
    rm -rf package-lock.json && \
    npm config set registry https://registry.npm.taobao.org/ && \
    npm install
# build web
RUN --mount=type=cache,target=/root/.npm \
    set -x && \
    source /root/.bashrc && \
    cd web && \
    npm config set registry https://registry.npm.taobao.org/ && \
    yarn install && \
    yarn build:open

COPY rust-toolchain.toml .
COPY Cargo.toml .
COPY crates ./crates
COPY docker_build ./docker_build
# build idp
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=target \
    set -x && \
    source /root/.bashrc && \
    # because docker buildkit mount cache target folder would not found after build,
    # so we need to copy binary target to another folder
    mkdir dist && \
    cargo b --bin license_generator && \
    cp target/debug/license_generator dist/ && \
    cargo b --release --bin idp && \
    cp target/release/idp dist/ && \
    cp crates/license_generator/README.md dist/ && \
    conda activate py38 && cargo b --release --bin idp_kernel && cp target/release/idp_kernel dist/kernel_py38 && \
    conda activate py39 && cargo b --release --bin idp_kernel && cp target/release/idp_kernel dist/kernel_py39 && \
    conda activate py310 && cargo b --release --bin idp_kernel && cp target/release/idp_kernel dist/kernel_py310



FROM ubuntu:20.04 AS base

RUN apt-get update && apt-get install -y \
    python3-dev \
    python3-pip \
    zip \
    curl && \
    apt-get clean all
COPY docker_build/pip.conf /root/.pip/

# init /root/IDPStudio
ADD docker_build/store /root/IDPStudio/store
COPY docker_build/start.sh /root/IDPStudio/
COPY docker_build/init.sh /root/IDPStudio/
RUN set -x && \
    cd /root/IDPStudio && \
    curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz && \
    tar zxf python_packages.tgz && \
    rm python_packages.tgz && \
    curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/lsp_all.tgz && \
    tar zxf lsp_all.tgz && \
    rm lsp_all.tgz

EXPOSE 3000
ENTRYPOINT ["/root/IDPStudio/idp"]

FROM base as release
# copy files to /root/IDPStudio
COPY --from=builder /idp-note/web/dist /root/IDPStudio/web
COPY --from=builder /idp-note/web/terminal /root/IDPStudio/terminal
COPY --from=builder /idp-note/dist/idp /root/IDPStudio/
COPY --from=builder /idp-note/dist/kernel_py38 /root/IDPStudio/bin/
COPY --from=builder /idp-note/dist/kernel_py39 /root/IDPStudio/bin/
COPY --from=builder /idp-note/dist/kernel_py310 /root/IDPStudio/bin/
COPY --from=builder /idp-note/dist/license_generator /root/IDPStudio/bin/
COPY --from=builder /idp-note/dist/README.md /root/IDPStudio/

FROM base as debug
# copy files to /root/IDPStudio
ADD web/dist /root/IDPStudio/web
ADD web/terminal /root/IDPStudio/terminal
COPY target/debug/idp_kernel /root/IDPStudio/
COPY target/debug/idp /root/IDPStudio/
