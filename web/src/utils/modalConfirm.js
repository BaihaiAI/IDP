import {Modal} from "antd"
import {ExclamationCircleOutlined} from "@ant-design/icons"

export function showApproveConfirm() {
  Modal.confirm({
    title: '完成个人信息认证后才可以进行此操作',
    icon: <ExclamationCircleOutlined />,
    okText:"去认证",
    onOk() {
      window.location.href = '/team/myAccount/personalInformation'
    },
  })
}
