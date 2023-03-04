import axios from 'axios';
import { logout } from '../src/utils/logout';
import { message } from 'antd';

function analysisUrl() {
    !Boolean(process.env.NODE_OPEN) && getAuthOrigin().then(res => {
        const open = res.data.data;
        const href = `//${open}/oauth2/auth?client_id=baihai-inner&scope=openid&response_type=code&state=${new Date().getTime()}&prompt=login&redirect_uri=${window.location.origin}/`;
        console.log('@认证中心地址:', href);
        window.location.href = href;
    }).catch((e)=> {
        const { status, data} = e.response;
        if ( status == '401' && data == 'Jwt verification fails') {
            // logout();
        }
    })
}

async function getAuthOrigin() {
    try {
        logout(false); // 不执行跳转逻辑
        const result = await axios.get('/0/api/v1/user/open/url');
        return result;
    } catch ({ response }) {
        // 当open接口返回401并且值为jwt时，重新清楚缓存和cookies。
        if (response.status == '401' && response.data == 'Jwt verification fails') {
            message.info('登录信息已过期，请重新登录')
            setTimeout(() => {
                logout();
            });
        }
    }

}

export {
    analysisUrl
}