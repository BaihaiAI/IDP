import { Layout, Menu } from "antd";
import { useEffect, useState } from "react";

import { useHistory, useLocation } from "react-router";
import navConfig from "@/pages/common/navConfig";
import globalData from "idp/global"
import { toJS } from "mobx"
import { observer } from "mobx-react";
import { isTraveler } from "@/store/cookie";
import UserInfoGlobal from 'idp/global/userinfo';

const { Sider } = Layout

export const travelRoutes = ["modelwarehouse"]

function LeftNavMenu() {

    const location = useLocation();
    const history = useHistory();

    const gotoPage = (newPath) => {
        let searchParams = new URLSearchParams(location.search);
        searchParams.delete('tensor');
        searchParams.delete('loadPage');
        searchParams.delete('drawer');
        history.push('/' + newPath + '?' + searchParams.toString());
    }

    let currentRoutes = navConfig.concat(toJS(globalData.routerMenuControl.currentRoutes));
    const userInfo = toJS(UserInfoGlobal.userInfo);
    if (userInfo?.navType == 'AIGC') {
        Object.values(currentRoutes).forEach(it => {
            if (it.key === 'workspace') {
                it.flg = false;
                it.component = <></>
            }
        })
    }
    if (isTraveler()) {
        currentRoutes = currentRoutes.filter(item => travelRoutes.includes(item.key))
    }

    const [selectedKeys, setSelectedKeys] = useState(location.pathname);

    useEffect(() => {
        const selectKey = location.pathname.split('/').filter(it => it != '');
        if (selectKey.length > 0) {
            if (location.pathname.startsWith('/modelwarehouse/model_AIGC_Detail')) {
                setSelectedKeys("/modelwarehouse/model_AIGC_Detail")
            } else {
                setSelectedKeys(`/${selectKey[0]}`)
            }
        }
    }, [location.pathname]);

    useEffect(() => {
        // @ts-ignore
        let pages = require.context("../../idp/plugins", true, /\/.*config\.json$/);
        const user = toJS(UserInfoGlobal.userInfo);
        pages.keys().map((key, index, arr) => {
            let config = pages(key);
            if (user?.navType == 'AIGC' && config?.fileName == 'develop') {
                // TODO
            } else {
                if (Object.prototype.toString.call(config) === '[object Array]') {
                    config.forEach(({ fileName, entry, enable = true }) => {
                        if (enable) {
                            require("@/idp/plugins/" + fileName + '/' + entry);
                        }
                    });
                } else {
                    const newConfig = Object.assign({ enable: true }, { ...config });
                    newConfig.enable && require("@/idp/plugins/" + newConfig.fileName + '/' + newConfig.entry);
                }
            }
        });
    }, [])

    return (
        <Sider id={'tour-side'} width={50} collapsed={true} collapsedWidth={50} trigger={null} style={{ paddingLeft: '4px' }}>
            <Menu id='tour-left-menu' theme="dark" mode="inline" selectedKeys={[selectedKeys]} >
                {
                    currentRoutes.map((menu) => {
                        let clzObj = { paddingLeft: "5px", paddingRight: "10px", paddingTop: "4px", margin: '10px 0' }; // 默认的样式
                        menu.menuClassName && Object.assign(clzObj, { ...menu.menuClassName });
                        !menu.flg && Object.assign(clzObj, { display: 'none' });
                        return (
                            <Menu.Item className={menu.key} onClick={() => gotoPage(menu.key)} key={'/' + menu.key} name={menu.key} icon={menu.iconUnChecked} style={clzObj} >
                                {Object.prototype.toString.call(menu.name) === '[string, Function]' ? menu.name() : menu.name}
                            </Menu.Item>
                        )
                    })
                }
            </Menu>
        </Sider>
    )
}

export default observer(LeftNavMenu)
