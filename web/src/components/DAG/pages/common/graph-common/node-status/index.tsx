import React from 'react'
import {
  CheckCircleOutlined,
  CloseCircleOutlined,
  SyncOutlined,
  MinusCircleOutlined,
  HourglassOutlined,
} from '@ant-design/icons'

interface Props {
  className?: string
  status: 'success' | 'fail' | 'running' | 'ready' | 'upChangeSuccess' | 'waiting'
}

export const NodeStatus: React.FC<Props> = (props) => {
  const { className, status } = props
  switch (status) {
    case 'fail':
      return (
        <div className={className} style={{background:"#FFF2F0"}}>
          <CloseCircleOutlined style={{ color: '#ff4d4f' }} /><span style={{marginLeft:'7px', lineHeight:"22px"}}>失败</span>
        </div>
      )
    case 'success':
    case 'upChangeSuccess': {
      const color = status === 'success' ? '#2ecc71' : '#1890ff'
      return (
        <div className={className} style={{background:"#F6FFED"}}>
          <CheckCircleOutlined style={{ color }} /><span style={{marginLeft:'7px', lineHeight:"22px"}}>成功</span>
        </div>
      )
    }
    case 'ready':
      return (
        <div className={className}>
          <MinusCircleOutlined style={{ color: '#1890ff' }} /><span style={{ marginLeft: '7px', lineHeight: "22px" }}>未开始</span>
        </div>
      )
    case 'waiting':
      return (
        <div className={className}>
          <HourglassOutlined spin={true} style={{ color: '#1890ff' }} /><span style={{ marginLeft: '7px', lineHeight: "22px" }}>等待中</span>
        </div>
      )
    case 'running':
      return (
        <div className={className} style={{background:"#FFFBE6"}}>
          <SyncOutlined spin={true} style={{ color: '#1890ff' }} /><span style={{marginLeft:'7px', lineHeight:"22px"}}>执行中</span>
        </div>
      )
    default:
      return null
  }
}
