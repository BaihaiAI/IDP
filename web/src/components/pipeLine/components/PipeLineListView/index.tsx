import { Tabs } from "antd"
import "./index.less"
import PipeLineJobList from "./PipeLineJobList"
import PipeLineJobInstanceList from "./PipeLineJobInstanceList"
import { useState } from "react"
import intl from "react-intl-universal"

const { TabPane } = Tabs

export type statusType =
  | "Init"
  | "Schedule"
  | "Pending"
  | "Success"
  | "Running"
  | "Fail" |'Kill'

export enum StatusEnum {
  Init = "未计划",
  Schedule = "计划内",
  Pending = "等待中",
  Success = "成功",
  Running = "运行中",
  Fail = "失败",
  Kill = '被中断'
}

function PipeListView() {
  const [taskOrJobKey, setTaskOrJobKey] = useState("task")

  return (
    <div id="pipe-line-list-view-container">
      <Tabs
        onChange={(activeKey) => {
          setTaskOrJobKey(activeKey)
        }}
        activeKey={taskOrJobKey}
      >
        <TabPane tab={intl.get("WORKFLOW_INSTANCE")} key="task">
          <div style={{ paddingTop: 10 }}>
            <PipeLineJobInstanceList taskOrJobKey={taskOrJobKey} />
          </div>
        </TabPane>
        <TabPane tab={intl.get("WORKFLOW")} key="job">
          <div style={{ paddingTop: 10 }}>
            <PipeLineJobList taskOrJobKey={taskOrJobKey} />
          </div>
        </TabPane>
      </Tabs>
    </div>
  )
}

export default PipeListView
