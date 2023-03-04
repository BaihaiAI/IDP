
import GlobalSearchFileModal from "@/components/globalSearchFileModal";
import { SearchOutlined } from "@ant-design/icons";
import { useEffect, useState } from "react";
import PubSub from "pubsub-js"
import intl from "react-intl-universal";
import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";

const configJson = require("./config.json")

function IDP_Header_Search_Plugs() {
    useEffect(() => {
        const globalsearch = PubSub.subscribe(
            "openGlobalSearch",
            () => {
                setGlobalSearchVisible(true)
            }
        )
        return () => {
            PubSub.unsubscribe(globalsearch)
        }
    })

    const [globalSearchVisible, setGlobalSearchVisible] = useState(false)

    return <>
        <span style={{ cursor: "pointer" }} onClick={() => { setGlobalSearchVisible(true) }} >
            <SearchOutlined style={{ color: "white", marginRight: 5, fontSize: 16 }} />
            <div style={{ color: "#fff", textAlign: "center", display: "inline-block" }} >
                {intl.get("SEARCH")}
            </div>
            <div style={{ display: "inline-block", color: "#fff", marginLeft: "16px", marginRight: "16px", fontSize: "20px" }} >|</div>
        </span>
        <GlobalSearchFileModal
            globalSearchVisible={globalSearchVisible}
            setGlobalSearchVisible={setGlobalSearchVisible}
        />
    </>
}

const config = {
    key: 'search',
    component: (<IDP_Header_Search_Plugs />),
    weight: 2,
}

globalData.register(RegisterApi.header_tool_api, {
    headerTool: config,
    id: `${configJson.fileName}/${configJson.entry}`,
})