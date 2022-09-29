import React from 'react'
import "./index.less"

function TravelHeader(props) {
  return (
    <div className={'travel-header-container'}>
      <div className="header">
        <div className="header-left">
          <div
            className={"logo"}
            style={{
              background: `url(${require('@/assets/image/logo.png').default}) no-repeat`,
              backgroundSize: 'contain',
              height: '29px',
              width: '34px',
              float: 'left',
              marginTop: '5px',
              cursor: 'pointer'
            }}
          />
        </div>
        <div className="header-right">
          <span onClick={()=>{
            window.location.href = '/login'
          }}>
            登录/注册
          </span>
        </div>
      </div>
    </div>
  )
}

export default TravelHeader
