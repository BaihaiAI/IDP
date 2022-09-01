import { IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import intl from "react-intl-universal";
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';

const { SubMenu } = Menu;

export const kernelMenus = () => {
    return (
        <SubMenu key="kernal" title={intl.get("MENU_KERNAL")}>
            <Menu.Item disabled key="break">
                {intl.get("SUB_MENU_BREAK")}
            </Menu.Item>
            <Menu.Item key="restart">{intl.get("SUB_MENU_RESTART")}</Menu.Item>
        </SubMenu>
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_kernel",
    items: kernelMenus
})