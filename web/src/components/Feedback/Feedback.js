import React from "react"
import { userId } from "@/store/cookie"
import feedbackApi from "@/services/feedbackApi"
import intl from "react-intl-universal"
import {  Modal, Form, Input, Upload } from "antd"

import { PlusOutlined } from "@ant-design/icons"
import "./Feedback.css"
import PropTypes from "prop-types"

const { TextArea } = Input

function getBase64(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.readAsDataURL(file)
    reader.onload = () => resolve(reader.result)
    reader.onerror = (error) => reject(error)
  })
}

// 需要的props visible onOk onCancel category required
class Feedback extends React.Component {

  static propTypes = {
    visible:PropTypes.bool,
    onOk:PropTypes.func,
    onCancel:PropTypes.func,
    category:PropTypes.string,
    required:PropTypes.bool,
    labelObj:PropTypes.object,
    width:PropTypes.number,
  }

  static defaultProps = {
    labelObj:{}
  }

  constructor(props) {
    super(props)
    this.state = {
      previewVisible: false,
      previewImage: "",
      previewTitle: "",
      visible: this.props.visible, //由父层控制
      listVisible: true,
      uploadView: false, //
      fileList: [
        //上传文件列表
      ],
      // 如果category为11 则是模型类的反馈
      category:props.category,
      required:props.required
    }
    this.formRef = React.createRef()
  }
  handleCancel = () => this.setState({ previewVisible: false })

  //问题项目
  problemItemClick = (e) => {
    this.setState("uploadView", true)
  }

  //取消提交
  ModalhandleCancel = () => {
    this.props.onCancel()
    this.setState({
      listVisible: true,
      fileList: [],
    })
    this.formRef.current.resetFields() //重置Form表单的内容
  }

  //提交表单
  onOk = () => {
    const form = this.formRef.current
    const _this = this

    form
      .validateFields()
      .then((values) => {
        _this.setState({
          listVisible: true,
        })

        let vals = _this.dataFormat(values)
        feedbackApi.save(vals)
          .then(function (res) {
            form.resetFields()
            Modal.success({
              title: `${intl.get("YOUR_FEEDBACK_HAS_BEEN_SUCCESSFULLY_SUBMITTED")}！`,
              content: `${intl.get("THANKS_FOR_YOUR_FEEDBACK")}！`,
            })
            _this.props.onOk()
            _this.setState({
              fileList: [],
            })
          })
          .catch(function (error) {
            Modal.error({
              title: `${intl.get("SUBMISSION_FAILED")}！`,
              content: `${intl.get("PLEASE_CHECK_YOUR_NETWORK")}！`,
            })
          })
        //onCreate(values);
      })
      .catch((info) => {
        console.log("Validate Failed:", info)
      })

    // this.props.onOk(values);//调用父组件给的onOk方法并传入Form的参数。
  }
  onCancel = () => {
    const form = this.formRef.current
    form.resetFields()
    this.setState({
      fileList:[]
    })
    this.props.form.resetFields() //重置Form表单的内容
    this.props.onCancel() //调用父组件给的方法
  }
  /*
	onFinish = valuse => {
		axios.post(this.state.path, values)
			.then(res){
				Modal.confrim(
							)
			}


	}
	*/

  //返回
  backHandle = (id) => {
    this.setState({
      listVisible: true,
    })
  }
  //前往
  forwardHandle = (e) => {
    this.setState({
      listVisible: false,
      fileList: [], //问题切换时重置上传列表
    })
    //设置表单分类
    this.formRef.current.setFieldsValue({
      category: e.target.type,
    })
  }

  //文件上传后更新state
  handleChange = ({ fileList: oldFileList,event }) => {
    const fileList = oldFileList.map((item) => {
      if (item.status === "error") {
        item.response = {}
      }
      return item
    })
    this.setState({ fileList })
  }
  dataFormat = (values) => {
    let arr = {}
    arr.category = this.props.category || "2"
    arr.userId = userId
    arr.feedback = values.feedback
    arr.contact = values.contact
    arr.userName = values.userName
    arr.fileIdList = []
    this.state.fileList &&
    this.state.fileList
        .filter((item) => item.status === "done")
        .forEach(function (item, i) {
          arr.fileIdList.push(item.response.data)
        })

    return arr
  }

