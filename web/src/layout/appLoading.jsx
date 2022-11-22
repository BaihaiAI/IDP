import React from 'react'
import {Col, Row} from "antd"
import loadingImg from "../assets/public/loading.gif"

function AppLoading(props) {

  const { initTips } = props

  return (
    <div>
      <Row>
        <Col span={24} style={{ textAlign: "center" }}>
          <img
            src={loadingImg}
            alt=""
          />
        </Col>
      </Row>
      <Row>
        <Col span={24} style={{ textAlign: "center" }}>
          {initTips}
        </Col>
      </Row>
    </div>
  )
}

export default AppLoading
