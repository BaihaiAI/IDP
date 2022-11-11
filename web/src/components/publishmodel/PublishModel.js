import React,{ useEffect, useState, useRef } from 'react'
import { Modal, Button, Form, Input, AutoComplete, message, Upload, Progress } from "antd"
import { UploadOutlined } from '@ant-design/icons'
import { useDebounceEffect } from 'ahooks'
import warenhouseApi from 'idpServices/warenhouseApi'
import { projectId, teamId, userId } from '../../store/cookie';


import './pm.less'
let cancelUploadFils = false
function PublishModel(props){

  const {PublishModelVisible, change, rightKey, whetherUpload, getList} = props
  const [publishFrom] = Form.useForm();

  // 模型列表
  const [versionList, setVersionList] = useState([])

  // 模型名称 版本名称
  const [inputValue, setInputValue] = useState("");
  const [autoCompleteValue, setAutoCompleteValue] = useState("")

  //  版本是否可选中   版本不填警告提醒  加载loding&&进度条显示  进度条进度百分比
  const [versionDisabled, setVersionDisabled] = useState(true)
  const [verification, setVerification] = useState(false)
  const [enterLoading, setEnterLoading] = useState(false)
  const [schedule, setSchedule] = useState(0);
  // 上传文件
  const [fileList, setFileList] = useState([])
  // 取消上床Id 取消上传
  const [cancelId, setCancelId] = useState(0)


  useEffect(() => {
    setVersionDisabled(true)
  }, [inputValue])

  const writVersion = (e) => {
    if(!inputValue) return;
    warenhouseApi.checkVersion({modelName: inputValue})
      .then(res => {
        const {data} = res
        setVersionList(data)
        setVersionDisabled(false)
      })
  }

  useDebounceEffect(
    () => {writVersion()},
    [inputValue],
    {wait: 500}
  )

  const calculate = (num) => {
    return 100
  }
  let frequency;
  const confirmRelease = () => {
    if(!autoCompleteValue){
      setVerification(true)
    }else{
      setVerification(false)
    }
    publishFrom.validateFields()
      .then(async res => {
        if(verification) return;
        if(!autoCompleteValue) return
        if(whetherUpload){
          setEnterLoading(true)
          const originFileObj = fileList[0].originFileObj;

          const size = originFileObj.size //总大小
          const shardSize = 2 * 1024 * 1024 //以2MB为一个分片
          let shardCount = Math.ceil(size / shardSize) //总片数
          shardCount = shardCount === 0 ? 1 : shardCount
          let num = calculate(shardCount);
          frequency = 1;
          for (let i = 0; i < shardCount; ++i) {
            if(!cancelUploadFils){
              await distributeSlices(i, shardSize, originFileObj, res, shardCount, num, size)
            }else{
              console.log("取消了哈哈")
              cancelUploadFils=false
              return
            }
          }
        }else{
          warenhouseApi.uploadClient({
            path: rightKey,
            modelName: res.username,
            version: autoCompleteValue,
            intro: !res.Introduction? "": res.Introduction
          }).then(response => {
            message.success("发布成功")
            cancel()
          })
        }
      })

  }

  const distributeSlices = (i, shardSize, originFileObj, res, shardCount, num, size) => {
    return new Promise((resolve) => {
      let start = i * shardSize
      let end = Math.min(size, start + shardSize)
      // 再此构造表单，而非，请求函数中
      const formData = new FormData()
      formData.append('name', originFileObj.name)
      formData.append("modelName", res.username)
      formData.append("version", autoCompleteValue)
      formData.append("intro", !res.Introduction? "": res.Introduction)
      formData.append("datafile", originFileObj.slice(start, end))
      formData.append("index", i + 1)
      formData.append("total", shardCount)
      return warenhouseApi.uploadFile(formData)
        .then(response => {
          let speed = shardCount===1? '100.00' : ((frequency / shardCount) * num).toFixed(2);
          console.log(frequency, shardCount, "---------", i)
          setSchedule(speed)
          frequency++
          resolve("执行了")
          setCancelId(response.data.id)
          if(speed === "100.00"){
            setEnterLoading(false)
            cancel()
            getList("")
            message.success("上传成功")
          }
        })
        .catch(err => {
          setEnterLoading(false)
        })
    })
  }

  const cancel = () => {
    if(enterLoading){
      cancelUploadFils=true
      console.log(cancelId)
      warenhouseApi.cancelUpload({id: cancelId})
        .then(res => {
          message.success("您已取消上传")
          setEnterLoading(false)
          change()
          setVersionDisabled(true)
          setVersionList([])
          publishFrom.resetFields()
        })
      // return
    }
    change()
    setVersionDisabled(true)
    setVersionList([])
    publishFrom.resetFields()
    setVerification(false)
    setFileList([])
    // setInputValue("")
    setAutoCompleteValue("")
    setSchedule(0)
    setCancelId(0)
  }

  const customRequest = () => {
    // console.log('customRequest')
  }

  const beforeUpload = (file) => {
    return true
  }

  const handleChange = (file) => {
    file["status"]='done'
    setFileList([file])
  }



  return (
    <Modal
      title={"发布模型"}
      visible={PublishModelVisible}
      maskClosable={false}
      className={"publishmodel"}
      closeIcon
      onCancel={() => cancel()}
      onOk={() => confirmRelease()}
      width={500}
      confirmLoading={enterLoading}
    >
      <Form
        form={publishFrom}
        name='publishform'
        labelCol={{
          span: 3,
        }}
        wrapperCol={{
          span: 24,
        }}
      >
        <Form.Item
          label="名称"
          name="username"
          rules={[{ required: true, message: "请输入模型名称" }]}
        >
          <Input
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            placeholder="请输入模型名称"/>
        </Form.Item>

        <Form.Item
          label="版本"
          name="version"
          extra="若选择历史版本进行发布，则会覆盖历史内容"
          className='itemAutoComplete'
        >
          <span className='xingxing'></span>
          <AutoComplete
            placeholder="请选择您的版本"
            disabled={versionDisabled} // 是否禁用
            value={autoCompleteValue}
            onChange={(e) => {
              setAutoCompleteValue(e)
              if(!e.length){
                setVerification(true)
              }else{
                setVerification(false)
              }
            }} // (e) => setAutoCompleteValue(e.target.value)
            options={versionList}
            status="error"
          />
          {
            verification? (
              <span className='errording'>请 选择/输入 模型版本</span>
            ) : null
          }
        </Form.Item>


        <Form.Item
          label="简介"
          name="Introduction"
          extra="可以备注模型准确率、召回率等指标，以及本次训练的主要调参，变更项等信息，方便后续查阅"
        >
          <Input.TextArea
            rows={4}
            // showCount
          />
        </Form.Item>


        {whetherUpload? (
          <Form.Item
            className='select'
            name="upload"
            rules={[{ required: true, message: "请选择上传文件" }]}
          >
            <Upload
              customRequest={customRequest}
              beforeUpload={beforeUpload}
              onChange={(e) => handleChange(e.file)}
              multiple= {true}
              fileList={fileList}
              className="upload">
              <Button icon={<UploadOutlined />}>选择上传文件</Button>
            </Upload>
          </Form.Item>
        ) : null}
        {/* enterLoading */}
        {whetherUpload && enterLoading?(
          <Form.Item
            className='progressbar'
            name="progressbar"
          >
            <Progress percent={schedule} />
          </Form.Item>
        ) : null}
      </Form>
    </Modal>
  )
}
export default PublishModel
