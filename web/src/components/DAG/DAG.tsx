import './DAG.less'
import React from 'react'
import '@antv/x6-react-shape'
import Dag from './pages'
// interface Props {
//   experimentId: string
//   experimentInstanceId?: string
//   className?: string
//   mode?: string
// }
class DAG extends React.Component<{ node?: Node, experimentId: string, experimentInstanceId?: string, mode?: string }> {
  shouldComponentUpdate() {
    // const { node, mode } = this.props

    return false
  }
  componentDidMount(){
  }
  render() {
    const { experimentId, experimentInstanceId, mode } = this.props
    return (
      <div className={`node`} id="container">
        <Dag experimentId={experimentId} experimentInstanceId={experimentInstanceId} mode={mode} />
      </div>
    )
  }
}
export default DAG;
