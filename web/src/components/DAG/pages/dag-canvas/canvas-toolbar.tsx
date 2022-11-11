import React, { Dispatch, useCallback, useEffect, useState } from "react"
import { Toolbar } from "@antv/x6-react-components"
import {
  // GatewayOutlined,
  // PlaySquareOutlined,
  // RollbackOutlined,
  SaveOutlined,
  PlayCircleOutlined,
  FieldTimeOutlined,
  LogoutOutlined,
} from "@ant-design/icons"
import { Switch, message } from "antd"
import { useObservableState } from "../../common/hooks/useObservableState"
// import { RxInput } from '../../component/rx-component/rx-input'
// import { showModal } from '../../component/modal'
import { PlanModal } from "../../component/modal/planModal"
import { PlanModalSimple } from "../../component/modal/planModal-simple"
// import { addNodeGroup } from '../../mock/graph'
// import { BehaviorSubject } from 'rxjs'
import { useExperimentGraph } from "../rx-models/experiment-graph"
// import { formatGroupInfoToNodeMeta } from '../../pages/rx-models/graph-util'
import pipeLineApi from "idpServices/pipelineApi"
import styles from "./canvas-toolbar.module.less"
// import Modal from 'antd/es/modal'
// import Select from 'rc-select'
import { queryExperiment } from "../../services/graph"

const { Item, Group } = Toolbar

interface Props {
  experimentId: string
  mode: string
}

enum Operations {
  UNDO_DELETE = "UNDO_DELETE",
  GROUP_SELECT = "GROUP_SELECT",
  RUN_SELECTED = "RUN_SELECTED",
  NEW_GROUP = "NEW_GROUP",
  UNGROUP = "UNGROUP",
  SETTING = "SETTING",
  RUN_PLAN = "RUN_PLAN",
  SAVE = "SAVE",
  RUN = "RUN",
}

// todo createContext
export const CanvasToolBarContext = React.createContext<{
  setExperimentPlan?: Dispatch<any>
  setDrawerVisible?: Dispatch<boolean>
  setModalVisible?: Dispatch<boolean>
}>({})

