import { IdpTools } from "@/idp/lib/tool";
import { locationToProjectListPage } from "@/utils"
import { Menu } from "antd";

const { SubMenu } = Menu;

export const runMenus = () => {

    const goTeam = (e) => {
        e.domEvent.stopPropagation();
        locationToProjectListPage({ path: 'project' })
    }

    return (
        <SubMenu style={{ borderBottom: '1px solid #ffffff52' }} className="idps-header-team" onTitleClick={(e) => goTeam(e)} key="team" title='返回团队空间'></SubMenu>
    )
}

IdpTools.registerIdpTool("idps", {
    key: "idps_team",
    items: runMenus
})