  handlePreview = async (file) => {
    if (!file.url && !file.preview) {
      file.preview = await getBase64(file.originFileObj)
    }

    this.setState({
      previewImage: file.url || file.preview,
      previewVisible: true,
      previewTitle:
        file.name || file.url.substring(file.url.lastIndexOf("/") + 1),
    })
  }

  render() {
    const { labelObj,width,category } = this.props
    const isShowLabel = JSON.stringify(labelObj) !== "{}"

    const { fileList, previewVisible, previewTitle, previewImage } = this.state
    const requiredRules = this.props.required?[
      {required:true,message:"该项是必填的"}
    ]:[]

    const uploadButton = (
      <div>
        <PlusOutlined />
        <div style={{ marginTop: 8 }}>Upload</div>
      </div>
    )
    return (
      <>
        <Modal
          width={width}
          title={isShowLabel?labelObj.title : intl.get("FEEDBACK")}
          visible={this.props.visible}
          onCancel={this.ModalhandleCancel}
          onOk={this.onOk}
          destroyOnClose={true}
        >
          <div className="feedback-form">
            <Form ref={this.formRef}>
              {
                !isShowLabel && <h4>
                  <span>*</span>
                  { intl.get("SUGGESTION")}
                </h4>
              }
              <Form.Item
                label={isShowLabel && labelObj.feedback}
                labelCol={isShowLabel && {
                  span:4
                }}
                name="feedback"
                rules={[
                  {
                    required: true,
                    message: intl.get("FEEDBACK_RULES_EMPTY"),
                  },
                ]}
              >
                <TextArea
                  showCount
                  maxLength={200}
                  placeholder={isShowLabel?"请输入需求描述": intl.get("SUGGESTION_TEXTAREA_PLACEHOLDER")}
                />
              </Form.Item>
              {
                !isShowLabel&& <h4>
                  {intl.get("FEEDBACK_UPLOAD")}
                  <span>{fileList.length}/4</span>
                </h4>
              }
              <Form.Item
                label={isShowLabel && labelObj.upload}
                labelCol={isShowLabel && {
                  span:4
                }}
                name="upload"
              >
                <Upload
                  accept={category!=="11" &&'image/*'}
                  listType={"picture-card"}
                  action="/0/api/v1/feedback/uploadfile"
                  fileList={fileList}
                  onChange={this.handleChange}
                  data={{ userId }}
                  onPreview={this.handlePreview}
                >
                  {fileList.length >= 4 ? null : uploadButton}
                </Upload>
                <div style={{color:"#9B9B9B"}}>文件大小不超过20M</div>
              </Form.Item>

              {
                !isShowLabel && <h4>联系人</h4>
              }
              <Form.Item
                label={isShowLabel && labelObj.userName}
                labelCol={isShowLabel && {
                  span:4
                }}
                name="userName"
                rules={requiredRules}
              >
                <Input placeholder={"请输入联系人姓名"} />
              </Form.Item>

              {
                !isShowLabel &&  <h4>{intl.get("FEEDBACK_CONTACT")}</h4>
              }
              <Form.Item
                label={isShowLabel && labelObj.contact}
                labelCol={isShowLabel && {
                  span:4
                }}
                name="contact"
                rules={requiredRules}>
                <Input placeholder={isShowLabel?"请输入手机号或固话":intl.get("FEEDBACK_CONTACT_PLACEHOLDER")} />
              </Form.Item>

            </Form>
          </div>

          <Modal
            width={700}
            visible={previewVisible}
            title={previewTitle}
            footer={null}
            onCancel={this.handleCancel}
          >
            <img alt="preview" style={{ width: "100%" }} src={previewImage} />
          </Modal>
        </Modal>
      </>
    )
  }
}

export default Feedback