export const CanvasToolbar: React.FC<Props> = (props) => {
  const { experimentId, mode } = props
  const [modalVisible, setModalVisible] = useState<boolean>(false)
  const [drawerVisible, setDrawerVisible] = useState<boolean>(false)
  // const [selectionEnabled, setSelectionEnabled] = useState<boolean>(false)
  const [isRunning, setIsRunning] = useState<boolean>(false)

  const expGraph = useExperimentGraph(experimentId)

  const [expId, setExpId] = useState(
    experimentId === "0" ? expGraph.experimentId : experimentId
  )

  const [activeNodeInstance] = useObservableState(
    () => expGraph.activeNodeInstance$
  )
  const [selectedNodes] = useObservableState(() => expGraph.selectedNodes$)
  const [selectedGroup] = useObservableState(() => expGraph.selectedGroup$)
  //for running
  const [running] = useObservableState(() => expGraph.running$)
  const [preparingRun, setPreparingRun] = useState(false)
  const [preparingStop, setPreparingStop] = useState(false)
  const [experimentPlan, setExperimentPlan] = useState<any>()

  const saveExperimentGraph = () => {
    expGraph.saveExperimentGraph((success: boolean) => {
      if (success) {
        message.success("保存成功！")
        setExpId(expGraph.experimentId)
      } else {
        message.error("保存失败！")
      }
    })
  }

  // running 的值发生变化，说明运行或停止按钮的操作产生了作用
  useEffect(() => {
    setPreparingRun(false)
    setPreparingStop(false)
  }, [running])

  // 运行实验
  const onRunExperiment = useCallback(() => {
    setPreparingRun(true)
    expGraph.runGraph().then((res: any) => {
      if (!res.success) {
        setPreparingRun(false)
      }
    })
  }, [expGraph])

  // 停止运行
  const onStopRunExperiment = useCallback(() => {
    setPreparingStop(true)
    expGraph.stopRunGraph().then((res: any) => {
      if (!res.success) {
        setPreparingStop(false)
      }
    })
  }, [expGraph])
  const runningConfigs = [
    {
      content: "运行",
      tip: "依次运行工作流的每个组件",
      icon: PlayCircleOutlined,
      disabled: preparingRun,
      clickHandler: onRunExperiment,
    },
    {
      content: "停止",
      tip: "停止运行工作流",
      icon: LogoutOutlined,
      disabled: preparingStop,
      clickHandler: onStopRunExperiment,
    },
  ]

  const runningConfig = runningConfigs[Number(!!running)]
  const RunningIcon = runningConfig.icon

  useEffect(() => {
    queryExperiment(expId).then(function (job: any) {
      //job.status 在新建时未undefind
      setIsRunning(job.status === "Schedule" ? true : false)
      setExperimentPlan(job.cronConfig || {}) //新建为{}
    })
  }, [])

  const onClickItem = useCallback(
    (itemName: string) => {
      if (mode === "view") return

      switch (itemName) {
        case Operations.SAVE:
          saveExperimentGraph()
          break

        case Operations.RUN:
          runningConfig.clickHandler()
          break

        case Operations.SETTING: {
          if (expId === "0") {
            message.warning("请您先构建并保存工作流！")
            return
          }
          //  如果cronType表达式比较复杂的话 则显示复杂模式
          if (/,|\?|\/|-/.test(experimentPlan?.cronExpression)) {
            setDrawerVisible(true)
          } else {
            if (experimentPlan.cronType === "advanced") {
              setDrawerVisible(true)
            } else {
              setModalVisible(true)
            }
          }
          break
        }

        case Operations.UNGROUP: {
          const descendantNodes = selectedGroup!.getDescendants()
          const childNodes = descendantNodes.filter((node) => node.isNode())
          childNodes.forEach((node) => {
            const nodeData = node.getData<any>()
            node.setData({ ...nodeData, groupId: 0 })
          })
          selectedGroup!.setChildren([])
          expGraph.deleteNodes(selectedGroup!)
          expGraph.unSelectNode()
          break
        }

        default:
      }
    },
    [
      expGraph,
      activeNodeInstance,
      selectedNodes,
      expId,
      selectedGroup,
      modalVisible,
      drawerVisible,
      running,
      isRunning,
      experimentPlan,
    ]
  )

  const onSwicth = (checked: boolean, event: Event) => {
    if (expId === "0") {
      message.warning("请您先构建并保存工作流！")
      return
    }
    pipeLineApi
      .jobCreatePlan({
        jobId: expId,
        status: checked ? "Schedule" : "Init",
      })
      .then(function (data) {
        setIsRunning(checked ? true : false)

        message.success(checked ? "计划已启动！" : "计划已停止！")
        setIsRunning(checked ? true : false)
      })
      .catch(function (err) {
        message.error("失败！")
      })
  }

  return (
    <CanvasToolBarContext.Provider
      value={{ setExperimentPlan, setDrawerVisible, setModalVisible }}
    >
      <div className={styles.canvasToolbar}>
        <Toolbar hoverEffect={true} onClick={onClickItem}>
          <Group>
            <Item
              name={Operations.SETTING}
              tooltip="设置执行计划"
              text="设置执行计划"
              icon={<FieldTimeOutlined />}
            />
            <Item
              name={Operations.RUN_PLAN}
              tooltip="立即执行设置的计划"
              text={isRunning ? "计划内" : "未启动计划"}
              icon={
                <Switch
                  checkedChildren="开启"
                  unCheckedChildren="关闭"
                  size="small"
                  disabled={mode === "view" ? true : false}
                  defaultChecked={isRunning}
                  checked={isRunning}
                  onChange={onSwicth}
                />
              }
            />
          </Group>
          <Group>
            <Item
              name={Operations.RUN}
              text={runningConfig.content}
              tooltip={runningConfig.tip}
              icon={<RunningIcon />}
            />
          </Group>

          <Group>
            <Item
              name={Operations.SAVE}
              text="保存"
              tooltip="保存"
              icon={<SaveOutlined />}
            />
          </Group>
        </Toolbar>
      </div>
      <PlanModal
        visible={drawerVisible}
        experimentId={expId}
        cronConfig={experimentPlan}
      />
      {/* <PlanModalSimple visible={modalVisible} experimentId={expId} cronConfig={experimentPlan}/> */}
      <PlanModalSimple
        visible={modalVisible}
        experimentId={expId}
        cronConfig={experimentPlan}
      />
    </CanvasToolBarContext.Provider>
  )
}
