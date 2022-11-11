import React, { useContext } from "react"
import { Drawer, Typography, message } from "antd"
import CronGenerator from "../cron-panel/cronPanel"
import { CanvasToolBarContext } from "../../pages/dag-canvas/canvas-toolbar"
interface Props {
  visible: boolean
  experimentId: any
  cronConfig: any
}

export const PlanModal: React.FC<Props> = (props) => {
  const { setExperimentPlan, setDrawerVisible, setModalVisible } =
    useContext(CanvasToolBarContext)
  const { visible } = props
  // const [form] = Form.useForm()

  // const onOk = () => {
  //   form.submit()
  // }
  // const onFinish = (values) => {
  //   //    const plan =  cronFormat(values)
  // }
  const onClose = (data?) => {
    console.log(data)
    setDrawerVisible(false)
  }
  // const refCronGenerator = React.createRef()
  const disableToggleToSimple = /,|\?|\/|-/.test(
    props.cronConfig?.cronExpression
  )
  return (
    <Drawer
      title={
        <div>
          <span style={{ marginRight: 20 }}>设置执行计划</span>{" "}
          <Typography.Text
            style={{
              cursor: "pointer",
              color: "#1890ff",
              fontSize: 13,
            }}
            onClick={() => {
              if (disableToggleToSimple) {
                message.warning("当前的表达式过于复杂,无法切换回简单模式")
                return
              }
              setModalVisible(true)
              onClose()
            }}
          >
            切换简单模式
          </Typography.Text>
        </div>
      }
      closable={true}
      visible={visible}
      onClose={onClose}
      width="750px"
      children={
        <CronGenerator
          setExperimentPlan={setExperimentPlan}
          experimentId={props.experimentId}
          cronConfig={props.cronConfig}
        />
      }
    />
  )
}
