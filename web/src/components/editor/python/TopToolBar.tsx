import { Button, Col, Input, Row } from "antd"
import Icons from "../../../components/Icons/Icons"
import intl from "react-intl-universal"
import { useState } from "react"

interface Props {
  doRun: Function
  doStop: Function
  status: string
}

export const TopToolBar: React.FC<Props> = (props: Props) => {
  const { doRun, doStop, status } = props
  const [parameters, setParameters] = useState('')

  const stop = () => {
    doStop()
  }

  const run = () => {
    doRun(parameters)
  }

  const stateButton = () => {
    switch (status) {
      case "pending":
        return (
          <Button
            icon={<Icons.BHCellPendingIcon />}
            style={{ height: '24px' }}
            type="text"
          ></Button>
        )
      case "executing":
        return (
          <Button
            icon={<Icons.BHCellExecutingIcon />}
            style={{ height: '24px' }}
            type="text"
            onClick={stop}
          ></Button>
        )
      default:
        return (
          <Button
            icon={<Icons.BHCellReadyIcon />}
            style={{ height: '24px' }}
            type="text"
            onClick={run}
          ></Button>
        )
    }
  }

  return (
    <div>
      <Row>
        <Col span={1} style={{ paddingLeft: '5px' }}>
          {stateButton()}
        </Col>
        <Col span={23}>
          <Input
            prefix={intl.get('STARTUP_PARAMETERS')}
            placeholder={intl.get('STARTUP_PARAMETERS_DESCRIPTION')}
            bordered={false}
            onChange={(e) => { 
              setParameters(` ${e.target.value}`) 
            }}
          />
        </Col>
      </Row>
    </div>
  )
}