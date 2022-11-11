import { IdpMenus, IdpTools } from "@/idp/lib/tool";
import { Menu } from "antd";
import { useEffect } from "react";

function IDP_Header_Example_Tool() {

    return <>
        <span>测试Tool插件</span>
    </>
}

IdpMenus.registerIdpMenu('team_menu', {
    menuType: 'Menu',
    content: <IDP_Header_Example_Tool />,
});

IdpTools.registerIdpTool('team_menu', {
    label: '',
    key: "test"
})