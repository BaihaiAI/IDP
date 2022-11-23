import React, { useEffect, useState } from "react";
import { notification, Tree, Spin } from "antd";
import warenhouseApi from '@/services/warenhouseApi';
import './zipview.less'
import { observer } from "mobx-react";

const { DirectoryTree } = Tree;

import terminal from '@/idp/lib/terminal';
import { toJS } from "mobx";

function ZipView(props) {

    const { path, name, content } = props.item;
    const [list, setList] = useState(content)
    const [expandNode, setExpandNode] = useState([]);

    useEffect(() => {
        if (list?.length !== 0) {
            setExpandNode(list[0]?.absolutePath)
        }
    }, [list])

    function loop(data) {
        return data.map((item) => {
            let title = (
                <div className={'add-title'}>
                    <span className="ant-tree-title">
                        {item.fileName}
                    </span>
                </div>
            )
            if (item.children) {
                return {
                    title,
                    key: item.absolutePath,
                    children: loop(item.children),
                }
            }
            return {
                title,
                key: item.absolutePath
            }
        })
    }

    const unZipFolder = async () => {
        const key = toJS(terminal.openFilePath);
        const node = key.replace(/\s*/g, "");
        const nodeFileName = node.split('/').reduce((previousValue, currentValue, currentIndex, array) => {
            if ((array.length - 1) === currentIndex) return currentValue;
        });
        const nodes = node.split(nodeFileName).filter(it => it != '').join('');
        const nodePath = nodes.substring(0, nodes.length - 1);
        try {
            const reuslt = await warenhouseApi.decompressFile(key, nodePath);
            if (reuslt.code == '21000000') {
                openNotification('open', <span>解压ZIP文件完成，请到文件管理器{<span style={{ color: '#3793EF' }}>重新刷新</span>}查看</span>, 5);
            } else {
                openNotification('error', `解压ZIP文件失败`);
            }
        } catch (error) {
            openNotification('error', `解压ZIP文件失败，错误信息：${error}`);
        }
    }

    const openNotification = (type, description, duration = 3) => {
        notification[type]({
            message: '解压ZIP',
            description,
            duration,
            onClick: () => { console.log('Notification Clicked!') },
            placement: "bottomRight"
        });
    };

    return (
        <div className="zipview">
            <div className="zipview-header">
                <div style={{ position: 'relative' }}>
                    <span className="zh-span">预览 {name}</span>
                    <span onClick={() => unZipFolder()} className="zh-zip" style={{ right: terminal.leftSideWidth === 0 ? 60 : 360 }}>解压ZIP</span>
                </div>
            </div>
            <div className="zipview-content">
                {list?.length !== 0 && expandNode?.length !== 0 ? (
                    <DirectoryTree
                        showIcon={false}
                        treeData={loop(list)}
                        autoExpandParent={false}
                        defaultExpandedKeys={[expandNode]} // expandNode
                    />
                ) : <div className="zipview-spin"><Spin size="large" /></div>}

            </div>
        </div>
    )
}


export default observer(React.forwardRef(ZipView))
