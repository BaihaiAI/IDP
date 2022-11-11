import { IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import intl from "react-intl-universal";
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';

const { SubMenu } = Menu;

export const runMenus = () => {

    return (
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
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_run",
    items: runMenus
})