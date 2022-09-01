import { useState, useEffect } from 'react';
import { Layout, Input, Tree, Tooltip, message } from 'antd'
import { useDispatch, useSelector } from 'react-redux';
import { changeOperatorDecision,changeOperatorKey } from 'idpStore/features/globalSlice'
import { InsertCodeSnippet } from 'idpStore/features/notebookSlice'
import { selectActivePath } from 'idpStore/features/filesTabSlice'
import operatorApi from 'idpServices/operatorApi';
import { store } from '../../../../store';
import { useGetState, useDebounceEffect } from 'ahooks';
import {SearchOutlined,PlusOutlined } from "@ant-design/icons"

import './index.less';

import { OperatorFocus, OperatorBlur } from 'idpStudio/components/workspace/keymap'

import { LspWebsocket } from "idpStudio/components/notebook/lib/LspWebsocket"

const { Sider } = Layout;
const { DirectoryTree } = Tree

function OperatorPanel(props) {
    const dispatch = useDispatch()
    const path = useSelector(selectActivePath)
    const [searchValue, setSearchValue] = useState("")
    const [treeData, setTreeData] = useState([])
    const [selectedKeys, setSelectedKeys, getSelectedKeys] = useGetState([])
    const [expandedKeys, setExpandedKeys] = useState([])


    const [treeAllKeys, setTreeAllKeys] = useState([])
    const [itemAll, setItemAll] = useState([])

    useEffect(() => {
        if (!searchValue) {
            operatorApi.getOperatorList({ keyword: "" })
                .then(res => {
                    const { children } = res.data;
                    let keyMapArr = []
                    let itemArr = []
                    recursiveValue(children, keyMapArr) // 递归拿到所有 key
                    setTreeData(children)
                    setTreeAllKeys(keyMapArr)

                    getItem(children, itemArr)
                    setItemAll(itemArr)
                    setExpandedKeys([])
                })
        }
    }, [searchValue])


    const onChange = () => {
        if (searchValue === "") return
        operatorApi.getOperatorList({ keyword: searchValue })
            .then(res => {
                const { children } = res.data;
                let keyMapArr = []
                let itemArr = []
                recursiveValue(children, keyMapArr) // 递归拿到所有 key
                setTreeData(children)
                setTreeAllKeys(keyMapArr)
                getItem(children, itemArr)
                setItemAll(itemArr)

                setExpandedKeys(keyMapArr)
            })
    }
    useDebounceEffect(
        () => {
            onChange()
        },
        [searchValue],
        { wait: 500 }
    )

    function getPreAndNextIndex() {
        let currentIndex = 0
        let preIndex = 0
        let nextIndex = 0

        for (let val of Object.values(store.getState().notebook.notebookList)) {
            if (val.path === path) {
                if (val.cells.length === 0) {
                    preIndex = 1
                    currentIndex = 0
                } else {
                    let key;
                    for (let prop of Object.keys(val.cellProps)) {
                        if (val.cellProps[prop]["focus"]) {
                            key = prop
                        }
                    }
                    for (let i = 0; i < val.cells.length; i++) {
                        if (val.cells[i].metadata.id === key) {
                            if (val.cells[i] === val.cells[val.cells.length - 1]) {
                                preIndex = val.cells[i].metadata.index
                                nextIndex = ((val.cells[i].metadata.index + 1) * 2) - val.cells[i].metadata.index
                                currentIndex = i;
                            } else {
                                preIndex = val.cells[i].metadata.index
                                nextIndex = val.cells[i + 1].metadata.index
                                currentIndex = i
                            }
                        }
                    }
                }
                break
            }
        }
        return { preIndex, nextIndex, currentIndex }
    }

    function insertCodes(recover) {
        const { preIndex, nextIndex, currentIndex } = getPreAndNextIndex()
        let cells = [], starindex = preIndex;
        for (let i = 0; i < recover.length; i++) {
            let cell = {
                cell_type: 'code',
                metadata: {
                    // index: preIndex + ((preIndex + nextIndex) / (100 - i))
                    index: (starindex + nextIndex) / 2
                },
                source: recover[i],
                path
            }
            cells.push(cell)
            starindex = (starindex + nextIndex) / 2;
        }
        dispatch(InsertCodeSnippet({ path, cells, currentIndex })).unwrap().then((res) => {
            message.success("代码片段添加成功")
            const { data, path } = res
            let lspWebsocket = LspWebsocket.instance
            if (!lspWebsocket || !lspWebsocket.didOpenFile[path]) return

            for (const cell of data) {
                const source = cell['source']
                let text = '';
                for (let i = 0; i < source.length; i++) {
                    text = `${text}${source[i]}`;
                }
                if (text.trim().startsWith('!')) continue
                lspWebsocket.didChange(
                    path,
                    cell['metadata']['id'],
                    { from: { line: 0, ch: 0 }, to: { line: 0, ch: 0 } },
                    text
                );
            }
        })
    }

    const addCodeSnippet = async (e, item) => {
        e.stopPropagation()
        const { key } = item;
        const { cells: recover } = await operatorApi.getOperatorCode({ key })
            .then(res => {
                return res.data
            })
        insertCodes(recover)
    }
    // 选中后 插入代码
    // const keyMapAddCodeSnippet = async (item) => {
    //   const { key } = item;
    //   const { cells: recover } = await operatorApi.getOperatorCode({key})
    //     .then(res => {
    //       return res.data
    //     })
    //   insertCodes(recover)
    // }

    const recursiveValue = (data, keyMapArr) => {
        const type = Object.prototype.toString.call(data);
        if (type === "[object Array]") {
            data.forEach(item => {
                keyMapArr.push(item.key)
                if (!item.isLeaf) {
                    recursiveValue(item.children, keyMapArr)
                }
            })
        }
    }
    const getItem = (data, itemArr) => {
        const type = Object.prototype.toString.call(data);
        if (type === "[object Array]") {
            data.forEach(item => {
                itemArr.push(item)
                if (!item.isLeaf) {
                    getItem(item.children, itemArr)
                }
            })
        }
    }

    const loop = (data) => {
        return data.map((item) => {
            const index = item.title.indexOf(searchValue)
            const beforeStr = item.title.substr(0, index)
            const afterStr = item.title.substr(index + searchValue.length)
            let title = searchValue !== "" && index > -1 ? (
                <div className={'add-title'}>
                    <span className="ant-tree-title">
                        {beforeStr}
                        <span className='add-search-value'>{searchValue}</span>
                        {afterStr}
                    </span>
                    <Tooltip placement="topRight" title={"在Notebook中插入该代码片段"}>
                        <span className="add-operator" onClick={(e) => addCodeSnippet(e, item)}><PlusOutlined /></span>
                    </Tooltip>
                </div>
            ) : item.isLeaf ? (
                <div className={'add-title'}>
                    <span className="ant-tree-title">{item.title}</span>
                    <Tooltip placement="topRight" title={"在Notebook中插入该代码片段"}>
                        <span className="add-operator" onClick={(e) => addCodeSnippet(e, item)}><PlusOutlined /></span>
                    </Tooltip>
                </div>
            ) : (
                <div className={'add-title'}>
                    <span className="ant-tree-title">{item.title}</span>
                </div>
            )

            if (item.children) {
                return {
                    title,
                    key: item.key,
                    children: loop(item.children),
                }
            }
            return {
                title,
                key: item.key,
            }
        })
    }

    const onSelect = (keys, info) => {
        console.log('-----------------', info)
        setSelectedKeys(keys);
        if (!info.node?.children.length) {
            dispatch(changeOperatorDecision(true))
            dispatch(changeOperatorKey(info.node.key))
        }
    };

    const onExpand = (keys, info) => {
        setExpandedKeys(keys)
    };


    return (
        <Sider
            theme="light"
            width="300"
            style={{ height: document.body.clientHeight - 40 }}
        >
            <div className='operator-warp'>
                <div className='operator-header'>公共代码片段</div>
                <div className="operator-input">
                    <Input
                        value={searchValue}
                        placeholder={"过滤代码片段"}
                        suffix={<SearchOutlined />}
                        onChange={(event) => setSearchValue(event.target.value)}
                    />
                </div>
                <div
                    className='operator-tree'
                    tabIndex="3"
                    onFocus={() => OperatorFocus({
                        selectDown() {
                            let selectedkey = getSelectedKeys()

                            if (selectedkey.length === 0) {
                                setSelectedKeys([treeAllKeys[0]])
                            } else {
                                let key = selectedkey[0];
                                let keyLen = treeAllKeys.indexOf(key)

                                if (keyLen + 1 === treeAllKeys.length) {
                                    setSelectedKeys([treeAllKeys[0]])
                                } else {
                                    setSelectedKeys([treeAllKeys[keyLen + 1]])
                                }
                                // 详情逻辑
                                for (let prop of itemAll) {
                                    if (prop.key === treeAllKeys[keyLen + 1]) {
                                        if (prop.isLeaf) {
                                            dispatch(changeOperatorDecision(true))
                                            dispatch(changeOperatorKey(treeAllKeys[keyLen + 1]))
                                        } else {
                                            dispatch(changeOperatorDecision(false))
                                        }
                                        setExpandedKeys(expandedKeys => [...expandedKeys, ...prop.path])
                                    } else if (prop.key === treeAllKeys[0]) {
                                        dispatch(changeOperatorDecision(false))
                                    }
                                }
                            }
                        },
                        selectUp() {
                            let selectedkey = getSelectedKeys()
                            if (selectedkey.length === 0) {
                                setSelectedKeys([treeAllKeys[0]])
                            } else {
                                let key = selectedkey[0];
                                let keyLen = treeAllKeys.indexOf(key)

                                if (keyLen === 0) {
                                    setSelectedKeys([treeAllKeys[treeAllKeys.length - 1]])
                                } else {
                                    setSelectedKeys([treeAllKeys[keyLen - 1]])
                                }
                                // 详情逻辑
                                for (let prop of itemAll) {
                                    if (prop.key === treeAllKeys[keyLen - 1]) {
                                        if (prop.isLeaf) {
                                            dispatch(changeOperatorDecision(true))
                                            dispatch(changeOperatorKey(treeAllKeys[keyLen - 1]))
                                        } else {
                                            dispatch(changeOperatorDecision(false))
                                        }
                                        setExpandedKeys(expandedKeys => [...expandedKeys, ...prop.path])
                                        break
                                    } else if (prop.key === treeAllKeys[treeAllKeys.length - 1]) {
                                        if (prop.isLeaf) {
                                            dispatch(changeOperatorDecision(true))
                                            dispatch(changeOperatorKey(treeAllKeys[treeAllKeys.length - 1]))
                                        } else {
                                            dispatch(changeOperatorDecision(false))
                                        }
                                        setExpandedKeys(expandedKeys => [...expandedKeys, ...prop.path])
                                        break
                                    }
                                }
                            }
                        }
                    })}
                    onBlur={() => OperatorBlur()}>
                    <DirectoryTree
                        onSelect={onSelect}
                        onExpand={onExpand}
                        treeData={loop(treeData)}
                        selectedKeys={selectedKeys}
                        showIcon={false}
                        expandedKeys={expandedKeys}
                    // defaultExpandParent={true}
                    />
                </div>
            </div>
        </Sider>
    )
}

export default OperatorPanel;
