import React from 'react'
import {Layout} from "antd"
import useShowFooter from "@/utils/hook/useShowFooter"

const { Footer } = Layout

function TravelFooter(props) {
  const isShowFooter = useShowFooter()

  return (
    <Footer
      style={{ display: isShowFooter ? "flex" : "none" }}
      className="footbar"
    >
      <div
        className="footbar"
      >
        <div className="runstate">

        </div>

        <div className="totalstate">

        </div>
      </div>
    </Footer>
  )
}

export default TravelFooter
