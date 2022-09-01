import { IdpMenus } from './src/idp/lib/menu';
import globalData from "./src/idp/global";

function loadModule(type: 'menu' | 'tool' | 'router' | 'footerBar' | 'rightSider') {
    switch (type) {
        case 'menu':
            return IdpMenus
        case 'rightSider':
            return globalData
        case 'router':
            return globalData
        case 'footerBar':
            return globalData
        case 'tool':
            return IdpMenus
    }
}
export {
    loadModule
};