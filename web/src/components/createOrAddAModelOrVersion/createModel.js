import React, { useEffect, useState, useRef } from "react"
import {
  Drawer,
  Button,
  Checkbox,
  Tooltip,
  Form,
  Col,
  Row,
  Input,
  Upload,
  Select,
  Radio,
  Modal,
  message, Progress,
} from "antd"
import "./createModel.less"
import ImgCrop from "antd-img-crop"
import createOrAddAModelOrVersionApi from "./services/createOrAddAModelOrVersionApi"
import {
  QuestionOutlined,
  CaretRightOutlined,
  PlusOutlined,
  UploadOutlined,
  ExclamationCircleOutlined,
  InfoCircleOutlined,
  QuestionCircleOutlined,
  PlusCircleOutlined,
  MinusCircleOutlined,
} from "@ant-design/icons"
import { gerModulePermissionList } from "./hooks/storage"
import { showModel2, showModel3, showModel4 } from "./modelFunction"
import { useMemoizedFn, useReactive, useUpdate } from "ahooks"
import intl from "react-intl-universal"
import useUrlState from "@ahooksjs/use-url-state"
import FileApi from "./services/fileApi"
import {nanoid} from "nanoid"

const { useForm } = Form
const { TextArea } = Input
const { Option } = Select
let createOrAddHeight
const children = []

const MAX_COUNT = 1

