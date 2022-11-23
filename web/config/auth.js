import axios from 'axios';

function analysisUrl() {
    !Boolean(process.env.NODE_OPEN) && getAuthOrigin().then(res => {
        const open = res.data.data;
        const href = `//${open}/oauth2/auth?client_id=baihai-inner&scope=openid&response_type=code&state=${new Date().getTime()}&prompt=login&redirect_uri=${window.location.origin}/`;
        console.log('@认证中心地址:', href);
        window.location.href = href;
    });
}

async function getAuthOrigin() {
    return await axios.get('/0/api/v1/user/open/url');
}

export {
    analysisUrl
}