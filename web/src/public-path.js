if (window.__POWERED_BY_QIANKUN__) {
    if ( process.env.NODE === 'dev') {
        __webpack_public_path__ = window.__INJECTED_PUBLIC_PATH_BY_QIANKUN__;
    } else {
        __webpack_public_path__ = `//${window.location.host}/`;
    }
}