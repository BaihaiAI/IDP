import { Col, Space, Layout, Menu, Row, Dropdown } from "antd";
import { observer } from "mobx-react";
import { Fragment, useEffect, useMemo } from "react"
import { useDispatch, useSelector } from "react-redux";
import { changeOperatorDecision, selectOperatorDecision } from "@/store/features/globalSlice";
import { toJS } from 'mobx';
import ShortcutList from "@/components/globalpopup/ShortcutList";
import globalData from "@/idp/global";
import loadPlugins from '@/idp/global/plugins/load';

import './index.less';

const { SubMenu } = Menu;

type Props = {};

let curLogin = '';
let time = null;

const IdpHeader: React.FC<Props> = () => {

    const dispatch = useDispatch();

    const vis = useSelector(selectOperatorDecision);

    const loadToolPlugs = useMemo(() => {
        const data = toJS(globalData.headerTool.headerToolList);
        const r = data?.map((item, index) => {
            return <div key={`tool_${index}`} style={{ display: 'flex', cursor: "pointer", alignItems: 'center' }}>
                {item.component}
            </div>
        });
        return r;
    }, [toJS(globalData.headerTool.headerToolList)]);

    const loadMenuPlugs = useMemo(() => {
        const data = toJS(globalData.headerMenu.headerMeunList);
        return <>
            {
                data.map((item, index) => {
                    return (
                        <div key={index}>
                            {item.component}
                        </div>
                    )
                })
            }
        </>;
    }, [toJS(globalData.headerMenu.headerMeunList)]);

    const loadPluginsPage = useMemo(() => {
        return (
            <div style={{ marginLeft: '15px', color: '#fff', opacity: (loadPlugins.currentLoadPluginSize != loadPlugins.pluginSize) ? '1' : '0', fontSize: '13px' }}>
                正在加载{loadPlugins?.currentLoadPluginRecord?.name}@{loadPlugins?.currentLoadPluginRecord?.version}插件...
            </div>
        )
    }, [loadPlugins.currentLoadPluginSize]);

    useEffect(() => {
        clearTimeout(time);
        curLogin = loadPlugins.currentLoadPluginRecord;
        time = setTimeout(() => {
            if (curLogin == loadPlugins.currentLoadPluginRecord) {
                clearTimeout(time);
                loadPlugins.updateCurrentLoadPluginSize(loadPlugins.pluginSize);
            }
        }, 5000);
    }, [loadPlugins.currentLoadPluginSize]);

    return (
        <Fragment>
            <Layout.Header className="idpheader" onClick={() => vis ? dispatch(changeOperatorDecision(false)) : null}>
                <Row wrap={false}>
                    <Col className={'header-left'} span={12}>
                        <div className={"header-container"}>
                            {loadMenuPlugs}
                            {loadPluginsPage}
                        </div>
                    </Col>
                    <Col className={'header-right'} span={12} style={{ whiteSpace: "nowrap", display: "flex", justifyContent: 'flex-end', color: 'rgb(255, 255, 255)' }}>
                        {loadToolPlugs}
                        <div style={{ marginLeft: '10px' }}></div>
                    </Col>
                </Row>
            </Layout.Header>
            <ShortcutList />
        </Fragment>
    )
}

export default observer(IdpHeader);
