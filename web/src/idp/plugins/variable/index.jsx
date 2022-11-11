import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";
import intl from "react-intl-universal";
import VariableManager from './pages/variableManager';
import RightIcons from "idpStudio/components/Icons/RigftIcons"

const configJson = require("./config.json")

const rightSide = {
    key: 'variable',
    title: () => intl.get("NOTEBOOK_VARIABLE_MANAGER"),
    icon: <RightIcons.BHVariableIcon style={{ fontSize: 30 }} />,
    menuItemStyle: {
        paddingLeft: "6px",
        paddingRight: "6px",
        paddingTop: "4px",
    },
    component: <VariableManager showVariableManager={true} />,
    weight: 3
}

globalData.register(RegisterApi.right_side_api, {
    rightSide,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: '变量管理器'
})
