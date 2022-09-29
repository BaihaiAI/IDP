
import GlobalSearchFileModal from "@/components/globalSearchFileModal";
import { IdpMenus } from "@/idp/lib/menu";
import { SearchOutlined } from "@ant-design/icons";
import { useEffect, useState } from "react";
import PubSub from "pubsub-js"
import intl from "react-intl-universal";

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
        </span>
        <div style={{ display: "inline-block", color: "#fff", marginLeft: "16px", marginRight: "16px", fontSize: "20px", position: 'relative', top: '-1px' }} >
            |
        </div>
        <GlobalSearchFileModal
            globalSearchVisible={globalSearchVisible}
            setGlobalSearchVisible={setGlobalSearchVisible}
        />
    </>
}

IdpMenus.registerIdpMenu('search', {
    menuType: 'Tool',
    content: <IDP_Header_Search_Plugs />,
});