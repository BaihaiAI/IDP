import React from 'react'
import {Layout} from "antd"
import LeftNavMenu from "@/pages/common/leftNav"
import RouterConfig from "@/router/router"

function TravelContent(props) {
  return (
      <Layout>
        <LeftNavMenu />
        <RouterConfig />
      </Layout>
  )
}

export default TravelContent
