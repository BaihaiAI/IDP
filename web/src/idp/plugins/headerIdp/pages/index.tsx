import { useEffect, useLayoutEffect, useState } from "react";
import { Menu, Dropdown, Form } from "antd";
import { useUpdate } from "ahooks";
import { DownOutlined } from "@ant-design/icons";
import PubSub from "pubsub-js";
import VersionDetails from "@/components/menu/VersionDetails";
import { observer } from "mobx-react";
import sneltoets from '@idp/global/sneltoets';
import globalData from "@/idp/global";
import userInfo from "@/idp/global/userinfo";
import intl from "react-intl-universal";
import { locationToProjectListPage } from "@/utils";
import useHeaderIdp from '@/components/header/useHeaderIdp';
import fileManager from  '@/idp/global/fileManager';

const { SubMenu } = Menu
const { useForm } = Form

const HeaderMenu = () => {

    const [isControlDropDownMenu, setIsControlDropDownMenu] = useState(false)
    const headerIdp = useHeaderIdp();

    useEffect(() => {
        const subscriber = PubSub.subscribe("isControlDropDownMenu", (msg, data) => {
            setIsControlDropDownMenu(data)
        })
        return () => {
            PubSub.unsubscribe(subscriber)
        }
    }, [])


    const [visible, setVisible] = useState(false);

    const update = useUpdate();

    useEffect(() => {
        const refreshHeader = PubSub.subscribe("refreshHeader", () => { update() });
        return () => PubSub.unsubscribe(refreshHeader);
    }, []);

    const handleClick = (e) => {
        const { workspaceRef, notebookTabRef } = globalData.appComponentData
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
            sneltoets.updateLineNumbers(!sneltoets.lineNumbers);
        } else if (e.key === "collapse_all_output") {
            sneltoets.updateCollapseAllOutput(!sneltoets.collapseAllOutput);
        } else if (e.key === "collapse_all_input") {
            sneltoets.updateCollapseAllInput(!sneltoets.collapseAllInput);
        } else if (e.key === "auto_warp") {
            sneltoets.updateAutoWarpOutput(!sneltoets.autoWarpOutput);
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
            sneltoets.updateSneltoetsListVisible(true);
        } else if ( e.key === 'document') {
            window.open(`https://baihai-idp.yuque.com/mwvla8/ps6ml8?#`)
        }
    }

    const cancelLook = () => {
        setVisible(false)
    };

    const goTeam = (e) => {
        e.domEvent.stopPropagation();
        // @ts-ignore
        locationToProjectListPage({ path: 'project' })
    }

    const logoOverlayMenuOther = (
        <div className="bh_top_menu">
            <Menu
                theme="dark"
                mode="vertical"
                className="main-menu"
                onClick={handleClick} /*触发菜单*/
                selectable={false}
            >
                <SubMenu style={{ borderBottom: '1px solid #ffffff52', display: userInfo?.navType === 'AIGC' ? 'none' : 'block' }} className="idps-header-team" onTitleClick={(e) => goTeam(e)} key="team" title='返回团队空间'></SubMenu>
                <SubMenu key="help" title={intl.get("MENU_HELP")}>
                    <Menu.Item key="document">
                        {intl.get("SUB_MENU_DOCUMENT")}
                    </Menu.Item>
                    <Menu.Item key="about">{intl.get("SUB_MENU_ABOUT")}</Menu.Item>
                </SubMenu>
            </Menu>
        </div>
    )

    const exportFile = (fileName) => {
        const fileType = fileName.substring(fileName.lastIndexOf('.'), fileName.length);
        if (['.ipynb', '.idpnb'].includes(fileType)) {
            return fileType;
        }
    }

    const logoOverlayMenu = (
        <div className="bh_top_menu">
            <Menu
                theme="dark"
                mode="vertical"
                className="main-menu"
                onClick={handleClick} /*触发菜单*/
                selectable={false}
            >
                <SubMenu style={{ borderBottom: '1px solid #ffffff52', display: userInfo?.navType === 'AIGC' ? 'none' : 'block' }} className="idps-header-team" onTitleClick={(e) => goTeam(e)} key="team" title='返回团队空间'></SubMenu>
                <SubMenu key="file" title={intl.get("MENU_FILE")}>
                    <Menu.Item key="newfile">{intl.get("SUB_MENU_NEW_FILE")}</Menu.Item>
                    <Menu.Item key="newfolder">
                        {intl.get("SUB_MENU_NEW_FOLDER")}
                    </Menu.Item>
                    <Menu.Item disabled key="openfile">
                        {intl.get("SUB_MENU_OPEN_FILE")}
                    </Menu.Item>
                    <SubMenu
                        key="recent"
                        title={intl.get("SUB_MENU_OPEN_RECENT")}
                        onTitleMouseEnter={() => sneltoets.headerGlobal.onTitleMouseEnter()}
                    >
                        {sneltoets.headerGlobal.historyOpenFiles.map((item, i) => {
                            return <Menu.Item key={item}>{item}</Menu.Item>
                        })}
                    </SubMenu>
                    <Menu.Item key="importfiles">
                        {intl.get("SUB_MENU_IMPORTFILES")}
                    </Menu.Item>
                    <Menu.Item key="importfolders">
                        {intl.get("SUB_MENU_IMPORTFOLDERS")}
                    </Menu.Item>
                    <Menu.Item key="rename">{intl.get("SUB_MENU_RENAME")}</Menu.Item>
                    <Menu.Item key="save" disabled>
                        {intl.get("SUB_MENU_SAVE")}
                    </Menu.Item>
                    <Menu.Item disabled key="save_as">
                        {intl.get("SUB_MENU_SAVE_AS")}
                    </Menu.Item>
                    <Menu.Item key="delete">{intl.get("SUB_MENU_DELETE")}</Menu.Item>
                    <Menu.Item key="versions" disabled>
                        {intl.get("SUB_MENU_VERSION")}
                    </Menu.Item>
                    {sneltoets.headerGlobal.isShowExportChildren() ? (
                        <SubMenu key="export" title={intl.get("SUB_MENU_EXPORT")}>
                            <Menu.Item key="export_ipynb">{fileManager?.fileName ? exportFile(fileManager.fileName) : '.ipynb'}</Menu.Item>
                            <Menu.Item key="export_html">.html</Menu.Item>
                            {/*<Menu.Item key="export_pdf">PDF</Menu.Item>*/}
                            <Menu.Item key="export_python">.py</Menu.Item>
                        </SubMenu>
                    ) : (
                        <Menu.Item key="export">{intl.get("SUB_MENU_EXPORT")}</Menu.Item>
                    )}
                </SubMenu>
                <SubMenu key="view" title={intl.get("MENU_VIEW")}>
                    {sneltoets.lineNumbers ? (
                        <Menu.Item key="showline">
                            {intl.get("MENU_SHOW_LINE_CHECKED")}
                        </Menu.Item>
                    ) : (
                        <Menu.Item key="showline">{intl.get("MENU_SHOW_LINE")}</Menu.Item>
                    )}
                    {sneltoets.autoWarpOutput ? (
                        <Menu.Item key="auto_warp">
                            {intl.get("MENU_SHOW_AUTO_WARP_CHECKED")}
                        </Menu.Item>) : (
                        <Menu.Item key="auto_warp">
                            {intl.get("MENU_SHOW_AUTO_WARP")}
                        </Menu.Item>
                    )}
                    {sneltoets.collapseAllInput ? (
                        <Menu.Item key="collapse_all_input">
                            {intl.get("MENU_COLLAPSE_ALL_INPUT_CHECKED")}
                        </Menu.Item>
                    ) : (
                        <Menu.Item key="collapse_all_input">
                            {intl.get("MENU_COLLAPSE_ALL_INPUT")}
                        </Menu.Item>
                    )}
                    {sneltoets.collapseAllOutput ? (
                        <Menu.Item key="collapse_all_output">
                            {intl.get("MENU_COLLAPSE_ALL_OUTPUT_CHECKED")}
                        </Menu.Item>
                    ) : (
                        <Menu.Item key="collapse_all_output">
                            {intl.get("MENU_COLLAPSE_ALL_OUTPUT")}
                        </Menu.Item>
                    )}
                    <Menu.Item disabled key="change_theme">
                        {intl.get("MENU_CHANGE_THEME")}
                    </Menu.Item>
                    <Menu.Item disabled key="full_screen">
                        {intl.get("MENU_FULL_SCREEN")}
                    </Menu.Item>
                </SubMenu>
                <SubMenu key="run" title={intl.get("MENU_RUN")}>
                    <Menu.Item key="stop_selected_cell">
                        {intl.get("SUB_MENU_STOP")}
                    </Menu.Item>
                    <Menu.Item key="stop_all_cell">
                        {intl.get("SUB_MENU_STOP_ALL_CELL")}
                    </Menu.Item>
                    <Menu.Item key="run_selected_cell">
                        {intl.get("SUB_MENU_RUN_SELECTED_CELL")}
                    </Menu.Item>
                    <Menu.Item key="run_all_cell">
                        {intl.get("SUB_MENU_RUN_ALL_CELL")}
                    </Menu.Item>
                    <Menu.Item key="run_above_selected_all_cell">
                        {intl.get("SUB_MENU_RUN_ABOVE_ALL")}
                    </Menu.Item>
                    <Menu.Item key="run_under_selected_all_cell">
                        {intl.get("SUB_MENU_RUN_UNDER_ALL")}
                    </Menu.Item>
                    <Menu.Item key="run_selected_line">
                        {intl.get("SUB_MENU_RUN_SELECTED_LINE")}
                    </Menu.Item>
                </SubMenu>
                <SubMenu key="kernal" title={intl.get("MENU_KERNAL")}>
                    <Menu.Item disabled key="break">
                        {intl.get("SUB_MENU_BREAK")}
                    </Menu.Item>
                    <Menu.Item key="restart">{intl.get("SUB_MENU_RESTART")}</Menu.Item>
                </SubMenu>
                <SubMenu key="tools" title={intl.get("MENU_TOOLS")}>
                    <Menu.Item disabled key="mount">
                        {intl.get("SUB_MENU_MOUNTS3")}
                    </Menu.Item>
                    <Menu.Item disabled key="command">
                        {intl.get("SUB_MENU_COMMAND")}
                    </Menu.Item>
                    <Menu.Item key="keybord_shortcut">
                        {/* 快捷键0 */}
                        {intl.get("SUB_MENU_KEYBORD_SHORTCUT")}
                    </Menu.Item>
                </SubMenu>
                <SubMenu key="help" title={intl.get("MENU_HELP")}>
                    <Menu.Item key="document">
                        {intl.get("SUB_MENU_DOCUMENT")}
                    </Menu.Item>
                    <Menu.Item key="about">{intl.get("SUB_MENU_ABOUT")}</Menu.Item>
                </SubMenu>
            </Menu>
        </div>
    )

    useEffect(() => { }, [sneltoets.headerGlobal.historyOpenFiles]);

    return (
        <div className={"header-container"}>
            {
                isControlDropDownMenu ? (
                    <Dropdown visible={true} overlayClassName={"logo-dropdown"} overlay={headerIdp ? logoOverlayMenu : logoOverlayMenuOther} arrow>
                        <div id={'tour-header-drop-down'} className={"logo-dropdown-content-wrapper"}>
                            <div className="logo" style={{ backgroundImage: `url(${require('@/assets/logo/logo.png').default})` }} />
                            <DownOutlined style={{ color: "white" }} />
                        </div>
                    </Dropdown>
                ) : (
                    <Dropdown overlayClassName={"logo-dropdown"} overlay={headerIdp ? logoOverlayMenu : logoOverlayMenuOther} arrow>
                        <div id={'tour-header-drop-down'} className={"logo-dropdown-content-wrapper"}>
                            <div className="logo" style={{ backgroundImage: `url(${require('@/assets/logo/logo.png').default})` }} />
                            <DownOutlined style={{ color: "white" }} />
                        </div>
                    </Dropdown>
                )
            }
            <VersionDetails visible={visible} cancel={cancelLook} />
        </div>
    )
}

export default observer(HeaderMenu)
