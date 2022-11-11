
import OperatorPanel from './pages/index';
import globalData from "idpStudio/idp/global";
import intl from "react-intl-universal";
import { RegisterApi } from "idpStudio/idp/register";
import RightIcons from "idpStudio/components/Icons/RigftIcons";
const configJson = require("./config.json")


const rightSide = {
    key: 'operatorList',
    title: '公共代码片段',
    icon: <RightIcons.OperatorIcon style={{ fontSize: 30 }} />,
    menuItemStyle: {
        paddingLeft: "9.3px",
        paddingRight: "6px",
        paddingTop: "1px",
    },
    component: (<OperatorPanel />),
    weight: 5
}

globalData.register(RegisterApi.right_side_api, {
    rightSide,
    autoStart: Boolean(process.env.NODE_OPEN) ? true : false,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: '包管理器'
})