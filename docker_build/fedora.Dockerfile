#cargo b --bin idp --bin idp_kernel && docker build -t note -f docker_build/fedora.Dockerfile . && docker run --rm -it --name note -p 3003:3000 note
FROM fedora:36 AS base

RUN dnf install --setopt=install_weak_deps=False -y \
        python3-pip \
        zip && \
    dnf clean all
COPY docker_build/pip.conf /root/.pip/

# init /root/IDPStudio
ADD docker_build/store /root/IDPStudio/store
RUN set -x && \
    cd /root/IDPStudio && \
    curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz && \
    tar zxf python_packages.tgz && \
    rm python_packages.tgz && \
    curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/lsp_all.tgz && \
    tar zxf lsp_all.tgz && \
    rm lsp_all.tgz

# copy files to /root/IDPStudio
ADD web/dist /root/IDPStudio/web
ADD web/terminal /root/IDPStudio/terminal
COPY target/debug/idp_kernel /root/IDPStudio/
COPY target/debug/idp /root/IDPStudio/
# COPY docker_build/idp_note.conf /etc/supervisord.d/idp_note.ini

EXPOSE 3000
ENTRYPOINT ["/root/IDPStudio/idp"]
#ENTRYPOINT ["supervisord", "--nodaemon"]


# FROM base AS debug
# COPY target/debug/idp_kernel /root/IDPStudio/
# COPY target/debug/idp /root/IDPStudio/

# FROM base AS release
# COPY target/release/idp_kernel /root/IDPStudio/
# COPY target/release/idp /root/IDPStudio/
