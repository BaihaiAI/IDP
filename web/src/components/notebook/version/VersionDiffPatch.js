import React,{ useContext, useEffect, useState, useRef, useMemo } from 'react'
import { Context } from './VersionPanel'
import { Button, Drawer, Radio, Tooltip, Modal, Form, Input, message } from 'antd'
import contentApi from '../../../services/contentApi'
import { useDispatch, useSelector } from "react-redux"
import { ExclamationCircleOutlined } from "@ant-design/icons"
import { setRefreshSource as refreshCode } from "../cells/codeCell/CodeCell"
import { setRefreshSource as refreshSql } from "../cells/sqlCell/SqlCell"
import { contentCatAsync } from '../../../store/features/notebookSlice'
import { selectUpdateList } from '../../../store/features/globalSlice'

import intl from "react-intl-universal"
import emptdiff from './emptdiff.png'
import './versiondiffpatch.less'
const { TextArea } = Input


function VersionDiffPatch(props){
  const { path, setShowVersionDrawer, isExecuting, showSaveVersion, setShowSaveVersion } = props;
  const [showVersionDrawer] = useContext(Context)
  const [snapshotId, setSnapshotId] = useState({
    left: {
      id: 0,
      index: 0,
    },
    right: {
      id: 0,
      index: 0
    },
  })
  const [check, setCheck] = useState({
    left: 0,
    right: 1,
  })
  const firstContent = useRef()
  const [hideUnchanged, setHideUnchanged] = useState(0)
  const [cellsLeft, setCellsLeft] = useState([]);
  const [cellsRight, setCellsRight] = useState([]);
  const [versionInfo, setVersionInfo] = useState("") 
  const [confirmLoading, setConfirmLoading] = useState(false) 
  const [snapshotList, setSnapshotList] = useState([])
  const [verticalForm] = Form.useForm()
  const updateList = useSelector(selectUpdateList)
  const dispatch = useDispatch()

  useEffect(() => {
    if(!showVersionDrawer) return
    getVersionList()
  }, [updateList])


  useEffect(() => {
    if(!showVersionDrawer) return
    getActivation()
  }, [snapshotList])


  const getVersionList = () => {
    contentApi
      .snapshotList({ path })
      .then(function (res) {
        const {snapshots} = res.data
        setSnapshotList(snapshots)
      })
      .catch(function (error) {
        Modal.error({
          content: error,
        })
      })
  }



  // 使用一个 老版本
  const checkVersion = (id) => {
    if (isExecuting) {
      Modal.confirm({
        title: `${intl.get(
          "RESTORING_A_HISTORICAL_SNAPSHOT_STOPS_CURRENTLY_RUNNING_TASKS"
        )}，${intl.get("CONFIRM_RECOVERY")}？`,
        icon: <ExclamationCircleOutlined />,
        content: "",
        onOk() {
          setVersion(id)
        },
        onCancel() {
          message.warning(intl.get("OPERATION_CANCELLED"))
        },
      })
    } else {
      setVersion(id)
    }
  }


  const setVersion = (id) => {
    setShowVersionDrawer(false)
    contentApi
      .snapshotRestore({
        id,
        path
      })
      .then(function (res) {
        console.log(res, '-------------')
        //更新工作区
        refreshCode(true)
        refreshSql(true)
        dispatch(contentCatAsync({path}))
          .then(err => {
            message.success(intl.get("HISTORICAL_SNAPSHOT_RESTORED"))
          })
        // dispatch(updateNotebookJson({ path, notebookJson: JSON.parse(res) }))
      })
      .catch(function (error) {
        Modal.error({
          content: error,
        })
      })
  }


  //保存新版本的输入框信息
  const versionInputChange = ({ target: { value } }) => {
    setVersionInfo(value)
  }


  const handleOk = () => {
    
    verticalForm.validateFields()
      .then(value => {
        setConfirmLoading(true)
        saveVersion()
      })
  }


  const handleCancel = () => {
    setShowSaveVersion(false)
    verticalForm.resetFields()
    setVersionInfo("")
  }


  // 保存新版本
  const saveVersion = () => {
    const params = {
      path,
      label: versionInfo,
    }
    contentApi
      .snapshot(params)
      .then(function (response) {
        setShowSaveVersion(false)
        setConfirmLoading(false)
        verticalForm.resetFields()
        setVersionInfo("")
      })
      .catch(function (err) {
        message.error(intl.get("SAVE_VERSION_FAILED"))
        setConfirmLoading(false)
      })
  }


  // 获取到 diff 对比内容
  const getCellContent = (snapshot) => {
    const { id: leftId } = snapshot.left;
    const { id: rightId } = snapshot.right;
    if(!leftId && !rightId) return false;
    contentApi.viewComparison({leftId, rightId, path})
      .then(res => {
        const { cells1, cells2} = res.data
        setCellsLeft(cells1);
        setCellsRight(cells2);
      })
      .catch(err => {
        console.log(err)
      })
  }


  // 选中要 diff 的 cell
  const getActivation = (e) => {
    if(snapshotList.length > 1){
      if(!e){
        setSnapshotId({
          left: {
            id: snapshotList[0].id,
            index: 0
          },
          right: {
            id: snapshotList[1].id,
            index: 1
          }
        })
        firstContent.current = {
          left: {
            id: snapshotList[0].id,
            index: 0
          },
          right: {
            id: snapshotList[1].id,
            index: 1
          }
        }
        setCheck({
          left: 0,
          right: 1
        })
        getCellContent(firstContent.current)
      }else{
        if(!e.target.defaultValue) return false;
        let temporaryId;
        if(e.target.name==='left'){
          setSnapshotId({
            left: {
              id: snapshotList?.[parseInt(e.target.defaultValue)]?.['id'],
              index: parseInt(e.target.defaultValue)
            },
            right: {
              id: snapshotId['right']['id'],
              index: snapshotId['right']['index']
            },
          })
          temporaryId = {
            left: {
              id: snapshotList?.[parseInt(e.target.defaultValue)]?.['id']
            },
            right: {
              id: snapshotId['right']['id'],
            }
          }
          setCheck({
            left: parseInt(e.target.defaultValue),
            right: parseInt(snapshotId['right']['index'])
          })
        }else if(e.target.name === 'right'){
          setSnapshotId({
            left: {
              id: snapshotId['left']['id'],
              index: snapshotId['left']['index']
            },
            right: {
              id: snapshotList?.[parseInt(e.target.defaultValue)]?.['id'],
              index: parseInt(e.target.defaultValue)
            }
          })
          temporaryId = {
            left: {
              id: snapshotId['left']['id'],
            },
            right: {
              id: snapshotList?.[parseInt(e.target.defaultValue)]?.['id'],
            }
          }
          setCheck({
            left: parseInt(snapshotId['left']['index']),
            right: parseInt(e.target.defaultValue)
          })
        }
        getCellContent(temporaryId)
      }
    }
  }


  const timestampHandling = (time) => {
    return time.slice(0, 19);
  }


  const leftCalculateHeight = (high, index, type) => {
    const corrCell = cellsRight.filter((key, i) => i === index)[0]
    const corrH = corrCell?.data?.lines?.length;
    if(type==="Code"){
      if(corrCell?.cellType === "Empty"){
        return `${high * 22 + 81}px`
      }else{
        if(high > corrH || high === corrH){
          return `${high * 22 + 81}px`
        }else if(high < corrH){
          return `${corrH * 22 + 81}px`
        }
      }
    }
    if(type==="Vis"){
      return `${34 * 7 + 81}px`
    }
    if(type === "Markdown"){
      if(corrCell?.cellType === "Empty"){
        return `${high * 22 + 81}px`
      }else{
        if(high > corrH || high === corrH){
          return `${high * 22 + 81}px`
        }else if(high < corrH){
          return `${corrH * 22 + 81}px`
        }
      }
    }
    if(type === "Sql"){
      if(corrCell?.cellType === "Empty"){
        return `${high * 22 + 115}px`
      }else{
        if(high > corrH || high === corrH){
          return `${high * 22 + 115}px`
        }else if(high < corrH){
          return `${corrH * 22 + 115}px`
        }
      }
    }
    if(type === "Empty"){
      switch(corrCell?.cellType){
        case "Code":
          return  `${corrH * 22 + 81}px`
        case "Vis":
          return `${34 * 7 + 81}px`
        case "Markdown":
          return  `${corrH * 22 + 81}px`
        case "Sql":
          return `${corrH * 22 + 115}px`
      }
    }
  }


  const rightCalculateHeight = (high, index, type) => {
    const corrCell = cellsLeft.filter((key, i) => i === index)[0]
    const corrH = corrCell?.data?.lines?.length;
    if(type==="Code"){
      if(corrCell?.cellType === "Empty"){
        return `${high * 22 + 81}px`
      }else{
        if(high > corrH || high === corrH){
          return `${high * 22 + 81}px`
        }else if(high < corrH){
          return `${corrH * 22 + 81}px`
        }
      }
    }
    if(type==="Vis"){
      return `${34 * 7 + 81}px`
    }
    if(type === "Markdown"){
      if(corrCell?.cellType === "Empty"){
        return `${high * 22 + 81}px`
      }else{
        if(high > corrH || high === corrH){
          return `${high * 22 + 81}px`
        }else if(high < corrH){
          return `${corrH * 22 + 81}px`
        }
      }
    }
    if(type === "Sql"){
      if(corrCell?.cellType === "Empty"){
        return `${high * 22 + 115}px`
      }else{
        const corrH = corrCell?.data?.lines?.length;
        if(high > corrH || high === corrH){
          return `${high * 22 + 115}px`
        }else if(high < corrH){
          return `${corrH * 22 + 115}px`
        }
      }
    }
    if(type === "Empty"){
      switch(corrCell?.cellType){
        case "Code":
          return  `${corrH * 22 + 81}px`
        case "Vis":
          return `${34 * 7 + 81}px`
        case "Markdown":
          return  `${corrH * 22 + 81}px`
        case "Sql":
          return `${corrH * 22 + 115}px`
      }
    }
  }


  // left   
  const letfDom = (
    <div className='deff-cell-left'>
      {cellsLeft.map((prop, index) => (
        <div className='puppet' key={index}  
          style={{height: `${leftCalculateHeight(prop?.data?.lines?.length, index, prop.cellType)}`}}>
          {
            prop?.same && hideUnchanged? null : (
              <div 
                className={
                  cellsRight[index]?.["cellType"]==="Empty"?
                  "cells-block new-cell" :
                  (prop.cellType === "Empty")?
                  "cells-block hide": "cells-block"}
              >
                {prop.cellType === "Empty" || prop.idx === null? (
                  <div className='cell-empty'></div>
                ): (
                  <div className='cells-black-warp'>
                    <div className='cell-title'>{prop.cellType}</div>
                    <>
                      {
                        prop.cellType === "Code" || prop.cellType === "Markdown"? (
                          <>
                            {prop.data.lines?.map((row, i) => (
                              <div className='cell-row' key={i}>
                                <p className={row.colored? "new-cell" : null}>
                                  {row.content}
                                  <span>{row.idx}</span>
                                </p>
                              </div>
                            ))}
                          </>
                        ) : prop.cellType === "Sql"? (
                          <div>
                            <div 
                              // className="cell-sql"
                              className={
                                cellsRight[index]?.["cellType"]==="Empty"?
                                "cell-sql new-cell" : (
                                  prop.data.data_source[0] || prop.data.df_name[0]? "cell-sql new-cell" : "cell-sql"
                                )}
                              >
                              <p className="p1">{intl.get('NOTEBOOK_SQL_DATASOURCE')} {prop.data.data_source[1]}</p>
                              <p className="p2">{intl.get('NOTEBOOK_SQL_VARIABLE')} {prop.data.df_name[1]}</p>
                            </div>
                            {prop.data.lines?.map((row, i) => (
                              <div className='cell-row' key={i}>
                                <p className={row.colored? "new-cell" : null}>
                                  {row.content}
                                  <span>{row.idx}</span>
                                </p>
                              </div>
                            ))}
                          </div>
                        ) : (
                          <>
                            {
                              Reflect.ownKeys(prop.data).map((item, i) => (
                                <div key={i}>
                                  {item.startsWith('i')? null : (
                                    <div 
                                      className={
                                        cellsRight[index]?.["cellType"]==="Empty"?
                                        "cell-vis new-cell" : 
                                        prop.data[item][0]? "cell-vis new-cell" : "cell-vis"}
                                    >
                                      <>
                                        <p className="p1">{item}</p>
                                        <p className="p2">{prop.data[item]}</p>
                                      </>
                                    </div>
                                  )}
                                </div>
                              ))
                            }
                          </>
                        )
                      }
                    </>
                  </div>
                )}
              </div>
            )
          }
        </div>
      ))}
    </div>
  )


  
  const rightDom = (
    <dir className='deff-cell-right'>
      {cellsRight.map((prop, index) => (
        <div className='puppet' key={index}
          style={{height: `${rightCalculateHeight(prop?.data?.lines?.length, index, prop.cellType)}`}}>
          {prop?.same && hideUnchanged? null : (
            <div 
              className={
                cellsLeft[index]?.["cellType"]==="Empty"?
                "cells-block new-cell" :
                (prop.cellType === "Empty")?
                "cells-block hide": "cells-block"}>
              {prop.cellType === "Empty" || prop.idx === null? (
                <div className='cell-empty'></div>
              ): (
                <div className='cells-black-warp'>
                  <div className='cell-title'>
                    {/* {cellsLeft[index]["cellType"]}    */}
                    {prop.cellType}
                  </div>
                  <>
                    {
                      prop.cellType === "Code" || prop.cellType === "Markdown"? (
                        <>
                          {prop.data.lines?.map((row, i) => (
                            <div className='cell-row' key={i}>
                              <p className={row.colored? "new-cell" : null}>
                                {row.content}
                                <span>{row.idx}</span>
                              </p>
                            </div>
                          ))}
                        </>
                      ) : prop.cellType === "Sql"? (
                        <div>
                          <div
                            // className="cell-sql"
                            className={
                              cellsLeft[index]?.["cellType"]==="Empty"?
                              "cell-sql new-cell" : (
                                prop.data.data_source[0] || prop.data.df_name[0]? "cell-sql new-cell" : "cell-sql"
                              )}
                            >
                            <p className="p1">{intl.get('NOTEBOOK_SQL_DATASOURCE')} {prop.data.data_source[1]}</p>
                            <p className="p2">{intl.get('NOTEBOOK_SQL_VARIABLE')} {prop.data.df_name[1]}</p>
                          </div>
                          {prop.data.lines?.map((row, i) => (
                            <div className='cell-row' key={i}>
                              <p className={row.colored? "new-cell" : null}>
                                {row.content}
                                <span>{row.idx}</span>
                              </p>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <>
                          {
                            Reflect.ownKeys(prop.data).map((item, i) => (
                              <div key={i}>
                              {item.startsWith('i')? null : (
                                <div 
                                  className={
                                    cellsLeft?.[index]?.["cellType"]==="Empty"?
                                    "cell-vis new-cell" : 
                                    prop.data[item][0]? "cell-vis new-cell" : "cell-vis"}
                                >
                                  <>
                                    <p className="p1">{item}</p>
                                    <p className="p2">{prop.data[item]}</p>
                                  </>
                                </div>
                              )}
                              </div>
                            ))
                          }
                        </>
                      )
                    }
                  </>
                </div>
              )}
            </div>
          )}
        </div>
      ))}
    </dir>
  )

  
  // console.log(cellsLeft, cellsRight, "-------------")
  
  return (
    <div>
      <Drawer
        placement="right"
        title={<div onClick={() =>{
          setShowVersionDrawer(false)
        }}>
          <span>{'←'}&nbsp;&nbsp;&nbsp;&nbsp;</span>{intl.get('SNAPSHOT_REVISION')}
        </div>}
        visible={showVersionDrawer}
        className="VersionDiffPatch"
        closable={false}
        destroyOnClose={true}
      >
        {snapshotList.length > 1? (
          <div className='diff-patch'>
            <div className='hideUn'>
              <Radio.Group defaultValue={hideUnchanged} value={hideUnchanged}>
                <Radio 
                  value={1}
                  onClick={() => {
                    if(hideUnchanged)setHideUnchanged(0);
                    else setHideUnchanged(1);
                  }}
                >{intl.get('SNAPSHOT_HIDE_CONTENT')}</Radio>
              </Radio.Group>
            </div>
            <div className='diff-cells'>
              {letfDom}
              {rightDom}
            </div>
          </div>
        ) : (
          <div className='diff-patch no-diff'>
            <div>
              <img src={emptdiff} />
                <p>{intl.get('SNAPSHOT_NO')}</p>
            </div>
          </div>
        )}
        
        <div className='right-sider'>
          <div className='snapshot-list'>
            {snapshotList.length <=1? null : (
              <div className='snapshot-list-select' onClick={getActivation}>
                <Radio.Group name="left" className="left" value={check.left}>
                  {snapshotList?.map((item, i) => (
                    <Radio 
                      value={i} 
                      buttonStyle="solid" 
                      disabled={snapshotId.right.index === i? true : false}
                      key={item.id}></Radio>
                  ))}
                </Radio.Group>
                <Radio.Group name="right" className="right" value={check.right}>
                  {snapshotList?.map((item, i) => (
                    <Radio 
                      value={i} 
                      buttonStyle="solid" 
                      disabled={snapshotId.left.index === i? true : false}
                      key={item.id}></Radio>
                  ))}
                </Radio.Group>
              </div>
            )}
            <div className='snapshot-list-warper'>
              {snapshotList?.map((item, i) => (
                <div key={item.id} className="snapshot-puppet">
                  <div className="version-list-body">
                    <div className="version-list-item">
                      {item.label?.length > 15? (
                        <Tooltip 
                          placement="topLeft" 
                          title={item.label}
                          mouseEnterDelay={1}>
                          <span>{item.label}</span>
                        </Tooltip>
                        
                      ) : (
                        <span>{item.label}</span>
                      )}
                      
                      <Button
                        type="text"
                        id={item.timestamp}
                        value={item.timestamp}
                        style={{ color: "#1890ff" }}
                        onClick={() => checkVersion(item.id)}
                      >
                        {intl.get("RECOVER")}
                      </Button>
                    </div>
                    <div className="revert-timestamp">{timestampHandling(item.time)}</div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </Drawer>


      <Modal
        title={intl.get("ADD_SNAPSHOT_INFORMATION")}
        visible={showSaveVersion}
        onOk={handleOk}
        onCancel={handleCancel}
        confirmLoading={confirmLoading}
        destroyOnClose={true}
      >
        <p className="modal-sub-title">{intl.get("NAME_THIS_SNAPSHOT")}</p>
        <Form layout="vertical" name="versionForm" form={verticalForm}>
          <Form.Item
            name="version"
            rules={[
              {
                required: true,
                message: `${intl.get("INPUT_CAN_NOT_BE_EMPTY")}!`,
              }
            ]}
          >
            <TextArea
              value={versionInfo}
              showCount
              maxLength={50}
              placeholder={intl.get("PLEASE_ENTER_A_SNAPSHOT_NAME")}
              onChange={versionInputChange}
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>  
  )
}

export default VersionDiffPatch
