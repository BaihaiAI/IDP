import { message } from 'antd';
import { Menu } from './type/menu';
import { Nodes } from '../common';
import MenuImpl from './impl/menuImpl';
/**
 * hader头部api注册方法
 */
export namespace IdpMenus {
    /**
      * 注册工具
      * @param name 
      * @param data 
      */
    export function registerIdpMenu(name: string, data: Menu, api?: boolean) {
        if (name.length === 0) return;
        if (Object.prototype.toString.call(data) == '[object Object]') {
            if (Nodes.someNode(name)) {
                const node = { nodeKey: name, menuType: data.menuType };
                Nodes.updateNode(node);
                Object.assign(data, node);
                api ? MenuImpl.register(name, data, data.menuType) : MenuImpl.idpRegister(data, data.menuType);
            } else {
                menuMsg(name);
            }
        }
    }
}

function menuMsg(name) {
    message.warn(`目录节点名称【${name}】已注册，请重新注册名称`);
}