function CreateModel(props) {
  const {
    createDrawerVisible,
    changeCreateDrawerVisible,
    createOrAdd, // 区分 create 还是 version
    category, // create 使用
    packageId, // version 使用
    renewList, // version 使用
    setCreateDrawerVisible,
    initShareOrNot, //  在共享中心 使用过一次 hy
    loadPage,
    clicksViewFlg,
    PreFillIn,
    noteBookModelFile, // 在 noteBook 特有，notebook重不需要上传模型文件，而是已经指定好了
    noteBookModelEnv, // 在notebook创建模型时带过来的
  } = props


  const myModulePermissionList = gerModulePermissionList("my_model")

  const [createOrAddModelsForm] = useForm()
  // 创建模型版本 创建模型 高度
  const [createHeight, setCreateHeight] = useState(450)
  const [configureHeight, setConfigureHeight] = useState(550)

  // 是否共享
  const [shareOrNot, setShareOrNot] = useState(!!initShareOrNot)

  // 模型文件上传 上传列表
  const [modelFileFileList, setModelFileFileList] = useState([])
  const [modelFileUploadReturnValue, setModelFileUploadReturnValue] =
    useState("")

  // 上传封面
  const [coverList, setCoverList] = useState([])
  const [coverUploadReturnVal, setCoverUploadReturnVal] = useState("")

  // 上传效果演示
  const [effectList, setEffectList] = useState([])
  const [effectUploadReturnVal, setEffectUploadReturnVal] = useState("")

  const inputTypesArr = useReactive([
    {
      name: "arg0",
      type: "string",
      required: true,
      stringContent: "",
      fileContent: "",
      fileList: [],
    },
  ])

  const handleAddInputTypeClick = useMemoizedFn(() => {
    inputTypesArr.push({
      name: `arg${inputTypesArr.length}`,
      type: "string",
      required: true,
      stringContent: "",
      fileContent: "",
      fileList: [],
    })
    inputFileListObjRef.current[`arg${inputTypesArr.length-1}`] = []
    uploadInputFileListInfoRef.current[`arg${inputTypesArr.length-1}`] = []
  })

  const handleDecreaseInputTypeClick = useMemoizedFn((index) => {
    inputFileListObjRef.current[inputTypesArr[index].name] = []
    uploadInputFileListInfoRef.current[`arg${inputTypesArr.length-1}`] = []
    inputTypesArr.splice(index, 1)
  })

  const [state, setState] = useUrlState({})

  useEffect(() => {
    pass()
  }, [createOrAdd])

  useEffect(() => {
    if (createDrawerVisible) {
      // console.log(createOrAddModelsForm.getFieldValue("modelEnter"))
      createOrAddModelsForm.setFieldsValue({ modelEnter: 0, modelOutput: 0 }) // modelOutput
    }
    if(createDrawerVisible && createOrAdd){
      const { modelFileType, operatingEnvironment, total } = PreFillIn
      createOrAddModelsForm.setFieldsValue({
        version: `V${total+1}.0.0`,
        modelFileType: modelFileType,
        operatingEnvironment: operatingEnvironment
      })
    }
    if(createDrawerVisible && noteBookModelFile.string){
      createOrAddModelsForm.setFieldsValue({
        modelFileUpload: noteBookModelFile.string,
      })
      const info = {
        file: {
          name: noteBookModelFile.name,
          status: 'done'
        }
      }
      pretendToUpload(info)
      setModelFileUploadReturnValue(noteBookModelFile.string)
    }
    if (createDrawerVisible) {
      const modelEnv = noteBookModelEnv ? noteBookModelEnv : ''
      createOrAddModelsForm.setFieldsValue({ operatingEnvironment: modelEnv })
    }
  }, [createDrawerVisible])

  const checkShared = (e) => {
    if (e.target.checked) {
      setShareOrNot(true)
    } else {
      setShareOrNot(false)
    }
  }

  const fileRef = useRef()

  const confirm = () => {
    Modal.confirm({
      title: "模型创建成功!是否确认共享您的模型？",
      icon: <ExclamationCircleOutlined />,
      content: "模型共享后，其他用户可以在共享中心查看、测试、克隆您的模型。",
      okText: "确认共享",
      cancelText: "取消共享",
      onOk: () => {
        info()
      },
      onCancel: () => {
        setShareOrNot(false)
        enterSubmit(false)
      },
    })
  }

  const info = () => {
    Modal.confirm({
      title: "您的“模型共享”申请已提交审批，审批通过后“模型共享”才会生效。",
      icon: <InfoCircleOutlined />,
      content: "您可以在审批中心查看审批进度，点击下方按钮跳转到审批中心。",
      okText: "审批中心",
      cancelText: "知道了",
      onOk: () => {
        const origin = window.location.origin
        const path = "/team/approvalCenter"
        const realUrl = origin + path
        window.open(realUrl)
        enterSubmit(true)
      },
      onCancel: () => {
        enterSubmit(true)
      },
    })
  }

  // 确定是否共享
  const enterWhetherSubmit = () => {
    if (shareOrNot) {
      enterSubmit(true)
    } else {
      enterSubmit()
    }
  }

  // 创建调用
  const enterSubmit = (visibleNew) => {
    if (createOrAdd) {
      enterAddSubmit(visibleNew)
      return
    }
    createOrAddModelsForm
      .validateFields()
      .then((value) => {
        const {
          name: modelName,
          label,
          modelFileType1: packageType,
          introduction: intro,
          scenes: applicationScene,
          case: realCase,
          classification: categoryId,
          modelOutput: outputType,
          modelOutputText: testingOutput,
          operatingEnvironment: runtimeEnv,
          executableFilename,
        } = value
        let needReturn = false
        if (inputTypesArr.length === 0) {
          message.warning("模型输入至少要有一项")
          return
        }

        const testingInputObj = {}
        const testingInputOriginObj = {}
        const inputTypes = inputTypesArr.map((item) => {
          const { name, type, required } = item
          if (type === "string") {
            if (!item.stringContent) {
              needReturn = true
            }
            testingInputObj[name] = item.stringContent
          } else {
            if (!item.fileContent) {
              needReturn = true
            }
            testingInputObj[name] = item.fileContent
            testingInputOriginObj[name] = inputFileListObjRef.current[name][0].name

          }
          return {
            name,
            type,
            required,
          }
        })

        createOrAddAModelOrVersionApi
          .createModel({
            inputTypes,
            testingInput: JSON.stringify(testingInputObj),
            testingInputOrigin:JSON.stringify(testingInputOriginObj),
            modelName,
            packageType,
            label,
            intro,
            applicationScene,
            realCase,
            coverUrl: coverUploadReturnVal, // 模型封面
            oedUrl: effectUploadReturnVal, // 效果演示
            location: modelFileUploadReturnValue, // 模型文件
            visible: visibleNew ? "Public" : "Private", // 是否共享
            fileFrom: "Upload",
            categoryId, // 分类Id
            outputType,
            testingOutput,
            runtimeEnv,
            executableFilename,
          })
          .then((res) => {
            setState({ drawer: undefined, loadPage: undefined })
            resetUploadReturnVal()
            createOrAddModelsForm.resetFields()
            changeCreateDrawerVisible()
            resetListRefAndInputTypes()
            if (shareOrNot) {
              if (myModulePermissionList.includes("share_model")) {
                showModel2(() => {
                  createOrAddAModelOrVersionApi
                    .decideToShare({ editionId: res.data, sharingFlag: true })
                    .then((res) => {
                      message.success("模型共享成功")
                    })
                })
              } else if (myModulePermissionList.includes("apply_share_model")) {
                showModel2(() => {
                  createOrAddAModelOrVersionApi
                    .decideToShare({ editionId: res.data, sharingFlag: true })
                    .then((res) => {
                      showModel4()
                      // message.success("创建成功！")
                    })
                })
              } else {
                showModel3()
              }
            } else {
              message.success("创建成功！")
            }
          })
          .catch((err) => {
            message.error(err.message)
          })
      })
      .catch((err) => {
        pass()
      })
  }

  // 添加 调用
  const enterAddSubmit = (visibleNew) => {
    createOrAddModelsForm
      .validateFields()
      .then((value) => {
        const {
          version: versionName,
          modelFileType: packageType,
          illustrate: intro,
          operatingEnvironment: runtimeEnv,
          executableFilename,
          modelOutput: outputType,
          modelOutputText: testingOutput,
        } = value

        let needReturn = false
        if (inputTypesArr.length === 0) {
          message.warning("模型输入至少要有一项")
          return
        }

        const testingInputObj = {}
        const testingInputOriginObj = {}
        const inputTypes = inputTypesArr.map((item) => {
          const { name, type, required } = item
          if (type === "string") {
            if (!item.stringContent) {
              needReturn = true
            }
            testingInputObj[name] = item.stringContent
          } else {
            if (!item.fileContent) {
              needReturn = true
            }
            testingInputObj[name] = item.fileContent
            testingInputOriginObj[name] = inputFileListObjRef.current[name][0].name
          }
          return {
            name,
            type,
            required,
          }
        })

        createOrAddAModelOrVersionApi
          .addVersion({
            inputTypes,
            testingInput: JSON.stringify(testingInputObj),
            testingInputOrigin:JSON.stringify(testingInputOriginObj),
            versionName,
            packageType,
            intro,
            location: modelFileUploadReturnValue,
            visible: visibleNew ? "Public" : "Private", // 是否共享
            fileFrom: "Upload",
            executableFilename,
            runtimeEnv,
            outputType,
            testingOutput,
            packageId,
          })
          .then((res) => {
            resetUploadReturnVal()
            createOrAddModelsForm.resetFields()
            resetListRefAndInputTypes()
            changeCreateDrawerVisible()
            if (renewList) {
              renewList()
            }

            if (shareOrNot) {
              createOrAddAModelOrVersionApi
                .decideToShare({ editionId: res.data, sharingFlag: true })
                .then((res) => {
                  message.success("创建版本成功！")
                })
            } else {
              message.success("创建版本成功！")
            }
          })
      })
      .catch((err) => {
        pass()
      })
  }

  // 是否通过校验
  const pass = () => {
    setConfigureHeight("auto")
    if (createOrAdd) {
      setCreateHeight(450)
      createOrAddHeight = 450
    } else {
      setCreateHeight(1050)
      createOrAddHeight = 1050
    }
  }

  const resetUploadReturnVal = () => {
    setModelFileFileList([])
    setModelFileUploadReturnValue("")
    setCoverList([])
    setCoverUploadReturnVal("")
    setEffectList([])
    setEffectUploadReturnVal("")
  }

  // 模型文件上传
  const handleModelFileUpload = (info) => {
    const { file, fileList } = info
    if (fileList.length) {
      createOrAddAModelOrVersionApi.upLoda({ file: file.originFileObj }).then((res) => {
        const [uploadReturnVal] = res.data
        file["status"] = "done"
        setModelFileFileList([file])
        setModelFileUploadReturnValue(uploadReturnVal)
      })
    } else {
      // fileList 字段完全受控， status 状态被写死 done
      // 所以，当 info.fileList.length === 0 时候 代表执行了删除操作
      // 此时我们要清空 fileList 字段对应的数组
      // 以及上传拿到的 upload return value
      // 并且将表单 中上传状态 改变为 undefined (因为该选项是必选项)
      setModelFileFileList([])
      setModelFileUploadReturnValue("")
      createOrAddModelsForm.setFieldsValue({ modelFileUpload: undefined })
    }
  }

  // 模型假装上传文件( 发布模型专用)
  const pretendToUpload = (info) => {
    const { file } = info
    setModelFileFileList([file])
  }

  // 模型封面上chuan
  const handleCoverUpload = ({ fileList: newFileList, file }) => {
    console.log(file)
    if (newFileList.length) {
      createOrAddAModelOrVersionApi.upLoda({ file: file.originFileObj }).then((res) => {
        const [uploadReturnVal] = res.data
        file["status"] = "done"
        setCoverList([file])
        setCoverUploadReturnVal(uploadReturnVal)
      })
    } else {
      setCoverList([])
      setCoverUploadReturnVal("")
    }
  }

  const customRequest = () => {
    createOrAddAModelOrVersionApi.upLoda({ file: fileRef.current }).then((res) => {
      const [uploadReturnVal] = res.data
      fileRef.current["status"] = "done"
      setCoverList([fileRef.current])
      setCoverUploadReturnVal(uploadReturnVal)
    })
  }
  const beforeUpload = (file) => {
    fileRef.current = file
    return true
  }

  const handleEffectUpload = (info) => {
    const { file, fileList } = info
    if (fileList.length) {
      createOrAddAModelOrVersionApi.upLoda({ file: file.originFileObj }).then((res) => {
        const [uploadReturnVal] = res.data
        file["status"] = "done"
        setEffectList([file])
        setEffectUploadReturnVal(uploadReturnVal)
      })
    } else {
      setEffectList([])
      setEffectUploadReturnVal("")
    }
  }

  // 选择标签
  const handleChange = (value) => {
    // console.log(`selected ${value}`);
  }

  useEffect(() => {
    let drawer = new URLSearchParams(window.location.search).get("drawer")
    if (drawer == "true" && loadPage == "1" && !clicksViewFlg) {
      console.log(createDrawerVisible)
      setCreateDrawerVisible(true)
    }
  }, [])

  const inputFileListObjRef = useRef({
    "arg0":[]
  })
  const uploadInputFileListInfoRef = useRef({
    "arg0":[]
  })

  const resetListRefAndInputTypes = ()=>{
    uploadInputFileListInfoRef.current = {
      "arg0":[]
    }
    uploadInputFileListInfoRef.current = {
      "arg0":[]
    }
    while (inputTypesArr.length){
      inputTypesArr.pop()
    }
    inputTypesArr[0] = {
      name: "arg0",
      type: "string",
      required: true,
      stringContent: "",
      fileContent: "",
      fileList: [],
    }
  }

  const update = useUpdate()

  const customRequestWithInput = (item) => {

    return ({file}) => {

      const size = file.size //总大小
      const shardSize = 2 * 1024 * 1024 //以2MB为一个分片
      let shardCount = Math.ceil(size / shardSize) //总片数
      shardCount = shardCount === 0 ? 1 : shardCount

      const newFileName = nanoid() + file.name.slice(file.name.lastIndexOf('.'))

      uploadInputFileListInfoRef.current[item.name]=[{totalSize:file.size,completeSize:0}]

      for (let i = 0; i < shardCount; ++i) {
        //计算每一片的起始与结束位置
        let start = i * shardSize
        let end = Math.min(size, start + shardSize)

        const datafile = file.slice(start, end)
        const path = `model_inputs/demo`
        const index  = i+1
        const total = shardCount
        const generateName = newFileName;
        (function(totalSize){
          FileApi.uploadBigFiles({datafile,path,index,total,generateName}).then((res)=>{
            if(res.data.status ==='over'){
              inputFileListObjRef.current[item.name].push(file)
              item.fileContent = res.data.filename
              uploadInputFileListInfoRef.current[item.name] = []
              update()
            }else{
              uploadInputFileListInfoRef.current[item.name][0].completeSize+=totalSize
              update()
            }
          })
        })(end-start)
      }
    }
  }

  const onRemove = (item) => {
    return (file) => {
      const fileList = inputFileListObjRef.current[item.name]
      inputFileListObjRef.current[item.name] = fileList.filter(
        (fileItem) => fileItem.uid !== file.uid
      )
      item.fileContent = ""
    }
  }
  const beforeUploadWithInput = (item) => {
    return (file) => {
      const itemArr = uploadInputFileListInfoRef.current[item.name]
      if(Array.isArray(itemArr) && itemArr.length){
        message.warning('当前有文件正在上传')
        return Upload.LIST_IGNORE
      }
      const fileList = inputFileListObjRef.current[item.name]
      if (fileList.length >= MAX_COUNT) {
        message.warning("上传文件数量超过限制")
        return Upload.LIST_IGNORE
      }
      return true
    }
  }

  return (
    <div>
      <Drawer
        className="createmodel-Drawer"
        visible={createDrawerVisible}
        keyboard={true}
        maskClosable={false}
        closable={false}
        width={800}
      >
        <div className="drawer-header">
          <div className="dh-name">
            <span>模型管理</span> /{" "}
            {createOrAdd === 0 ? "创建模型" : "添加模型版本"}
          </div>
          <div className="dh-option">
            <div className="dh-option-r">
              <Checkbox onChange={checkShared} checked={shareOrNot}>
                模型共享
              </Checkbox>
              <Tooltip
                placement="bottom"
                title={
                  "模型共享后，其他用户可以在共享中心查看、测试、克隆你的模型。"
                }
              >
                <p>
                  <QuestionOutlined />
                </p>
              </Tooltip>
            </div>
            <div className="dh-option-m"></div>
            <div className="dh-option-l">
              <Button
                onClick={() => {
                  changeCreateDrawerVisible()
                  resetListRefAndInputTypes()
                  setState({ drawer: undefined, loadPage: undefined })
                }}
              >
                取消
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  enterWhetherSubmit()
                }}
              >
                确认
              </Button>
            </div>
          </div>
        </div>

        <Form
          className="createOrAddModelsForm"
          form={createOrAddModelsForm}
          labelAlign={"right"}
          labelCol={{
            span: 5,
          }}
          wrapperCol={{
            span: 17,
            offset: 1,
          }}
          layout="horizontal"
        >
          <div
            className="create shared"
            style={{ height: `${createHeight}px` }}
          >
            <div
              className="create-header"
              onClick={() => {
                if (createHeight === 40) setCreateHeight(createOrAddHeight)
                else setCreateHeight(40)
              }}
            >
              <span
                style={{
                  transform: `rotate(${createHeight === 40 ? 0 : 90}deg)`,
                }}
              >
                <CaretRightOutlined />
              </span>
              <span>&nbsp;&nbsp;&nbsp;&nbsp;模型信息</span>
            </div>

            {createOrAdd === 0 ? (
              <div className="model-info">
                <Row justify={"center"}>
                  <Col span={16}>
                    <Form.Item
                      label={"模型名称"}
                      name={"name"}
                      rules={[{ required: true }]}
                    >
                      <Input placeholder="请输入模型名称" />
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify={"center"}>
                  <Col span={16}>
                    <Form.Item label={"模型封面"} name={"uploadImg"}>
                      <ImgCrop
                        rotate
                        aspect={9 / 5}
                        // onModalOK={}
                      >
                        <Upload
                          accept="image/*"
                          multiple={true}
                          fileList={coverList}
                          // onChange={handleCoverUpload}
                          customRequest={customRequest}
                          beforeUpload={beforeUpload}
                          onRemove={() => {
                            // console.log("Remove GO！")
                            setCoverList([])
                            setCoverUploadReturnVal("")
                          }}
                          listType="picture"
                        >
                          <Button icon={<UploadOutlined />}>上传封面</Button>
                        </Upload>
                      </ImgCrop>
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item
                      label={"模型简介"}
                      name={"introduction"}
                      rules={[{ required: true }]}
                    >
                      <TextArea
                        showCount
                        maxLength={1024}
                        placeholder="请输入简介"
                        autoSize={{
                          minRows: 3,
                          maxRows: 3,
                        }}
                      />
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item
                      label={"适用场景"}
                      name={"scenes"}
                      rules={[{ required: true }]}
                    >
                      <TextArea
                        showCount
                        maxLength={1024}
                        placeholder="请输入场景"
                        autoSize={{
                          minRows: 3,
                          maxRows: 3,
                        }}
                      />
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item label={"落地案例"} name={"case"}>
                      <TextArea
                        showCount
                        maxLength={1024}
                        placeholder="请输入案例"
                        autoSize={{
                          minRows: 3,
                          maxRows: 3,
                        }}
                      />
                    </Form.Item>
                  </Col>
                </Row>

                {/*<Row justify="center">
                  <Col span={16}>
                    <Form.Item label={"效果演示"} name={"effectUpload"}>
                      <Upload
                        accept="video/*"
                        listType="picture"
                        customRequest={() => {}}
                        beforeUpload={() => true}
                        multiple={true}
                        fileList={effectList}
                        onChange={(e) => handleEffectUpload(e)}
                      >
                        <Button icon={<UploadOutlined />}>效果演示</Button>
                      </Upload>
                    </Form.Item>
                  </Col>
                </Row>*/}

                <Row justify="center" className="modelFileType">
                  <Col span={16}>
                    <Form.Item
                      label={"模型文件类型"}
                      name={"modelFileType1"}
                      rules={[{ required: true }]}
                    >
                      <Radio.Group>
                        <Radio value={"EXECUTABLE"}>可执行文件</Radio>
                        {/* <Radio value={"SOURCE"}>源代码</Radio> */}
                      </Radio.Group>
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item
                      label={"模型文件"}
                      name={"modelFileUpload"}
                      rules={[{ required: true, message: "请选择模型文件" }]}
                    >
                      <Upload
                        customRequest={() => {}}
                        beforeUpload={() => true}
                        multiple={true}
                        fileList={modelFileFileList}
                        onChange={(e) => handleModelFileUpload(e)}
                        onRemove={() => {
                          if(noteBookModelFile.string){
                            message.warning("不可修改模型文件！")
                          }
                          return false
                        }}
                      >
                        <Button icon={<UploadOutlined />} disabled={noteBookModelFile.string? true: false} >上传模型文件</Button>
                      </Upload>
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item
                      label={"分类"}
                      name={"classification"}
                      rules={[{ required: true }]}
                    >
                      <Select placeholder="请选择分类" allowClear>
                        {category.map((item, index) => (
                          <Option value={item.cateId} key={item.cateId}>
                            {item.cateName}
                          </Option>
                        ))}
                      </Select>
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item label={"标签"} name={"label"}>
                      <Select
                        mode="tags"
                        style={{
                          width: "100%",
                        }}
                        placeholder="请输入标签(回车键确认标签)"
                        onChange={handleChange}
                      >
                        {children}
                      </Select>
                    </Form.Item>
                  </Col>
                </Row>
              </div>
            ) : (
              <div className="version-info">
                <Row justify={"center"}>
                  <Col span={16} className="add-version">
                    <Form.Item
                      label={"版本号"}
                      name={"version"}
                      rules={[{ required: true }]}
                    >
                      <Input placeholder="请输入版本号" />
                    </Form.Item>
                    <Tooltip
                      placement="top"
                      title={
                        "版本号的格式建议为 “x.x.x” 或 “x.x.x_y”，其中 x 为数字 y 为字符串，如：1.0.0、1.0.0_alpha_dev1。"
                      }
                    >
                      <span className="model-name-tip">
                        <QuestionCircleOutlined />
                      </span>
                    </Tooltip>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item label={"版本说明"} name={"illustrate"}>
                      <TextArea
                        showCount
                        maxLength={1024}
                        placeholder="请输入版本说明"
                        autoSize={{
                          minRows: 3,
                          maxRows: 3,
                        }}
                      />
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center" className="modelFileType">
                  <Col span={16}>
                    <Form.Item
                      label={"模型文件类型"}
                      name={"modelFileType"}
                      rules={[{ required: true }]}
                    >
                      <Radio.Group>
                        <Radio value={"EXECUTABLE"}>可执行文件</Radio>
                        {/* <Radio value={"SOURCE"}>源代码</Radio> */}
                      </Radio.Group>
                    </Form.Item>
                  </Col>
                </Row>

                <Row justify="center">
                  <Col span={16}>
                    <Form.Item
                      label={"模型文件"}
                      name={"modelFileUpload"}
                      rules={[{ required: true, message: "请选择模型文件" }]}
                    >
                      <Upload
                        customRequest={() => {}}
                        beforeUpload={() => true}
                        multiple={true}
                        fileList={modelFileFileList}
                        onChange={(e) => handleModelFileUpload(e)}
                      >
                        <Button icon={<UploadOutlined />}>上传模型文件</Button>
                      </Upload>
                    </Form.Item>
                  </Col>
                </Row>
              </div>
            )}
          </div>
          <div className="configure shared" style={{ height: configureHeight }}>
            <div
              className="configure-header"
              onClick={() => {
                if (configureHeight === 40) setConfigureHeight("auto")
                else setConfigureHeight(40)
              }}
            >
              <span
                style={{
                  transform: `rotate(${configureHeight === 40 ? 0 : 90}deg)`,
                }}
              >
                <CaretRightOutlined />
              </span>
              <span>&nbsp;&nbsp;&nbsp;&nbsp;运行环境配置</span>
            </div>

            <Row justify={"center"}>
              <Col span={16}>
                <Form.Item
                  label={"启动文件"}
                  name={"executableFilename"}
                  rules={[{ required: true }]}
                >
                  <Input placeholder="请输入可执行文件名称" />
                </Form.Item>
              </Col>
            </Row>

            <Row justify="center">
              <Col span={16}>
                <Form.Item
                  label={"运行环境"}
                  name={"operatingEnvironment"}
                  rules={[{ required: true }]}
                >
                  {noteBookModelEnv ? <Input disabled /> :
                    <Select placeholder="请选择运行环境" allowClear>
                      <Option value="Python 3.8">Python 3.8</Option>
                      <Option value="Python 3.9">Python 3.9</Option>
                      <Option value="Python 3.10">Python 3.10</Option>
                      <Option value="Java 8">Java 8</Option>
                      <Option value="Java 11">Java 11</Option>
                      <Option value="Java 17">Java 17</Option>
                      {/* <Option value="GLibC">GLibC</Option> */}
                    </Select>}
                </Form.Item>
              </Col>
            </Row>

            <Row
              style={{ paddingLeft: 130, marginBottom: 10 }}
              className="modelEnter"
            >
              <span style={{ color: "#ff4d4f", marginRight: 5 }}>*</span>
              <span>定义模型参数:</span>
            </Row>

            <div className="input-list-container">
              <Row className="input-list-header">
                <span className={"arg-name"}>参数名</span>
                <span className={"arg-type"}>参数类型</span>
                <span className={"arg-test"}>参数示例</span>
              </Row>

              {inputTypesArr.map((item, index) => {
                return (
                  <Row className={"input-list-item"} key={index}>
                    <span className={"arg-name"}>{item.name}</span>
                    <span className={"arg-type"}>
                      <Select
                        value={item.type}
                        onChange={(value) => {
                          inputTypesArr[index].type = value
                          if (value === "string") {
                            item.fileContent = ""
                            item.fileList = []
                          } else {
                            item.stringContent = ""
                          }
                        }}
                      >
                        <Option value={"string"}>文本</Option>
                        <Option value={"file"}>文件</Option>
                      </Select>
                    </span>
                    <span className={"arg-test"}>
                      {item.type === "string" ? (
                        <Input
                          value={item.stringContent}
                          onChange={(ev) => {
                            item.stringContent = ev.target.value
                          }}
                          style={{ width: "310px" }}
                        />
                      ) : (
                        <Upload
                          customRequest={customRequestWithInput(item)}
                          beforeUpload={beforeUploadWithInput(item)}
                          fileList={inputFileListObjRef.current[item.name]}
                          onRemove={onRemove(item)}
                          maxCount={MAX_COUNT}
                        >
                          {(inputFileListObjRef.current[item.name].length < MAX_COUNT) ? (
                            <Button
                              style={{ width: "310px" }}
                              icon={<UploadOutlined />}
                            >
                              上传文件
                            </Button>
                          ) : null}
                          {
                            Array.isArray(uploadInputFileListInfoRef.current[item.name]) && uploadInputFileListInfoRef.current[item.name].length>0 && uploadInputFileListInfoRef.current[item.name].map(info=>{
                              return <Progress style={{width:"80%"}} percent={Math.ceil((info.completeSize/info.totalSize)*100)} status="active" />
                            })
                          }
                        </Upload>
                      )}

                      <MinusCircleOutlined
                        onClick={()=>{
                          handleDecreaseInputTypeClick(index)
                        }}
                        style={{
                          marginLeft: 15,
                          cursor: "pointer",
                          marginTop: 8,
                        }}
                      />
                    </span>
                  </Row>
                )
              })}

              <Row className="input-list-footer">
                <Button
                  onClick={handleAddInputTypeClick}
                  style={{ width: "90%", margin: "0 auto" }}
                  icon={<PlusCircleOutlined />}
                >
                  添加参数
                </Button>
              </Row>
            </div>

            <Row justify="center" className="modelOutput">
              <Col span={16}>
                <Form.Item
                  label={"模型输出"}
                  name={"modelOutput"}
                  rules={[{ required: true }]}
                >
                  <Radio.Group>
                    <Radio value={0}>文本</Radio>
                    <Radio value={1}>文件</Radio>
                  </Radio.Group>
                </Form.Item>
              </Col>
            </Row>

            <Row justify="center" className="modelOutput-text">
              <Col span={16}>
                <Form.Item  hidden label={" "} name={"modelOutputText"}>
                  <TextArea
                    showCount
                    maxLength={1024}
                    placeholder="请提供模型输出的示例"
                    autoSize={{
                      minRows: 3,
                      maxRows: 3,
                    }}
                  />
                </Form.Item>
              </Col>
            </Row>
          </div>
        </Form>
      </Drawer>
    </div>
  )
}

export default CreateModel
