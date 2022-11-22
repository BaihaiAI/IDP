import {Modal} from "antd"
import {ExclamationCircleOutlined,InfoCircleOutlined,CloseCircleOutlined } from "@ant-design/icons"



// icon:	<InfoCircleOutlined style={{color:"#1890FF"}} />,
// icon:	<ExclamationCircleOutlined />,
// icon:	<CloseCircleOutlined style={{color:"red"}} />,

export function showModel1(onOk,onCancel) {
  Modal.confirm({
    icon:<ExclamationCircleOutlined />,
    title:"是否确认共享您的模型？",
    content:"模型共享后，其他用户可以在共享中心查看、测试、克隆您的模型",
    onOk,
    onCancel
  })
}

export function showModel2(onOk,onCancel) {
  Modal.confirm({
    icon:<ExclamationCircleOutlined />,
    title:"模型创建成功！是否确认共享您的模型？",
    content:"模型共享后，其他用户可以在共享中心查看、测试、克隆您的模型。",
    onOk,
    onCancel
  })
}

export function showModel3(
  onOk,
  onCancel
) {
  Modal.confirm({
    icon:<CloseCircleOutlined style={{color:"red"}} />,
    title:"抱歉，您没有模型共享操作权限。您完成 个人信息认证 后即可获得模型共享操作权限",
    okText:"去认证",
    onOk(){
      window.location.href = '/team/myAccount/personalInformation'
      typeof onOk ==='function'&& onOk()
    },
    onCancel
  })
}

export function showModel4(
 onOk,
 onCancel
) {
  Modal.confirm({
    icon:	<InfoCircleOutlined style={{color:"#1890FF"}} />,
    title:"您的“模型共享”申请已提交审批，审批通过后“模型共享”才会生效。",
    content:"您可以在审批中心查看审批进度，点击下方按钮跳转到审批中心。",
    okText:"审批中心",
    cancelText:"知道了",
    onOk(){
      const origin = window.location.origin
      const path = "/team/approvalCenter"
      const realUrl = origin + path
      window.open(realUrl)
      typeof onOk ==='function'&& onOk()
    },
    onCancel
  })
}

export function showModel5(onOk,onCancel) {
  Modal.confirm({
    icon:	<InfoCircleOutlined style={{color:"#1890FF"}} />,
    title:"是否取消共享？",
    content:"取消共享后，其他用户无法在共享中心查看您的模型。",
    okText:"继续共享",
    cancelText:"取消共享",
    onOk,
    onCancel,
  })
}

export function showModel6(onOk,onCancel) {
  Modal.confirm({
    title:"您的“模型共享”申请还在审批中，是否取消共享？",
    content:"取消共享后，“模型共享”申请将被撤回。",
    okText:"继续共享",
    cancelText:"取消共享",
    onOk,
    onCancel
  })
}
