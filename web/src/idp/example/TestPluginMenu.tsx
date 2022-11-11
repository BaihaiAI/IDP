import { IdpMenus } from "@/idp/lib/tool";
import { useEffect, useState } from "react";
import { Modal } from "antd";

function IDP_Header_Example_Menu() {

    const [visable, setVisable] = useState(false);

    return <>
        <>
            <div onClick={() => setVisable(true)}>测试Menu插件</div>
            <Modal title="测试插件Menu" visible={visable} onOk={() => setVisable(false)} onCancel={() => setVisable(false)}>
                <p>Some contents...</p>
                <p>Some contents...</p>
                <p>Some contents...</p>
            </Modal>
        </>
    </>
}

export default IDP_Header_Example_Menu

IdpMenus.registerIdpMenu('team1', {
    menuType: 'Tool',
    content: <IDP_Header_Example_Menu />,
});