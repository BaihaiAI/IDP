
import { FileWordOutlined } from "@ant-design/icons";
import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";

const configJson = require("./config.json")

function IDP_Header_Help_Plugs() {

    const help = () => {
        window.open(`https://baihai-idp.yuque.com/mwvla8/ps6ml8?#`)
    }

    return <>
        <span style={{ cursor: "pointer" }} onClick={help} >
            <FileWordOutlined style={{ color: "white", marginRight: 5, fontSize: 15 }} />
            <div style={{ color: "#fff", textAlign: "center", display: "inline-block" }} >
                {'帮助'}
            </div>
            <div style={{ display: "inline-block", color: "#fff", marginLeft: "16px", marginRight: "16px", fontSize: "20px" }} >|</div>
        </span>
    </>
}

const config = {
    key: 'help',
    component: (<IDP_Header_Help_Plugs />),
    weight: 3,
}

globalData.register(RegisterApi.header_tool_api, {
    headerTool: config,
    id: `${configJson.fileName}/${configJson.entry}`,
})