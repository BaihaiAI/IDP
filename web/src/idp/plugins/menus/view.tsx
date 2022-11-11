import { IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import intl from "react-intl-universal";
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';

const { SubMenu } = Menu;

export const viewMenus = () => {

    return (
        <SubMenu key="view" title={intl.get("MENU_VIEW")}>
            {ToolImpl.lineNumbers ? (
                <Menu.Item key="showline">
                    {intl.get("MENU_SHOW_LINE_CHECKED")}
                </Menu.Item>
            ) : (
                <Menu.Item key="showline">{intl.get("MENU_SHOW_LINE")}</Menu.Item>
            )}
            {ToolImpl.autoWarpOutput ? (
                <Menu.Item key="auto_warp">
                    {intl.get("MENU_SHOW_AUTO_WARP_CHECKED")}
                </Menu.Item>) : (
                <Menu.Item key="auto_warp">
                    {intl.get("MENU_SHOW_AUTO_WARP")}
                </Menu.Item>
            )}
            {ToolImpl.collapseAllInput ? (
                <Menu.Item key="collapse_all_input">
                    {intl.get("MENU_COLLAPSE_ALL_INPUT_CHECKED")}
                </Menu.Item>
            ) : (
                <Menu.Item key="collapse_all_input">
                    {intl.get("MENU_COLLAPSE_ALL_INPUT")}
                </Menu.Item>
            )}
            {ToolImpl.collapseAllOutput ? (
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
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_view",
    items: viewMenus
})