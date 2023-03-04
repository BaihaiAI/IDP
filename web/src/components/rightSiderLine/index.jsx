import React, { useEffect, useState } from "react"
import { Layout, Menu, Tooltip } from "antd"
import "./index.less"
import classNames from "classnames"
import RightIcons from "../Icons/RigftIcons"
import intl from "react-intl-universal"
import { useDispatch, useSelector } from "react-redux"
import { changeOperatorDecision, selectOperatorDecision, changeUpdateList } from '@/store/features/globalSlice'
import { observer } from "mobx-react"
import globalData from "idp/global"
import { toJS } from "mobx"

import IdpTerminal from '@/idp/lib/terminal';

const { Sider } = Layout

function RightSideLine(props) {
    const {
        rightLineSelectKey,
        setRightLineSelectKey,
        showVersionDrawer,
        setShowVersionDrawer,
    } = props

    const dispatch = useDispatch()
    const vis = useSelector(selectOperatorDecision)

    useEffect(() => {
        IdpTerminal.setRightBarOpenStatus(rightLineSelectKey.length == 0 || rightLineSelectKey === 'historyVersion' ? false : true);
    }, [rightLineSelectKey]);

    const [openRightPane, setOpenRightPane] = useState('');

    const rightSideList = toJS(globalData.rightSideControl.rightSideList)

    return (
        <div className={classNames("last-right-side")}>
            <Sider
                collapsed={true}
                width={48}
                collapsedWidth={48}
                style={{ paddingTop: 32 }}
                onClick={() => {
                    if (vis) {
                        dispatch(changeOperatorDecision(false))
                    }
                }}
            >
                <Menu
                    style={{ padding: "0 3px" }}
                    theme="dark"
                    mode="inline"
                    selectedKeys={[rightLineSelectKey]}
                    onClick={(option) => {
                        const { key } = option;
                        setRightLineSelectKey(key === rightLineSelectKey ? "" : key, rightLineSelectKey);
                        IdpTerminal.setRightBarOpenStatus(key.length == 0 || key === 'historyVersion' ? false : true);
                        if (key === "historyVersion") {
                            setShowVersionDrawer(!showVersionDrawer)
                            dispatch(changeUpdateList())
                        } else {
                            if (rightLineSelectKey === key) {
                                IdpTerminal.setRightSidePanelWidth(0);
                                setOpenRightPane('');
                            } else {
                                IdpTerminal.setRightSidePanelWidth(-300);
                                setOpenRightPane(key);
                            }
                        }
                    }}
                >
                    {
                        rightSideList.map(item => {
                            return (
                                <Menu.Item
                                    key={item.key}
                                    style={item.menuItemStyle}
                                    icon={item.icon}
                                >
                                    {
                                        Object.prototype.toString.call(item.title) === '[object Function]' ? item.title() : item.title
                                    }
                                </Menu.Item>
                            )
                        })
                    }
                </Menu>
            </Sider>
        </div>
    )
}

export default observer(RightSideLine)
