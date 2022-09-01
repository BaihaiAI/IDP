import { Col, Space, Layout, Menu, Row, Dropdown } from "antd";
import { observer } from "mobx-react";
import { Fragment, useMemo } from "react"
import { useDispatch, useSelector } from "react-redux";
import { changeOperatorDecision, selectOperatorDecision } from "@/store/features/globalSlice";
import Menus from '@/idp/lib/menu/impl/menuImpl';
import { toJS } from 'mobx';
import ShortcutList from "@/components/globalpopup/ShortcutList";

import './index.less';
import { DownOutlined } from "@ant-design/icons";

const { SubMenu } = Menu;

type Props = {}

const IdpHeader: React.FC<Props> = () => {

    const dispatch = useDispatch();

    const vis = useSelector(selectOperatorDecision);

    // 初始化内部自定义数据
    const loadToolPlugs = useMemo(() => {
        const data = toJS(Menus.toolMap);
        const r = data?.map((item, index) => {
            return <div key={`tool_${index}`} style={{ display: 'flex', cursor: "pointer", alignItems: 'center' }}>
                {item.content}
                <div style={{ display: "inline-block", color: "#fff", marginLeft: "16px", marginRight: "16px", fontSize: "20px" }} >
                    |
                </div>
            </div>
        });
        return r;
    }, [toJS(Menus.toolMap)]);

    // 初始化外部菜单数据
    const loadMenuPlugs = useMemo(() => {
        const data = toJS(Menus.menuMap);
        // const overlay = toJS(ToolImpl.toolMap).filter( it => it.nodeKey == 'team_menu');
        const r = data?.map((item, index) => {
            return <div key={`menu_${index}`} style={{ color: '#fff', marginLeft: '8px', cursor: 'pointer', fontSize: '13px' }}>
                <Dropdown overlay={() => null}>
                    <a onClick={e => e.preventDefault()}>
                        <Space>
                            <span style={{ color: '#fff', fontSize: '10px' }}>{item.content}</span>
                            <DownOutlined style={{ color: "white", fontSize: '10px' }} />
                        </Space>
                    </a>
                </Dropdown>
            </div>
        });
        return r;
    }, [toJS(Menus.menuMap)]);

    // 初始化内部菜单数据
    const loadIdpMenuPlugs = useMemo(() => {
        const data = toJS(Menus.idpMenuMap);
        const r = data?.map((item, index) => {
            return <div key={`idp_menu_${index}`}>
                {item.content}
            </div>
        });
        return r;
    }, [toJS(Menus.idpMenuMap)]);

    // 初始化内部自定义数据
    const loadIdpToolPlugs = useMemo(() => {
        const data = toJS(Menus.idpToolMap);
        const r = data?.map((item, index) => {
            return <div style={{ display: 'flex', alignItems: 'center' }} key={`idp_tool_${index}`}>
                {item.content}
            </div>
        });
        return r;
    }, [toJS(Menus.idpToolMap)]);

    return (
        <Fragment>
            <Layout.Header className="idpheader" onClick={() => vis ? dispatch(changeOperatorDecision(false)) : null}>
                <Row wrap={false}>
                    <Col className={'header-left'} span={12}>
                        <div style={{ display: 'flex' }}>
                            {loadIdpMenuPlugs}
                            {loadMenuPlugs}
                        </div>
                    </Col>
                    <Col className={'header-right'} span={12} style={{ whiteSpace: "nowrap", display: "flex", justifyContent: 'flex-end', color: 'rgb(255, 255, 255)' }}>
                        {loadToolPlugs}
                        {loadIdpToolPlugs}
                    </Col>
                </Row>
            </Layout.Header>
            <ShortcutList />
        </Fragment>
    )
}

export default observer(IdpHeader);
