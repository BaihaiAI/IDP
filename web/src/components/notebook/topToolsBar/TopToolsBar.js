import { useState } from "react"
import Icons from "../../Icons/Icons"
import { Button, Space, Row, Tooltip, Col } from "antd"
import intl from "react-intl-universal"
import "./topToolsBar.less"
import { ResourceBar } from "./ResourceBar"

const TopToolsBar = (props) => {
  const {
    isExecuting,
    isPaused,
    runAllCell,
    stopAllCell,
    restartKernel,
    resumeRun,
    saveVersion,
    hasEffectiveClick = true,
    resourceRef,
  } = props

  const [restartKernelDisabled, setRestartKernelDisabled] = useState(false)

  return (
    <Row className="toptoolsbar">
      <Col>
        <Space>
          {isPaused ? <Tooltip placement="bottom" title={intl.get("RESUME_RUN_TIP")}>
            <Button
              icon={<Icons.BHResumeRunIcon />}
              size="small"
              type="text"
              onClick={() => {
                if (!hasEffectiveClick) return
                resumeRun()
              }}
            >
              {intl.get("RESUME_RUN")}
            </Button>
          </Tooltip> : (isExecuting ? (
            <Tooltip placement="bottom" title={intl.get("STOPALLTIP")}>
              <Button
                icon={<Icons.BHStopAllIcon />}
                size="small"
                type="text"
                onClick={() => {
                  if (!hasEffectiveClick) return
                  stopAllCell()
                }}
              >
                {intl.get("STOPALL")}
              </Button>
            </Tooltip>
          ) : (
            <Tooltip placement="bottom" title={intl.get("RUNALLTIP")}>
              <Button
                icon={<Icons.BHStartAllIcon />}
                size="small"
                type="text"
                onClick={() => {
                  if (!hasEffectiveClick) return
                  runAllCell()
                }} 
              >
                {intl.get("RUNALL")}
              </Button>
            </Tooltip>
          ))}
          <Tooltip placement="bottom" title={intl.get("RESTARTTIP")}>
            <Button
              icon={<Icons.BHRestartIcon />}
              size="small"
              type="text"
              disabled={restartKernelDisabled}
              onClick={() => {
                if (!hasEffectiveClick) return
                setRestartKernelDisabled(true)
                restartKernel(() => setRestartKernelDisabled(false))
              }}
            >
              {intl.get("RESTART")}
            </Button>
          </Tooltip>
          <Tooltip placement="bottom" title={intl.get("SAVE_VERSION_TIP")}>
            <Button
              icon={<Icons.BHSaveIcon />}
              size="small"
              type="text"
              onClick={() => {
                if (!hasEffectiveClick) return
                saveVersion()
              }}
            >
              {intl.get("SAVE_VERSION")}
            </Button>
          </Tooltip>
        </Space>
      </Col>
      <Col span={13}>
        <ResourceBar resourceRef={resourceRef} />
      </Col>
    </Row>
  )
}

export default TopToolsBar
