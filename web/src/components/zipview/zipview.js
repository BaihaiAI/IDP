import React, { useEffect, useState } from "react";
import contentApi from '../../services/contentApi'
import { Button, Tree, Spin } from "antd";
import './zipview.less'

const { DirectoryTree } = Tree;


function ZipView(props){
  const { path, name,content } = props.item;
  const [list, setList] = useState(content)
  const [expandNode, setExpandNode] = useState([])

  useEffect(() => {
    if(list?.length !== 0){
      setExpandNode(list[0]?.absolutePath)
    }
  }, [list])

  function loop(data){
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


  return (
    <div className="zipview"
         style={{
           height: (document.body.clientHeight - 93),
         }}>
      <div className="zipview-header">
        <span className="zh-span">预览 {name}</span></div>
      <div className="zipview-content">
        {list?.length !== 0 && expandNode?.length !== 0 ?(
          <DirectoryTree
            showIcon={false}
            treeData={loop(list)}
            autoExpandParent={false}
            defaultExpandedKeys={[expandNode]} // expandNode
          />
        ): <div className="zipview-spin"><Spin size="large"/></div>}

      </div>
    </div>
  )
}

export default React.forwardRef(ZipView)
