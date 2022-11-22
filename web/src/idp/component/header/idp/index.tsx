import { useEffect, useState } from "react";
import { Menu, Dropdown, Form } from "antd";
import { useUpdate } from "ahooks";
import { DownOutlined } from "@ant-design/icons";
import PubSub from "pubsub-js";
import VersionDetails from "@/components/menu/VersionDetails";
import { observer } from "mobx-react";
import { toJS } from "mobx";
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';
import HeaderGlobal, { HeaderGlobal as HeaderGlobalBean } from '@/idp/global/header';
import globalData from "@/idp/global";
import useHeaderIdp from '@/components/header/useHeaderIdp';

const { SubMenu } = Menu
const { useForm } = Form

const HeaderMenu = () => {

    const [visible, setVisible] = useState(false);
    const headerIdp = useHeaderIdp();

    const update = useUpdate();

    useEffect(() => {
        const refreshHeader = PubSub.subscribe("refreshHeader", () => { update() });
        return () => PubSub.unsubscribe(refreshHeader);
    }, []);

    const handleClick = (e) => {
        const { workspaceRef, notebookTabRef } = globalData.appComponentData
        HeaderGlobal.onClick(e);
        if (e.key === "newfile") {
            workspaceRef.addFile({ event: null, props: null })
        } else if (e.key === "newfolder") {
            workspaceRef.addFolder({ event: null, props: null })
        } else if (e.key === "open") {
        } else if (e.key === "recent") {
        } else if (e.key === "importfiles") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            document.getElementById("chooseFiles").click()
        } else if (e.key === "importfolders") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            document.getElementById("chooseFolder").click()
        } else if (e.key === "rename") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.menu_rename()
        } else if (e.key === "save") {
        } else if (e.key === "save_as") {
            //todo
        } else if (e.key === "delete") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.delete({ event: null, props: null })
        } else if (e.key === "versions") {
        } else if (e.key === "export") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.menu_onExportClick()
        } else if (e.key === "export_ipynb") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.menu_onExportClick()
        } else if (e.key === "export_html") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.menu_onExportClick("html")
        } else if (e.key === "export_pdf") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.menu_onExportClick("pdf")
        } else if (e.key === "export_python") {
            if (!workspaceRef.handleIsFileTree()) {
                return
            }
            workspaceRef.menu_onExportClick("python")
        } else if (e.key === "showline") {
            ToolImpl.updateLineNumbers(!ToolImpl.lineNumbers);
        } else if (e.key === "collapse_all_output") {
            ToolImpl.updateCollapseAllOutput(!ToolImpl.collapseAllOutput);
        } else if (e.key === "collapse_all_input") {
            ToolImpl.updateCollapseAllInput(!ToolImpl.collapseAllInput);
        } else if (e.key === "auto_warp") {
            ToolImpl.updateAutoWarpOutput(!ToolImpl.autoWarpOutput);
        } else if (e.key === "change_theme") {
        } else if (e.key === "full_screen") {
        } else if (e.key === "stop_selected_cell") {
            notebookTabRef.current.stopCell()
        } else if (e.key === "stop_all_cell") {
            notebookTabRef.current.stopAllCell()
        } else if (e.key === "run_selected_cell") {
            notebookTabRef.current.runCell()
        } else if (e.key === "run_all_cell") {
            notebookTabRef.current.runAllCell()
        } else if (e.key === "run_above_selected_all_cell") {
            notebookTabRef.current.runPreCell()
        } else if (e.key === "run_under_selected_all_cell") {
            notebookTabRef.current.runNextCell()
        } else if (e.key === "run_selected_line") {
            notebookTabRef.current.runCell()
        } else if (e.key === "restart") {
            notebookTabRef.current.restartKernel()
        } else if (e.key === "feedback") {
            // feedback
        } else if (e.key === "about") {
            setVisible(true)
        } else if (e.key.startsWith("/")) {
            const node = {
                key: e.key,
                name: e.key.substring(e.key.lastIndexOf("/")),
                isLeaf: true,
                fileType: "FILE",
            }
            const info = {
                node,
            }
            workspaceRef.onSelect(null, info);
        } else if (e.key === 'keybord_shortcut') {
            ToolImpl.updateSneltoetsListVisible(true);
        }
    }

    const cancelLook = () => {
        setVisible(false)
    };

    const logoOverlayMenu = (
        <div className="bh_top_menu">
            <Menu
                theme="dark"
                mode="vertical"
                className="main-menu"
                selectable={false}
                onClick={handleClick} /*触发菜单*/
            >
                {
                    toJS(ToolImpl.idpToolMap).filter(it => it.nodeKey === 'idps').map(it => {
                        const flg = headerIdp.includes(it.key);
                        if (flg) return it.items()
                    })
                }
            </Menu>
        </div>
    )

    useEffect(() => { }, [ToolImpl.headerGlobal.historyOpenFiles]);

    const goHome = (e) => {
        if ( process.env.REACT_APP_VERSION === 'MODEL' ) {
            const openMenuDIv = document.getElementsByClassName('ant-dropdown-open');
            if ( openMenuDIv.length > 0 ) {
                const flg = openMenuDIv[0].contains(e.target);
                if (flg) {
                    window.location.href = '/sharePlatform'
                }
            }
        }
    };

    return (
        <div className={"header-container"} onClick={(e) => goHome(e)}>
            <Dropdown overlayClassName={"logo-dropdown"} overlay={logoOverlayMenu} arrow>
                <div className={"logo-dropdown-content-wrapper"}>
                    <div className="logo" style={{ backgroundImage: `url(${require('@/assets/logo/logo.png').default})` }} />
                    <DownOutlined style={{ color: "white" }} />
                </div>
            </Dropdown>
            <VersionDetails visible={visible} cancel={cancelLook} />
        </div>
    )
}

export default observer(HeaderMenu)
