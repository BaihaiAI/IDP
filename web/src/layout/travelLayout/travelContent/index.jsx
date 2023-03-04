import React, { useEffect } from 'react'
import { Layout } from "antd"
import LeftNavMenu from "@/pages/common/leftNav"
import RouterConfig from "@/router/router"
import { loadLocalSystemExtensions } from '../../../../config/extensions-config';

function TravelContent(props) {

  useEffect(() => {
    loadLocalSystemExtensions(() => { });
  }, [])

  return (
    <Layout>
      <LeftNavMenu />
      <RouterConfig />
    </Layout>
  )
}

export default TravelContent
