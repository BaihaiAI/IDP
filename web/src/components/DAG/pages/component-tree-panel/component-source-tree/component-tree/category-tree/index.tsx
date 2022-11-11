import React, { useCallback, useEffect, useMemo, useRef, useState } from "react"
import { Tree } from "antd"
import { FolderFilled, FolderOpenFilled } from "@ant-design/icons"
import { NodeTitle } from "./node-title"
import styles from "./index.module.less"
import workspaceApi from "idpServices/workspaceApi"
import PubSub from "pubsub-js"
import {mergeArray} from "idpUtils/index"
import {useDidRecover} from "react-router-cache-route"

const { DirectoryTree, TreeNode } = Tree

const FolderIcon = ({ expanded }: { expanded: boolean }) => {
  return expanded ? <FolderOpenFilled /> : <FolderFilled />
}


function createExpandedKeyArr(string) {
  const arr = []
  const stringArr = string.split("/")
  stringArr.pop()
  while (stringArr.length > 0) {
    const str = stringArr.join("/")
    if (str) {
      arr.push(str)
    }
    stringArr.pop()
  }
  return arr
}

const updateTreeData = (list, key, children) => {
  return list.map((node) => {
    if (node.browserPath === key) {
      return { ...node, children }
    }
    if (node.children) {
      return {
        ...node,
        children: updateTreeData(node.children, key, children),
      }
    }
    return node
  })
}



export const CategoryTree = () => {
  const [treeList, setTreeList] = useState([])
  const [searchValue, setSearchValue] = useState("")
  const [expandedKeys, setExpandedKeys] = useState([])
  const loadNodeListRef = useRef(['/'])

  useEffect(() => {
    const subscriber = PubSub.subscribe(
      "category-tree-search-value-change",
      (msg, value) => {
        setSearchValue(value)
      }
    )
    const subscriber2 = PubSub.subscribe(('refresh-category-tree'),()=>{
      loadFileList()
    })
    return () => {
      PubSub.unsubscribe(subscriber)
      PubSub.unsubscribe(subscriber2)
    }
  }, [])

  useEffect(() => {
    handleExpandedKeys()
  }, [searchValue])

  const handleExpandedKeys = () => {
    const value = searchValue
    if (value !== "") {
      const expandedKeys = dataList
        .map((item) => {
          if (item.title.indexOf(value) > -1) {
            return getParentKey(item.key, treeList)
          }
          return null
        })
        .filter((item, i, self) => item && self.indexOf(item) === i)

      let realExpandedKeys = []
       expandedKeys.forEach(expandedKey=>{
         const arr = createExpandedKeyArr(expandedKey)
         arr.push(expandedKey)
         realExpandedKeys = mergeArray(realExpandedKeys,arr)
       })
      setExpandedKeys(realExpandedKeys)
    } else {
      setExpandedKeys([])
    }
  }

  const onExpand = (expandedKeys, { expanded: bool, node }) => {
    setExpandedKeys(expandedKeys)
  }

  const generateList = (data, dataList) => {
    for (let i = 0; i < data.length; i++) {
      const node = data[i]
      const newNode = {
        key: node.browserPath,
        title: node.fileName,
      }
      if (node.children) {
        generateList(node.children, dataList)
      }
      dataList.push(newNode)
    }
  }
  const dataList = useMemo(() => {
    const arr = []
    generateList(treeList, arr)
    return arr
  }, [treeList])

  const getParentKey = (key, tree) => {
    let parentKey
    for (let i = 0; i < tree.length; i++) {
      const node = tree[i]
      if (node.children) {
        if (node.children.some((item) => item.browserPath === key)) {
          parentKey = node.browserPath
        } else {
          const result = getParentKey(key, node.children)
          if (result) {
            parentKey = result
          }
        }
      }
    }
    return parentKey
  }


  const onLoadData = (node)=>{
    return new Promise<void>((resolve) => {
      const path = [node.key]
      if (loadNodeListRef.current.indexOf(node.key) === -1) {
        loadNodeListRef.current.push(node.key)
      }
      workspaceApi
        .lazyLoadDirBrowse({ path })
        .then((res) => {

          setTreeList((oldTreeList)=>{
            const newTreeData = updateTreeData(
              oldTreeList,
              node.key,
              res.data.children
            )
            return newTreeData
          })
          resolve()
        })
        .catch(() => {
          resolve()
        })
    })
  }

  const loadFileList = ()=>{
    setTreeList([])
    const path = loadNodeListRef.current
    workspaceApi.lazyLoadDirBrowse({
      path,onlyPipelineSupport:true
    }).then(function (response) {
      setTreeList(response.data.children)
    })
  }

  useEffect(() => {
    loadFileList()
  }, [])

  useDidRecover(()=>{
    loadFileList()
  })

  const renderTree = useCallback(
    (treeList: any[] = [], searchKey: string = "") => {
      return treeList.map((item) => {
        const { fileType, browserPath, children } = item
        const key = browserPath.toString()
        const node = {
          id: item.browserPath,
          defSource: 2,
          docUrl: "#" + item.browserPath,
          ioType: 0,
          up: 148,
          down: 11,
          iconType: 1,
          isDisabled: false,
          author: item.author,
          codeName: item.codeName,
          catId: item.catId,
          lastModifyTime: "2020-08-25 15:43:39",
          createdTime: "2015-04-16 13:38:11",
          engineType: item.engineType,
          isComposite: false,
          sequence: 0,
          owner: item.owner,
          description: item.absolutePath,
          name: item.fileName,
          parentId: item.parentId,
          isBranch: false,
          isDir: item.fileType === "DIRECTORY",
          social: {
            defSource: 2,
            isEnabled: true,
            docUrl: "#" + item.browserPath,
            iconType: 1,
            isDisabled: false,
            author: item.author,
            codeName: item.codeName,
            catId: item.catId,
            lastModifyTime: "2020-08-25 15:43:39",
            createdTime: "2015-04-16 13:38:11",
            owner: item.owner,
            description: item.absolutePath,
            name: item.fileName,
            id: item.browserPath,
          },
        }

        const title = <NodeTitle node={node} searchKey={searchKey} />

        if (fileType === "DIRECTORY") {
          return (
            <TreeNode
              icon={FolderIcon}
              key={key}
              title={title}
              className={styles.treeFolder}
            >
              {renderTree(children, searchKey)}
            </TreeNode>
          )
        }

        return (
          <TreeNode
            isLeaf={true}
            key={key}
            icon={<span />}
            title={title}
            className={styles.treeNode}
          />
        )
      })
    },
    []
  )

  // const treeList = componentTreeNodes.filter((node) => node.status !== 4)
  return (
    <div className={styles.list}>
      <DirectoryTree
        loadData={onLoadData}
        showIcon={true}
        selectable={false}
        autoExpandParent={false}
        className={styles.tree}
        expandedKeys={expandedKeys}
        onExpand={onExpand}
      >
        {/*需要searchKey*/}
        {renderTree(treeList, searchValue)}
      </DirectoryTree>
    </div>
  )
}
