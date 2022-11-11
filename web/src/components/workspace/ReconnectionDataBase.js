import React,{useEffect} from 'react';
import { useSetState } from 'ahooks'
import { Form, Input, message, Modal } from "antd"
import DataSetIcon from "../dataSet/DataSetIcon"
import dataSetApi from '../../services/dataSetApi'
import intl from "react-intl-universal"


function ReconnectionDataBase (props) {
  const { dataBaseItem, visibleData, showReconnectionModal, RefreshDataBase } = props;
  const [reconnectForm] = Form.useForm()

  const [selectItemInfo, setSelectItemInfo] = useSetState({
    aliasDB: "",
    db: "",
    dbUrl: "",
    dbName: "",
  })

  const [modalShow, setModalShow] = React.useState(false)

  useEffect(() => {
    if(visibleData === false){
      reconnectForm.resetFields()
      setSelectItemInfo({
        aliasDB: "",
        db: "",
        dbUrl: "",
        dbName: "",
      })
    }else{
      setSelectItemInfo({
        aliasDB: dataBaseItem.alias,
        db: dataBaseItem.datasource,
        dbUrl: dataBaseItem.path,
        dbName: dataBaseItem.dbname,
      })
    }
  }, [visibleData])

  // // 重新连接
  const enterReconnectCloud = () => {
    setModalShow(true)
    console.log('重新连接部分')
    reconnectForm.validateFields()
      .then(value => {
        const { username, password } = value
        console.log(selectItemInfo)
        const data = {
          ...selectItemInfo,
          username: username ? username.trim() : "",
          password: password ? password.trim() : "",
        }
        dataSetApi.reconnectDataBase(data)
        .then(() => {
          setModalShow(false)
          message.success(intl.get("RELINK_SUCCESSFULLY"))
          RefreshDataBase()
          showReconnectionModal()
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
        title={`${intl.get("RECONNECTION")} ${dataBaseItem.alias}`}
        visible={visibleData}
        onOk={enterReconnectCloud}
        onCancel={() => {
          showReconnectionModal()
          setModalShow(false)
        }}
        getContainer={false}
        forceRender={true}
        confirmLoading={modalShow}
      >
        <Form
          form={reconnectForm}
          labelAlign={"right"}
          labelCol={{
            span: 8,
          }}

        >
          <Form.Item
            label={intl.get('DATABASE_USERNAME')}
            name={"username"}
            rules={[{ required: true }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label={intl.get("DATABASE_PASSWORD")}
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
export default ReconnectionDataBase;
