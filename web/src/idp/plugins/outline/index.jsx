import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";
import intl from "react-intl-universal";
import MarkdownOutLine from './pages/MarkdownOutLine';
import RightIcons from "idpStudio/components/Icons/RigftIcons"

const configJson = require("./config.json")

const rightSide = {
    key: 'outline', // key值，和路由保持一致，必填
    title: () => intl.get("MARK_DOWN_OUT_LINE"),
    icon: <RightIcons.BHOutLineIcon style={{ fontSize: 30 }} />,
    menuItemStyle: {
        paddingLeft: "6px",
        paddingRight: "6px",
        paddingTop: "4px",
    },
    component: <MarkdownOutLine />,
    weight: 2
}

globalData.register(RegisterApi.right_side_api, {
    rightSide,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: '大纲'
})
