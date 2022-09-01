import "./RightSiderShow.less"
import { observer } from "mobx-react"
import { toJS } from "mobx"
import globalData from "idp/global"

function RightSiderShow(props) {
  const { rightLineSelectKey } = props
  const rightSideList = toJS(globalData.rightSideControl.rightSideList)

  const renderContent = () => {
    const item = rightSideList.find(item => item.key === rightLineSelectKey)
    if (item) {
      return item.component
    }
    return null
  }

  return <div className={"right-sider-show-container"}>{renderContent()}</div>
}

export default observer(RightSiderShow)
