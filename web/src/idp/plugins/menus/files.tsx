import { IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import intl from "react-intl-universal";
import HeaderGlobal from '@/idp/global/header'
import globalData from "@/idp/global";

const { SubMenu } = Menu;

export const fileMenus = () => {

    IdpTools.getMenuKey().then(e => {
        // console.log(e);
    })

    const onTitleMouseEnter = () => {
        IdpTools.exeHistoryOpenFile();
    }

    return (
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
                onTitleMouseEnter={() => { onTitleMouseEnter() }}
            >
                {IdpTools.getHistoryOpenFile().map((item: any) => {
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
            {HeaderGlobal.isShowExportChildren() ? (
                <SubMenu key="export" title={intl.get("SUB_MENU_EXPORT")}>
                    <Menu.Item key="export_ipynb">.ipynb</Menu.Item>
                    <Menu.Item key="export_html">HTML</Menu.Item>
                    <Menu.Item key="export_python">PYTHON</Menu.Item>
                </SubMenu>
            ) : (
                <Menu.Item key="export">{intl.get("SUB_MENU_EXPORT")}</Menu.Item>
            )}
        </SubMenu>
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_files",
    items: fileMenus
});