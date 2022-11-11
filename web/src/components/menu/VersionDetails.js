import React,{ useEffect } from 'react';
import { Modal, Divider, Button } from "antd"
import './VD.less'
import historyData from './VersionDetails.json'
import intl from "react-intl-universal"
function VersionDetails (props){
  const { visible, cancel } = props

  const versionHistory = () => {
    return (
      <div>
        {historyData.version.map((item,index) => {
          return (
            <div className='all' key={index}>
              <Divider plain dashed>{item.time}</Divider>
              <div className='v-title'>{item.versionnum}</div>
              {item.characteristic.length !== 0? (
                <ul>
                  <li className='li-title'>新特性:</li>
                  {
                    item.characteristic.map(iChild => (
                      <li className='li-body' key={iChild}>{iChild}</li>
                    ))
                  }
                </ul>) : null}
                {item.functionadjustment.length !== 0? (
                <ul>
                  <li className='li-title'>功能调整:</li>
                  {
                    item.functionadjustment.map(iChild => (
                      <li className='li-body' key={iChild}>{iChild}</li>
                    ))
                  }
                </ul>
              ): null}
              {item.repair.length !== 0? (
                <ul>
                  <li className='li-title'>Bug修复:</li>
                  {
                    item.repair.map(iChild => (
                      <li className='li-body' key={iChild}>{iChild}</li>
                    ))
                  }
                </ul>
              ) : null}
            </div>
          )
        })}
      </div>
    )
  }
  return(
    <Modal
      title={intl.get("HISTORIC_VERSION")}
      visible={visible}
      maskClosable={false}
      className={"history"}
      footer={[
        <Button key="back" type="primary" onClick={() => cancel()}>
          知道了
        </Button>]}
      centered
      width={888}
      // width={500}
      closeIcon
    >
      <div className='idp'>IDP</div>
      <div className='moment'>{historyData['version'][0]['versionnum']}</div>
      <div className='modal-warpper'>
        <div className='modal-scroll'>
          {versionHistory()}
        </div>
      </div>
      <div className='copyright'>Copyright © 北京白海科技有限公司</div>
    </Modal>
  )
}
export default VersionDetails
