
COPY docker_build/pip.conf /root/.pip/
RUN set -x && \
    mkdir /root/IDPStudio && \
    cd /root/IDPStudio && \
    curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz && \
    tar zxf python_packages.tgz && \
    rm python_packages.tgz && \
    curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/lsp_all.tgz && \
    tar zxf lsp_all.tgz && \
    rm lsp_all.tgz
ADD docker_build/store/ /root/IDPStudio/
# COPY docker_build/idp_note.conf /etc/supervisord.d/idp_note.ini
ENTRYPOINT ["/root/IDPStudio/idp"]
#ENTRYPOINT ["supervisord", "--nodaemon"]

FROM base AS web
ADD web/dist /root/IDPStudio/web
ADD web/terminal /root/IDPStudio/
EXPOSE 3000

FROM web AS debug
COPY target/debug/idp_kernel /root/IDPStudio/
COPY target/debug/idp /root/IDPStudio/

FROM web AS release
COPY target/release/idp_kernel /root/IDPStudio/
COPY target/release/idp /root/IDPStudio/
