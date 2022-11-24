import React, { useEffect, useState } from "react";
import { notification, Tree, Spin } from "antd";
import warenhouseApi from '@/services/warenhouseApi';
import './zipview.less'
import { observer } from "mobx-react";
import intl from "react-intl-universal";

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
              openNotification('open', <span>{intl.get('FILE_ZIP_DECOMPRESS_INFO_1')}{<span style={{ color: '#3793EF' }}>{intl.get('FILE_ZIP_DECOMPRESS_INFO_2')}</span>}{intl.get('FILE_ZIP_DECOMPRESS_INFO_3')}</span>, 5);
            } else {
                openNotification('error', intl.get('FILE_ZIP_DECOMPRESS_ERROR'));
            }
        } catch (error) {
          openNotification('error', `${intl.get('FILE_ZIP_DECOMPRESS_ERROR') }: ${error}`);
        }
    }

    const openNotification = (type, description, duration = 3) => {
        notification[type]({
            message: intl.get('FILE_ZIP_DECOMPRESS'),
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
                    <span className="zh-span">{intl.get('PREVIEW')} {name}</span>
                    <span onClick={() => unZipFolder()} className="zh-zip" style={{ right: terminal.leftSideWidth === 0 ? 60 : 360 }}>{intl.get('FILE_ZIP_DECOMPRESS')}</span>
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
