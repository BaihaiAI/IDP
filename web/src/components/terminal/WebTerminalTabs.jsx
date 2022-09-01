import { Tabs } from "antd";
import { useEffect, useState } from "react";
import WebTerminal from "./WebTerminal";

const { TabPane } = Tabs;

let tableIndex = 1;

const WebTerminalTabs = () => {

    const [panes, setPanes] = useState([]);
    const [activeKey, setActiveKey] = useState("");

    const onChange = (activeKey) => {
        setActiveKey(activeKey);
    }

    const onEdit = (targetKey, action) => {
        action === "remove" && remove(targetKey);
        action === "add" && add();
    }

    const add = () => {
        const activeKey = `terminal ${tableIndex++}`;
        const newPanes = [...panes];
        newPanes.push({ title: activeKey, key: activeKey });
        setPanes(newPanes);
        setActiveKey(activeKey);
    };

    const remove = (targetKey) => {
        let newActiveKey = activeKey;
        let lastIndex;
        panes.forEach((pane, i) => {
            if (pane.key === targetKey) {
                lastIndex = i - 1;
            }
        });
        const newPanes = panes.filter(pane => pane.key !== targetKey);
        if (newPanes.length && newActiveKey === targetKey) {
            if (lastIndex >= 0) {
                newActiveKey = newPanes[lastIndex].key;
            } else {
                newActiveKey = newPanes[0].key;
            }
        }
        setPanes(newPanes);
        setActiveKey(newActiveKey);
    };

    useEffect(() => {
        add();
    }, []);

    return (
        <div id="terminal-tabs-container">
            <Tabs type="editable-card" onChange={onChange} activeKey={activeKey} onEdit={onEdit} >
                {panes.map(pane => (
                    <TabPane tab={pane.title} key={pane.key} closable={panes.length > 1}>
                        <WebTerminal 
                            key={pane.key} 
                            terminalId={pane.key} 
                        />
                    </TabPane>
                ))}
            </Tabs>
        </div>
    );
}

export default WebTerminalTabs;
