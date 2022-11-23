import { IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import intl from "react-intl-universal";
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';

const { SubMenu } = Menu;

export const toolMenus = () => {
    return (
        <SubMenu key="tools" title={intl.get("MENU_TOOLS")}>
            <Menu.Item disabled key="mount">
                {intl.get("SUB_MENU_MOUNTS3")}
            </Menu.Item>
            <Menu.Item disabled key="command">
                {intl.get("SUB_MENU_COMMAND")}
            </Menu.Item>
            <Menu.Item key="keybord_shortcut">
                {intl.get("SUB_MENU_KEYBORD_SHORTCUT")}
            </Menu.Item>
        </SubMenu>
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_tool",
    items: toolMenus
})