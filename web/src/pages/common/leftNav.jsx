import { Layout, Menu } from "antd";
import { useEffect } from "react";

import { useHistory, useLocation } from "react-router";
import navConfig from "@/pages/common/navConfig";
import globalData from "idp/global"
import { toJS } from "mobx"

const { Sider } = Layout

export default function LeftNavMenu() {

    const location = useLocation();
    const history = useHistory();

    const gotoPage = (newPath) => {
        let searchParams = new URLSearchParams(location.search);
        searchParams.delete('tensor')
        history.push('/' + newPath + '?' + searchParams.toString());
    }

    const currentRoutes = navConfig.concat(toJS(globalData.routerMenuControl.currentRoutes));

    return (
        <Sider width={50} collapsed={true} collapsedWidth={50} trigger={null} style={{ paddingLeft: '4px' }}>
            <Menu theme="dark" mode="inline" selectedKeys={[location.pathname]} >
                {
                    currentRoutes.map((menu) => {
                        let clzObj = { paddingLeft: "5px", paddingRight: "10px", paddingTop: "4px", margin: '10px 0' }; // 默认的样式
                        menu.menuClassName && Object.assign(clzObj, { ...menu.menuClassName });
                        !menu.flg && Object.assign(clzObj, { display: 'none' });
                        return (
                            <Menu.Item onClick={() => gotoPage(menu.key)} key={'/' + menu.key} name={menu.key} icon={menu.iconUnChecked} style={clzObj} >
                                {Object.prototype.toString.call(menu.name) === '[string, Function]' ? menu.name() : menu.name}
                            </Menu.Item>
                        )
                    })
                }
            </Menu>
        </Sider>
    )
}
