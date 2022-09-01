import React, { useState, useEffect, useContext } from "react"
import {
  Button,
  Space,
  TimePicker,
  DatePicker,
  Select,
  Modal,
  Form,
  message,
} from "antd"
import styles from "./planModal.module.less"
import moment from "moment"
import pipeLineApi from "idpServices/pipelineApi"
import { CanvasToolBarContext } from "../../pages/dag-canvas/canvas-toolbar"
const { Option } = Select

interface Props {
  visible: boolean
  experimentId: any
  cronConfig: any
}

// // 调用接口校验
// getCronTime = (cronText) => {
//     this.props.actions.getCron(cronText)
// }

export const PlanModalSimple: React.FC<Props> = (props) => {
  const { setExperimentPlan, setDrawerVisible, setModalVisible } =
    useContext(CanvasToolBarContext)
  const { visible } = props
  const [hoursVisible, setHoursVisible] = useState<boolean>(true)
  const [weekVisible, setWeekVisible] = useState<boolean>(false)
  const [dayVisible, setDayVisible] = useState<boolean>(false)
  const [metric] = useState<string>("day")

  const [form] = Form.useForm()
  const selectDateTimeChange = (value) => {
    switch (value) {
      case "minute":
        setWeekVisible(false)
        setDayVisible(false)
        setHoursVisible(false)
        break
      case "hour":
        setWeekVisible(false)
        setDayVisible(false)
        setHoursVisible(false)
        break
      case "day":
        setHoursVisible(true)
        setWeekVisible(false)
        setDayVisible(false)
        break
      case "week":
        setWeekVisible(true)
        setHoursVisible(true)
        setDayVisible(false)
        break

      case "month":
        setDayVisible(true)
        setHoursVisible(true)
        setWeekVisible(false)
        break
    }
  }

  const onOk = () => {
    // 相当于执行onFinish
    form.submit()
  }
  const onCancel = () => {
    resetAndCloseModal()
  }

  const getCronItemByType = (type) => {
    let exp = props?.cronConfig?.cronExpression?.split(" ")
    let types = ["minute", "hour", "day", "month", "week"]
    if (exp) {
      let item =
        exp[
          types.findIndex(function (item) {
            return item === type
          })
        ]
      if (/minute|hour/.test(type) && Number(item)) {
        return formatNum(item)
      }
      return item
    }
    return false
  }
  /**
   * 格式化0~9的数字
   * @param num
   */
  const formatNum = (num) => {
    if (num < 10 && num > -1) {
      return "0" + num
    }
    return num
  }

  //
  const defaultConfig = {
    //开始日期
    startDate: props.cronConfig?.cronStartDate
      ? moment(props.cronConfig?.cronStartDate)
      : moment(new Date(), "YYYY-MM-DD"),
    //结束日期
    endDate: props.cronConfig?.cronEndDate
      ? moment(props.cronConfig?.cronEndDate)
      : moment(
          new Date(new Date().getTime() + 24 * 60 * 60 * 1000),
          "YYYY-MM-DD"
        ),
    //开始时间
    "plan-start-time": moment(
      props.cronConfig?.cronStartTime
        ? props.cronConfig.cronStartTime
        : "00:00",
      "HH:mm"
    ),
    //结束时间
    "plan-end-time": moment(
      props.cronConfig?.cronEndTime ? props.cronConfig?.cronEndTime : "00:00",
      "HH:mm"
    ),

    //计划分钟设置
    "plan-execution-interval-time":
      getCronItemByType("minute") &&
      props.cronConfig?.cronExpression?.split(" ")[0] !== "*"
        ? moment(
            getCronItemByType("hour") + ":" + getCronItemByType("minute"),
            "HH:mm"
          )
        : moment("00:00", "HH:mm"),
    //计划时间单位设置
    "plan-execution-interval-metric": metric,
    //一个月内的某一天
    "plan-execution-interval-day":
      getCronItemByType("day") &&
      props.cronConfig?.cronExpression?.split(" ")[2] !== "*"
        ? props.cronConfig?.cronExpression?.split(" ")[2]
        : 1,
    //一周之中的周几
    "plan-execution-interval-week":
      getCronItemByType("week") &&
      props.cronConfig?.cronExpression?.split(" ")[4] !== "*"
        ? props.cronConfig?.cronExpression?.split(" ")[4]
        : 1,
  }

  // console.log(defaultConfig, "------")

  const onFinish = (data) => {
    console.log(data,'------')

    if(!(data.startDate && data.endDate && data['plan-start-time'] && data['plan-end-time'])){
      message.warning('计划开始时间和计划结束时间是必填的')
      return
    }

    let minute = /day|week|month/.test(data["plan-execution-interval-metric"])
      ? data["plan-execution-interval-time"].minute()
      : /hour/.test(data["plan-execution-interval-metric"])
      ? 1
      : "*"
    let hour = /day|week|month/.test(data["plan-execution-interval-metric"])
      ? data["plan-execution-interval-time"].hour()
      : "*"
    let day = /month/.test(data["plan-execution-interval-metric"])
      ? data["plan-execution-interval-day"]
      : "*"
    let month = "*"
    let week = /week/.test(data["plan-execution-interval-metric"])
      ? data["plan-execution-interval-week"]
      : "*"
    let cronText = minute + " " + hour + " " + day + " " + month + " " + week
    const cronConfig = {
      cronExpression: cronText,
      cronStartDate: data.startDate.format("YYYY-MM-DD"),
      cronEndDate: data.endDate.format("YYYY-MM-DD"),
      cronStartTime: data["plan-start-time"].format("HH:mm"),
      cronEndTime: data["plan-end-time"].format("HH:mm"),
      cronType: "simple",
    }

    pipeLineApi
      .jobCreatePlan({
        jobId: props.experimentId,
        cronConfig,
      })
      .then(function (data) {
        setExperimentPlan(cronConfig)
        message.success("创建成功！")
      })
    resetAndCloseModal()
  }

  const resetAndCloseModal = () => {
    // form.resetFields();
    /*    setWeekVisible(false)
    setDayVisible(false)
    setHoursVisible(false)*/
    setModalVisible(false)
  }

  const openSinorPanel = (data) => {
    onCancel()
    // setSinorPanelVisible(!sinorPanelVisible)
    setDrawerVisible(true)
  }
  // const refCronGenerator = React.createRef()
  useEffect(() => {
    if (props?.cronConfig?.cronExpression) {
      let rules = props.cronConfig.cronExpression.split(" ")
      let type = "minute"

      //    格式: {分} {时} {日} {月} {周} ”
      //    5 * * * * Command              每小时的第5分钟执行一次命令
      //    30 18 * * * Command          指定每天下午的 6:30 执行一次命令
      //    30 7 8 * * Command           指定每月8号的7：30分执行一次命令
      //    30 6 * * 0 Command           指定每星期日的6:30执行一次命令

      //    5,12，18 * * * Command       每天的下午6点,5 min、12 min时执行命令。
      //    0 */4 * * *    Command       每四个小时执行一个任务
      //    */5 * * * *    Command       每5分钟执行一个任务
      for (let i = 0; i < rules.length; i++) {
        if (i === 4 && rules[i] !== "*") {
          type = "week"
        }
        if (
          i === 3 &&
          rules[i] === "*" &&
          rules[i + 1] === "*" &&
          rules[i - 1] !== "*"
        ) {
          type = "month"
        }

        if (i === 0 && rules[i] !== "*") {
          if (rules[i + 1] === "*") {
            type = "hour"
          } else {
            type = "day"
          }
        }
      }
      form.setFieldsValue({
        "plan-execution-interval-metric": type,
      })
      selectDateTimeChange(type)

      const newConfig = {
        //开始日期
        startDate: props.cronConfig?.cronStartDate
          ? moment(props.cronConfig?.cronStartDate)
          : moment(new Date(), "YYYY-MM-DD"),
        //结束日期
        endDate: props.cronConfig?.cronEndDate
          ? moment(props.cronConfig?.cronEndDate)
          : moment(
              new Date(new Date().getTime() + 24 * 60 * 60 * 1000),
              "YYYY-MM-DD"
            ),
        //开始时间
        "plan-start-time": moment(
          props.cronConfig?.cronStartTime
            ? props.cronConfig.cronStartTime
            : "00:00",
          "HH:mm"
        ),
        //结束时间
        "plan-end-time": moment(
          props.cronConfig?.cronEndTime
            ? props.cronConfig?.cronEndTime
            : "00:00",
          "HH:mm"
        ),

        //计划分钟设置
        "plan-execution-interval-time":
          getCronItemByType("minute") &&
          props.cronConfig?.cronExpression?.split(" ")[0] !== "*"
            ? moment(
                getCronItemByType("hour") + ":" + getCronItemByType("minute"),
                "HH:mm"
              )
            : moment("00:00", "HH:mm"),
        //一个月内的某一天
        "plan-execution-interval-day":
          getCronItemByType("day") &&
          props.cronConfig?.cronExpression?.split(" ")[2] !== "*"
            ? props.cronConfig?.cronExpression?.split(" ")[2]
            : 1,
        //一周之中的周几
        "plan-execution-interval-week":
          getCronItemByType("week") &&
          props.cronConfig?.cronExpression?.split(" ")[4] !== "*"
            ? props.cronConfig?.cronExpression?.split(" ")[4]
            : 1,
      }

      form.setFieldsValue({
        ...newConfig,
      })
    }
  }, [props?.cronConfig?.cronExpression, visible])
  return (
    <>
      <Modal
        visible={visible}
        title="设置执行计划"
        width="450px"
        // okText='确定'
        // cancelText='取消'
        // onOk={onOk}
        onCancel={onCancel}
        footer={[
          <Button
            type="link"
            style={{ float: "left" }}
            onClick={openSinorPanel}
          >
            高级设置
          </Button>,
          <Button key="back" onClick={onCancel}>
            取消
          </Button>,
          <Button key="submit" type="primary" onClick={onOk}>
            确定
          </Button>,
        ]}
        children={
          <div className={styles.dateSelectBox}>
            <Form
              name="planSetting"
              form={form}
              onFinish={onFinish}
              initialValues={{ ...defaultConfig }}
            >
              <div className={styles.dateSelectItem}>
                <Space>
                  <Form.Item className={styles.formItem}>
                    <div style={{ lineHeight: 1.5715 }}>计划开始时间：</div>
                  </Form.Item>
                  <Form.Item
                    name="startDate"
                    className={styles.formItem}
                    //  rules={[{ required: true, message: '必须选择开始时间!' }]}
                  >
                    <DatePicker
                      format={"YYYY-MM-DD"}
                      // defaultValue={
                      //     props.cronConfig?.cronStartDate ?
                      //     moment(props.cronConfig?.cronStartDate) :
                      //     moment((new Date()), 'YYYY-MM-DD')
                      // }
                      // value={
                      //     props.cronConfig?.cronStartDate ?
                      //     moment(props.cronConfig?.cronStartDate) :
                      //     moment((new Date()), 'YYYY-MM-DD')
                      // }
                    />
                  </Form.Item>
                  <Form.Item name="plan-start-time" className={styles.formItem}>
                    <TimePicker format="HH:mm" />
                  </Form.Item>
                </Space>
              </div>
              <div className={styles.dateSelectItem}>
                <Space>
                  <Form.Item className={styles.formItem}>
                    <div style={{ lineHeight: 1.5715 }}>每</div>
                  </Form.Item>
                  {/* <Form.Item name="plan-execution-interval-initNumber" className={styles.formItem}>
                                <Select>
                                    {Array.from({ length: 60 }).map((item, index) => {
                                        return <Option value={index + 1}>{index + 1}</Option>
                                    })}
                                </Select>
                            </Form.Item> */}
                  <Form.Item
                    name="plan-execution-interval-metric"
                    className={styles.formItem}
                  >
                    <Select onSelect={(val) => selectDateTimeChange(val)}>
                      <Option value="minute">分钟</Option>
                      <Option value="hour">小时</Option>
                      <Option value="day">天</Option>
                      <Option value="week">周</Option>
                      <Option value="month">月</Option>
                    </Select>
                  </Form.Item>
                  <Form.Item className={styles.formItem}>
                    <div style={{ display: hoursVisible ? "" : "none" }}>
                      的
                    </div>
                  </Form.Item>
                  <div style={{ display: dayVisible ? "block" : "none" }}>
                    <Form.Item
                      name="plan-execution-interval-day"
                      className={styles.formItem}
                    >
                      <Select>
                        {Array.from({ length: 31 }).map((item, index) => {
                          return <Option value={index + 1}>{index + 1}</Option>
                        })}
                      </Select>
                    </Form.Item>
                  </div>
                  <div style={{ display: weekVisible ? "block" : "none" }}>
                    <Form.Item
                      name="plan-execution-interval-week"
                      className={styles.formItem}
                    >
                      <Select>
                        <Option value="1">1</Option>
                        <Option value="2">2</Option>
                        <Option value="3">3</Option>
                        <Option value="4">4</Option>
                        <Option value="5">5</Option>
                        <Option value="6">6</Option>
                        <Option value="7">7</Option>
                      </Select>
                    </Form.Item>
                  </div>
                  <div style={{ display: hoursVisible ? "block" : "none" }}>
                    <Form.Item
                      name="plan-execution-interval-time"
                      className={styles.formItem}
                    >
                      <TimePicker format="HH:mm" />
                    </Form.Item>
                  </div>
                </Space>
              </div>
              <div className={styles.dateSelectItem}>
                <Space>
                  <Form.Item className={styles.formItem}>
                    <div style={{ lineHeight: 1.5715 }}>计划结束时间：</div>
                  </Form.Item>
                  <Form.Item name="endDate" className={styles.formItem}>
                    <DatePicker format={"YYYY-MM-DD"} />
                  </Form.Item>
                  <Form.Item name="plan-end-time" className={styles.formItem}>
                    <TimePicker format="HH:mm" />
                  </Form.Item>
                </Space>
              </div>
            </Form>
          </div>
        }
      />
    </>
  )
}
