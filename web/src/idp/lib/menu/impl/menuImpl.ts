import IRegister from '@/idp/base/index';
import { Menu } from '@/idp/lib/menu/type/menu';
import { action, observable, toJS } from 'mobx';

/**
 * 头部功能实现类
 */
class MenuImpl<T> implements IRegister<T> {

    @observable idpToolMap: any = []; // 对内自定义数据
    @observable idpMenuMap: any = []; // 对内菜单数据
    @observable toolMap: any = []; // 对外自定义数据
    @observable menuMap: any = []; // 对外菜单数据

    // 外部api方法
    @action register(name: string, data: T, menuType: string) {
        this[`${menuType.toLowerCase()}Map`] = this[`${menuType.toLowerCase()}Map`].concat([data]);
    };

    // 只对外部的数据销毁
    @action destroyRegister(name: string) {
        this.menuMap = [];
        this.toolMap = [];
    };

    // 内部api方法
    @action idpRegister(data: T, menuType: string) {
        const { nodeKey }: any = data;
        // 此处暂时性做兼容性处理，插件版本已经替换了整个业务逻辑
        if (nodeKey == 'notification') {
            const _data = toJS(this[`idp${menuType}Map`]);
            _data.splice(0, 0, data);
            this[`idp${menuType}Map`] = _data;
            console.log(toJS(this[`idp${menuType}Map`]));
        } else {
            this[`idp${menuType}Map`] = this[`idp${menuType}Map`].concat([data]);
        }
    }
}

export default new MenuImpl<Menu>();