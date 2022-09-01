import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";
import intl from "react-intl-universal";
import RightIcons from "idpStudio/components/Icons/RigftIcons"

const configJson = require("./config.json")

const rightSide = {
    key: 'historyVersion',
    title: () => intl.get("VERSION"),
    icon: <RightIcons.BHHistoryIcon style={{ fontSize: 30 }} />,
    menuItemStyle: {
        paddingLeft: "6px",
        paddingRight: "6px",
        paddingTop: "4px",
    },
    component: null,
    weight: 4
}

globalData.register(RegisterApi.right_side_api, {
    rightSide,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: '快照列表',
})
