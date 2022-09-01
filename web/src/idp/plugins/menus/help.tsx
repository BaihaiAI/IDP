import { IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import intl from "react-intl-universal";

const { SubMenu } = Menu;

export const helpMenus = () => {
    return (
        <SubMenu key="help" title={intl.get("MENU_HELP")}>
            <Menu.Item disabled key="document">
                {intl.get("SUB_MENU_DOCUMENT")}
            </Menu.Item>
            <Menu.Item key="about">{intl.get("SUB_MENU_ABOUT")}</Menu.Item>
        </SubMenu>
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_help",
    items: helpMenus,
})