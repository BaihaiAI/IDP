import React from "react";
import contentApi from "../../../../services/contentApi";
import {
  Modal,
  message
} from "antd"
function UpDatePlayIDP (props){
  const { changeVisible, visible } = props

  const enterReset = () => {
    contentApi.cat2({path: "/玩转IDP/version"}).then(res => {
      console.log(res)
      let version;
      if(res.data.code === 21000000){
        version = res.data.data.content;
        console.log(version)
        contentApi.cat2_example({version}).then(res => {
          message.success(res.message)
          changeVisible()
        })
      }else if(res.data.code === 51040000){
        // 空值
        version = ""
        contentApi.cat2_example({version}).then(res => {
          message.success(res.message)
          changeVisible()
        })
      }
      
    }).catch(err => {
      console.log('--------', err)
    })
  }
  return (
    <Modal 
      title="更新'玩转IDP'" 
      visible={visible} 
      onOk={enterReset} 
      onCancel={changeVisible}
    >
      <div>&nbsp;</div>
      <p>此操作会重置文件夹“玩转IDP”</p>
    </Modal>
  )
}

export default UpDatePlayIDP