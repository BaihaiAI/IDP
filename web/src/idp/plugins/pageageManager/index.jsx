import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";
import { Layout } from "antd";
import intl from "react-intl-universal";
import Package from './pages/Package';
import RightIcons from "idpStudio/components/Icons/RigftIcons"

const { Sider } = Layout;

const configJson = require("./config.json")

const rightSide = {
    key: 'package',
    title: () => intl.get("PACKAGE_MANAGER"),
    icon: <RightIcons.BHPackageIcon style={{ fontSize: 33 }} />,
    menuItemStyle: {
        paddingLeft: "5px",
        paddingRight: "6px",
        paddingTop: "4px",
    },
    component: (<Sider
        theme="light"
        width="300"
        style={{ height: document.body.clientHeight - 40 }}
    >
        <Package />
    </Sider>),
    weight: 1
}

globalData.register(RegisterApi.right_side_api, {
    rightSide,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: '包管理器'
})
