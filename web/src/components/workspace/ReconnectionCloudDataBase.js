import React,{useEffect, useState} from 'react';
import Icons from '../Icons/Icons'
import { useSetState } from 'ahooks'
import { Form, Input, message, Modal } from "antd"
import DataSetIcon from "../dataSet/DataSetIcon"
import dataSetApi from '../../services/dataSetApi'
import intl from "react-intl-universal"


function ReconnectionCloudDataBase (props) {
  const { visibleData, showReconnectionCloudModal, cloudItem, RefreshDataBase } = props;
  const [reconnectForm] = Form.useForm()
  const [filesyStemItem, setFilesyStemItem] = useState({
    alias: "",
    bucket: "",
    endPoint: "",
    cloudName: "",
    fsType: "",
    mountPath: "",
  })

  const [modalShow, setModalShow] = useState(false)
  useEffect(() => {
    if(visibleData) {
      setFilesyStemItem({
        alias: cloudItem.title,
        bucket: cloudItem.bucket,
        endPoint: cloudItem.endPoint,
        cloudName: cloudNameValue(cloudItem.sourceType),
        fsType: cloudItem.sourceType,
        mountPath: cloudItem.title,
      })
    } else {
      // 关闭 modal
      reconnectForm.resetFields()
      setFilesyStemItem({
        alias: "",
        bucket: "",
        endPoint: "",
        cloudName: "",
        fsType: "",
        mountPath: "",
      })
    }
  }, [visibleData])

  const cloudNameValue = (cName) => {
    let cloudName;
    switch(cName){
      case 'amazon s3':
        cloudName='s3'
        break
      case 'aliyun s3':
        cloudName='aliyun'
        break
      case 'ucloud s3':
        cloudName='cloudName'
        break
      default:
        cloudName=cName
    }
    return cloudName;
  }

  const enterReconnectCloud = () => {
    setModalShow(true)
    reconnectForm.validateFields()
      .then(value => {
        const { username, password} = value
        const data = {
          ...filesyStemItem,
          accessKey: username,
          secretKey: password
        }
        console.log(data)
        dataSetApi.mountCloud(data)
          .then(res => {
            setModalShow(false)
            message.success(intl.get('RELINK_SUCCESSFULLY'))
            RefreshDataBase()
            showReconnectionCloudModal()
            // dataSetApi.kernelMountCloud(data).then(res => {

            // }).catch(err => {
            //   setModalShow(false)
            // })
          })
          .catch(() => {
            setModalShow(false)
          })
      })
      .catch(() => {
        setModalShow(false)
      })
  }

  return (
    <div>
      <Modal
        width={448}
        title={`${intl.get('RECONNECTION')} ${cloudItem.title}`}
        visible={visibleData}
        onOk={enterReconnectCloud}
        onCancel={() => {
          showReconnectionCloudModal()
          setModalShow(false)
        }}
        confirmLoading={modalShow}
        getContainer={false}
        forceRender={true}
      >
        <Form
          form={reconnectForm}
          labelAlign={"right"}
          labelCol={{
            span: 8,
          }}
        >
          <Form.Item
            label={"Access Key"}
            name={"username"}
            rules={[{ required: true }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label={"secret Key"}
            name={"password"}
            rules={[{ required: true }]}
          >
            <Input.Password
              style={{
                borderRight: "1px solid",
                borderLeft: "1px solid",
                borderColor: "#d9d9d9",
                borderRadius: '2px'
              }}
              iconRender={(visible) =>
                visible ? (
                  <DataSetIcon.showPwdIcon style={{ fontSize: 20 }} />
                ) : (
                  <DataSetIcon.hidePwdIcon style={{ fontSize: 20 }} />
                )
              }
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}
export default ReconnectionCloudDataBase;
