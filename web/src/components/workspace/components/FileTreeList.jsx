import React,{Fragment} from 'react'
import {CaretDownOutlined} from "@ant-design/icons"
import {Tooltip, Tree, Spin } from "antd"
import classNames from "classnames"
import intl from "react-intl-universal"
import Icons from "../../Icons/Icons"
import MySqlIcon from "../../../assets/logo/sql.svg"
import PostgreSqllIcon from "../svgicons/postgerSQL.svg"
import {cloudChild, objectStorageType} from "../FileTree"
import {useHotkeys} from "react-hotkeys-hook"

const { DirectoryTree } = Tree



function FileTreeList(props) {
  const {
    onLoadData,
    selectedKeys,
    expandedKeys,
    autoExpandParent,
    treeData,
    onExpand,
    onSelect,
    handleContextMenu,
    fileTreeDragAble,
    onDragStart,
    onDragOver,
    dropFileTree,
    searchValue,
    overkeys,
    onVisibleChange,
    reconnectionCloudDataBase,
    reconnectionDataBase,
    handlerCutFileKey,
    handlerCopyFileKey,
    handlerStickFileKey
  } = props

  const loop = (data) => {
    return data.map((item) => {
      const isObject = typeof item.title == "object"
      const index = isObject ? -1 : item.title.indexOf(searchValue)
      const beforeStr = isObject ? null : item.title.substr(0, index)
      const afterStr = isObject
        ? null
        : item.title.substr(index + searchValue.length)
      const showTitle = item.title
      let title =
        searchValue !== "" && index > -1 ? (
          <span id={item.key.replace(/\s*/g, "").replace(new RegExp("/","g"), '_')} className={classNames("filename" + item.key, item.fileType)}>
            {beforeStr}
            <span className="file-tree-search-value">{searchValue}</span>
            {afterStr}
          </span>
        ) : isObject ? (
          <span id={item.key.replace(/\s*/g, "").replace(new RegExp("/","g"), '_')} className={classNames("filename" + item.key, item.fileType)}>
            {showTitle}
          </span>
        ) : (
          <div style={{position: 'relative'}}>
            <Tooltip title={item.key} mouseEnterDelay={1.5} visible={ overkeys === item.key } onVisibleChange={()=>onVisibleChange(item.key, overkeys == item.key)} placement="topLeft" >
                    <span className={"title-container"}>
                    <span className={classNames("filename" + item.key, item.fileType)}>
                        {showTitle}
                        <span id={item.key.replace(/\s*/g, "").replace(new RegExp("/","g"), '_')} style={{ position: 'absolute', zIndex: 3, right: 0, opacity: 0 }}><Spin size="small" /></span>
                    </span>
                      {
                        (cloudChild(item.key) && !item.active)?
                          (
                            <span className={"data-source-type-title"}>
                              <span className="data-source-name">&nbsp;{objectStorageType(item.sourceType)}</span>
                              <Tooltip placement="top" title={intl.get("RECONNECT")}>
                              <Icons.BHDisconnectDatabase
                                onClick={(e) => {
                                  e.stopPropagation()
                                  reconnectionCloudDataBase(item)
                                }}
                              />
                              </Tooltip>
                            </span>) : (
                            <span className={"data-source-type-title"}>
                                <span className="data-source-name" style={{paddingRight: '20px'}}>&nbsp;{objectStorageType(item.sourceType)}</span>
                            </span>
                          )
                      }
                    </span>
            </Tooltip>
          </div>

        )
      let icon = null
      if (item.fileType === "database") {
        let dataSourceTypeTitle = ""
        switch (item.dataSourceType) {
          case "mysql":
            dataSourceTypeTitle = "MySQL"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={MySqlIcon}
                alt={"数据源图标"}
              />
            )
            break
          case "postgresql":
            dataSourceTypeTitle = "PostgreSQL"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={PostgreSqllIcon}
                alt={"数据源图标"}
              />
            )
        }

        title = (
          <div className={"title-container"}>
            <span>{title}</span>
            <span className={"data-source-type-title"}>
            <span className="data-source-name">{dataSourceTypeTitle}</span>
              {item.status === '0'?
                <span className="data-source-name" style={{paddingRight: '20px'}}></span> :
                (<Tooltip placement="top" title={intl.get('RECONNECT')}>
                  <Icons.BHDisconnectDatabase
                    onClick={(event) => {
                      event.stopPropagation()
                      reconnectionDataBase(item)
                    }}/>
                </Tooltip>)}
            </span>
          </div>
        )
      }
      // 数据源icon逻辑， 勿删
      // if(item.sourceType){
      //   switch(item.sourceType){
      //     case 'aliyun s3':
      //       icon = (
      //         <img
      //           src={aliyunImg}
      //           style={{ width: 15, height: 15 }}
      //           alt="阿里云OSS"/>
      //       )
      //       break
      //     case 'amazon s3':
      //       icon = (
      //         <img
      //           src={amazonImg}
      //           style={{ width: 15, height: 15 }}
      //           alt="Amazon s3"/>
      //       )
      //       break
      //     case 'ucloud s3':
      //       icon = (
      //         <img
      //           src={ucloudImg}
      //           style={{ width: 15, height: 15 }}
      //           alt="Ucloud S3"/>
      //       )
      //       break
      //     default:
      //       break
      //   }
      // }
      if (item.children) {
        return {
          title,
          key: item.key,
          name: item.title,
          isLeaf: item.isLeaf,
          fileType: item.fileType,
          tableName: item.tableName,
          parentKey: item.parentKey,
          children: loop(item.children),
          status:item.fileType==='database'?item.status:"",
          // S3 图标逻辑 勿删
          // icon: item.fileType === 'DIRECTORY' && item.key === '/storage-service'?
          // (<Icons.BHStorageServiceIcon/>) : icon
          icon
        }
      }

      return {
        title,
        key: item.key,
        name: item.title,
        isLeaf: item.isLeaf,
        fileType: item.fileType,
        tableName: item.tableName,
        parentKey: item.parentKey,
      }
    })
  }

  useHotkeys('ctrl+x',handlerCutFileKey)
  useHotkeys('ctrl+v',handlerStickFileKey)
  useHotkeys('ctrl+c',handlerCopyFileKey)

  return (
    <Fragment >
      <DirectoryTree
        loadData={onLoadData}
        showLine
        selectedKeys={selectedKeys}
        expandedKeys={expandedKeys}
        autoExpandParent={autoExpandParent}
        treeData={loop(treeData)}
        onExpand={onExpand}
        onSelect={onSelect}
        showIcon={false}
        switcherIcon={<CaretDownOutlined />}
        //onDoubleClick={dbClick}
        onRightClick={({ event, node }) => {
          handleContextMenu(event, node)
        }}
        draggable={fileTreeDragAble}
        blockNode
        onDragStart={(event)=>onDragStart(event)}
        onDragOver={()=> onDragOver()}
        onDrop={dropFileTree}
      />
    </Fragment>
  )
}

export default FileTreeList
