// Cron.js
import React, { Dispatch } from "react"
import {
  InputNumber,
  Tabs,
  Radio,
  Checkbox,
  Space,
  Form,
  DatePicker,
  Button,
  message,
} from "antd"
import moment from "moment"
// import { createPlan } from "../../services/graph"
import pipeLineApi from "idpServices/pipelineApi"
import styles from "./cronPanel.module.less"
interface Props {
  experimentId: any
  cronConfig: any
  setExperimentPlan: Dispatch<any>
}

/**
 *CRON表达式生成器
 *@author Shellcoochi
 */

class CronGenerator extends React.Component<Props> {
  state = {
    cronText: this.props.cronConfig.cronExpression || "* * * * *",
    // cronText:  '* * * * *',
    // 常量数组
    cronType: ["minute", "hour", "day", "month", "week"],
    // cronType: ['second', 'minute', 'hour', 'day', 'month', 'week'],

    // 单选按钮的值
    // 如果是1 则代表选的对应的第一行 如果是loop则代表选的第二行  如果是point代表选中第三行
    radioValue: {
      //   second: 1,
      minute: 1,
      hour: 1,
      day: 1,
      month: 1,
      week: 1,
    },

    // 暂时没有用到这个值
    periodValue: {
      //   second: { max: 1, min: 1 },
      minute: { max: 1, min: 1 },
      hour: { max: 1, min: 1 },
      day: { max: 1, min: 1 },
      month: { max: 1, min: 1 },
      week: { max: 1, min: 1 },
    },

    // 用于绑定每种时间对应的第二行的值
    loopValue: {
      //   second: { start: 1, end: 1 },
      minute: {
        start: 1,
        end: 1,
        // default : this.parseCronTable(0, this.props.cronText)
      },
      hour: { start: 1, end: 1 },
      day: { start: 1, end: 1 },
      month: { start: 1, end: 1 },
      week: { start: 1, end: 1 },
    },

    // 暂时没有用到pointValue的值
    pointValue: {
      minute: [],
      hour: [],
      day: [],
      month: [],
      week: [],
    },

    // cronText 以对象的形式展现
    cron: {
      //   second: '*',
      minute: "*",
      hour: "*",
      day: "*",
      month: "*",
      week: "*",
    },
    // 基本用于每种时间的第三行的值
    cronParams: {
      //   second: '*',
      minute: "*",
      hour: "*",
      day: "*",
      month: "*",
      week: "*",
    },
    // 提交给后端后 生成的数组
    cronPreview: [],
    // 开始时间 放在日期选择器中
    startDate: this.props.cronConfig.cronStartDate
      ? this.props.cronConfig.cronStartDate
      : moment(new Date(), "YYYY-MM-DD"),
    // 结束时间 放在日期选择器中
    endDate: this.props.cronConfig.cronEndDate
      ? this.props.cronConfig.cronEndDate
      : moment(
          new Date(new Date().getTime() + 24 * 60 * 60 * 1000),
          "YYYY-MM-DD"
        ),
    // 控制分里面的 指定按钮的value值
    mCheckbox: false,

    // 暂时注释掉 没有用到这个数据
    /*defaultCron: {
      minute: {
        type: null,
        selected: null,
        value: null,
      },
      hour: {
        type: null,
        selected: null,
        value: null,
      },
      day: {
        type: null,
        selected: null,
        value: null,
      },
      month: {
        type: null,
        selected: null,
        value: null,
      },

      week: {
        type: null,
        selected: null,
        value: null,
      },
    },*/
  }
  /**
   * 解析cron
   * @returns {object}
   */
  UNSAFE_componentWillReceiveProps(
    nextProps: Readonly<Props>,
    nextContext: any
  ) {
    const { cronConfig } = nextProps
    this.setState({
      cronText: cronConfig.cronExpression || "* * * * *",
      startDate: cronConfig.cronStartDate
        ? cronConfig.cronStartDate
        : moment(new Date(), "YYYY-MM-DD"),
      endDate: cronConfig.cronEndDate
        ? cronConfig.cronEndDate
        : moment(
            new Date(new Date().getTime() + 24 * 60 * 60 * 1000),
            "YYYY-MM-DD"
          ),
    })

    this.updateState(cronConfig.cronExpression || "* * * * *")
  }

  parseCronTable = (cron: string) => {
    const val = cron.split(" ")
    const indexs = this.state.cronType
    const data: any = {
      minute: {
        type: null,
        selected: null,
        value: null,
      },
      hour: {
        type: null,
        selected: null,
        value: null,
      },
      day: {
        type: null,
        selected: null,
        value: null,
      },
      month: {
        type: null,
        selected: null,
        value: null,
      },

      week: {
        type: null,
        selected: null,
        value: null,
      },
    }
    val.map(function (item, i) {
      if (item.includes("/")) {
        data[indexs[i]].type = "loop"
        data[indexs[i]].selected = "loop"
        data[indexs[i]].value = item.split("/")[1]
      }
      if (item.includes("*")) {
        data[indexs[i]].type = "default"
        data[indexs[i]].selected = 1
        data[indexs[i]].value = "*"
      }
      if (item.includes("?")) {
        data[indexs[i]].type = "undefined"
        data[indexs[i]].selected = 2
        data[indexs[i]].value = "?"
      }
      if (item.includes(",") || Number(item) >= 0) {
        data[indexs[i]].type = "point"
        data[indexs[i]].selected = "point"
        data[indexs[i]].value = item.includes(",")
          ? item.split(",").map((item) => {
              return Number(item)
            })
          : [Number(item)]
      }
      return undefined
    })
    console.log(data, "======")
    // this.setState({'defaultCron': data});
    return data
  }
  componentDidMount(): void {
    this.updateState(this.state.cronText)
  }

  updateState = (cronText: string) => {
    const data = this.parseCronTable(cronText)
    const $this = this
    if (data) {
      // type ,selected ,value
      this.setState(
        {
          radioValue: {
            minute: data.minute.selected,
            hour: data.hour.selected,
            day: data.day.selected,
            month: data.month.selected,
            week: data.week.selected,
          },
          loopValue: {
            minute: {
              start: 1,
              end: data.minute.type === "loop" ? data.minute.value : 1,
            },
            hour: {
              start: 1,
              end: data.hour.type === "loop" ? data.hour.value : 1,
            },
            day: {
              start: 1,
              end: data.day.type === "loop" ? data.day.value : 1,
            },
            month: {
              start: 1,
              end: data.month.type === "loop" ? data.month.value : 1,
            },
            week: {
              start: 1,
              end: data.week.type === "loop" ? data.week.value : 1,
            },
          },
          pointValue: {
            minute: data.minute.type === "point" ? data.minute.value : [],
            hour: data.hour.type === "point" ? data.hour.value : [],
            day: data.day.type === "point" ? data.day.value : [],
            month: data.month.type === "point" ? data.month.value : [],
            week: data.week.type === "point" ? data.week.value : [],
          },
          cron: {
            minute: data.minute.value,
            hour: data.hour.value,
            day: data.day.value,
            month: data.month.value,
            week: data.week.value,
          },
          cronParams: {
            minute: data.minute.value,
            hour: data.hour.value,
            day: data.day.value,
            month: data.month.value,
            week: data.week.value,
          },
        },
        function () {
          $this.forceUpdate()
        }
      )
    }
  }

  /**
   * 生成cron
   * @returns {Promise<void>}
   */
  createCron = async () => {
    const { startDate, endDate } = this.state
    if (!(startDate && endDate)) {
      message.warning("计划开始时间和计划结束时间是必填的")
      return
    }

    let { cronType } = this.state
    const $this = this
    for (let type of cronType) {
      await this.cronGenerator(type)
    }
    let { minute, hour, day, month, week } = this.state.cron
    let cronText = minute + " " + hour + " " + day + " " + month + " " + week

    let hasError = false
    Object.values(this.state.cron).forEach((value) => {
      if (!value) {
        hasError = true
      }
    })
    if (hasError) {
      message.error("当前操作有误 无法生成表达式")
      return
    }

    this.setState(
      {
        cronText,
      },
      () => {
        //create plan
        // @ts-ignore
        const cronConfig = {
          cronType: "advanced",
          cronExpression: cronText,
          cronStartDate:
            typeof startDate === "object"
              ? startDate.format("YYYY-MM-DD")
              : startDate,
          cronEndDate:
            typeof endDate === "object"
              ? endDate.format("YYYY-MM-DD")
              : endDate,
        }
        pipeLineApi
          .jobCreatePlan({
            jobId: this.props.experimentId,
            cronConfig,
            // status: 'Init',
          })
          .then(function (response) {
            $this.setState({
              cronPreview: response.data,
            })
            $this.props.setExperimentPlan(cronConfig)
          })
      }
    )
  }

  /**
   * cron生成器
   * @param type
   */
  cronGenerator = (type) => {
    let srv = this.state.radioValue[type]
    let period = this.state.periodValue[type]
    let loop = this.state.loopValue[type]
    let param = this.state.cronParams[type]
    let data = ""
    switch (srv) {
      case 1:
        data = "*"
        break
      case 2:
        data = "?"
        break
      case "point":
        for (let v of param) {
          data = data + v + ","
        }
        data = data.substring(0, data.length - 1)
        break
      case "period":
        data = period.min + "-" + period.max
        break
      case "loop":
        data = loop.start + "/" + loop.end
        break
      default:
        data = "*"
    }
    this.setState({
      cron: Object.assign(
        {},
        this.state.cron,
        this.cronItemGenerator(type, data)
      ),
    })
  }

  /**
   * 对象生成器
   * @param type
   * @param data
   * @returns { {second: *}|{minute: *}}
   */
  cronItemGenerator = (type, data) => {
    switch (type) {
      //   case 'second': return { second: data }
      case "minute":
        return { minute: data }
      case "hour":
        return { hour: data }
      case "day":
        return { day: data }
      case "month":
        return { month: data }
      case "week":
        return { week: data }
    }
  }

  /**
   * 生成多选框
   * @param col 每行个数
   * @param minNum 最小值
   * @param maxNum 最大值
   * @param key
   */
  // createCheckbox = (key, col, minNum, maxNum) => {
  //   let colArray = []
  //   let rowArray = []
  //   let count = col
  //   let keyNum = minNum
  //   let data =  Object.assign([],this.state.pointValue[key] )
  //   for (minNum; minNum <= maxNum; minNum++) {

  //     let checkbox = <Checkbox key={key + keyNum + Math.random()} defaultChecked={data[0] == minNum} checked={data[0] == minNum} value={minNum}>{this.formatNum(minNum)}</Checkbox>

  //     if(data[0] == minNum){
  //       data.shift()
  //     }

  //     if (count > 0) {
  //       colArray.push(checkbox)
  //       count--
  //       if (minNum === maxNum)rowArray.push(<Col key={key + keyNum} span={24}>{colArray}</Col>)
  //     } else {
  //       rowArray.push(<Col key={key + keyNum} span={24}>{colArray}</Col>)
  //       colArray = []
  //       minNum--
  //       count = col
  //     }
  //     keyNum++
  //   }
  //   return <Row>{rowArray}</Row>
  // }
  createCheckbox = (key, col, minNum, maxNum) => {
    let data = []
    for (minNum; minNum <= maxNum; minNum++) {
      data.push({ label: minNum, value: minNum })
    }
    const comp = (
      <Checkbox.Group
        style={{ width: "100%" }}
        defaultValue={
          this.state.radioValue[key] === "point"
            ? this.state.cronParams[key]
            : []
        }
        value={
          this.state.radioValue[key] === "point"
            ? this.state.cronParams[key]
            : []
        }
        options={data}
        onChange={(e) => this.handleCheckboxChange(e, key)}
      />
    )
    return comp
  }
  /**
   * 格式化0~9的数字
   * @param num
   */
  formatNum = (num) => {
    if (num < 10 && num > -1) {
      return "0" + num
    }
    return num
  }

  handleRadioChange = (e, type) => {
    this.setState({
      radioValue: Object.assign(
        {},
        this.state.radioValue,
        this.cronItemGenerator(type, e.target.value)
      ),
    })
  }

  handleCheckboxChange = (checkedValues, type) => {
    this.setState({
      cronParams: Object.assign(
        {},
        this.state.cronParams,
        this.cronItemGenerator(type, checkedValues)
      ),
      mCheckbox: 1,
    })
  }

  // 从没用到过这个函数
  handlePeriodChange = (e, type, tar) => {
    let data = this.state.periodValue
    data[type] =
      tar === "max"
        ? { max: e, min: data[type].min }
        : { max: data[type].max, min: e }
    this.setState({
      periodValue: Object.assign(
        {},
        this.state.periodValue,
        this.cronItemGenerator(type, data[type])
      ),
    })
  }

  handleLoopChange = (e, type, tar) => {
    let data = this.state.loopValue
    data[type] =
      tar === "start"
        ? { start: e, end: data[type].end }
        : { start: data[type].start, end: e }
    this.setState({
      loopValue: Object.assign(
        {},
        this.state.loopValue,
        this.cronItemGenerator(type, data[type])
      ),
    })
  }

  render() {
    // console.log(this.props.cronConfig.cronExpression,'------')
    const { TabPane } = Tabs
    const { radioValue } = this.state
    const radioStyle = {
      //   display: 'block',
      //   height: '30px',
      lineHeight: "30px",
    }
    // const secondCheckbox = this.createCheckbox('second', 8, 0, 59)
    const minuteCheckbox = this.createCheckbox("minute", 10, 0, 59)
    const hourCheckbox = this.createCheckbox("hour", 10, 0, 23)
    const dayCheckbox = this.createCheckbox("day", 10, 1, 31)
    const monthCheckbox = this.createCheckbox("month", 10, 1, 12)
    const weekCheckbox = this.createCheckbox("week", 7, 1, 7)
    const config = {
      // rules: [{ required: true, message: '必须选择一个日期！' }],
    }
    // const cronDefaultData = this.parseCronTable(this.props.cronConfig.cronExpression);

    return (
      <>
        <Form name="time_related_controls" onFinish={this.createCron}>
          <div className={styles.dateSelectItem}>
            <Space>
              <div style={{ lineHeight: "32px" }}>计划开始时间：</div>
              <Form.Item name="start-date-picker" {...config}>
                <DatePicker
                  format="YYYY-MM-DD"
                  // defaultValue={moment(this.props.cronConfig.cronStartDate,'YYYY-MM-DD')}
                  defaultValue={
                    this.props.cronConfig.cronStartDate
                      ? moment(
                          this.props.cronConfig.cronStartDate,
                          "YYYY-MM-DD"
                        )
                      : moment(new Date(), "YYYY-MM-DD")
                  }
                  value={
                    this.props.cronConfig.cronStartDate
                      ? moment(
                          this.props.cronConfig.cronStartDate,
                          "YYYY-MM-DD"
                        )
                      : moment(new Date(), "YYYY-MM-DD")
                  }
                  onChange={(value) => {
                    this.setState({
                      startDate: value ? value.format("YYYY-MM-DD") : null,
                    })
                  }}
                />
              </Form.Item>
              {/* <TimePicker
                format='HH:mm'
                onChange={(value) =>
                  this.setState({startTime: value})
                }
              /> */}
              <div style={{ lineHeight: "32px" }}> - </div>
              <div style={{ lineHeight: "32px" }}>计划结束时间：</div>
              <Form.Item name="end-date-picker" {...config}>
                <DatePicker
                  defaultValue={
                    this.props.cronConfig.cronEndDate
                      ? moment(this.props.cronConfig.cronEndDate, "YYYY-MM-DD")
                      : moment(
                          new Date(new Date().getTime() + 24 * 60 * 60 * 1000),
                          "YYYY-MM-DD"
                        )
                  }
                  onChange={(value) => {
                    this.setState({
                      endDate: value ? value.format("YYYY-MM-DD") : null,
                    })
                  }}
                  value={
                    this.props.cronConfig.cronEndDate
                      ? moment(this.props.cronConfig.cronEndDate, "YYYY-MM-DD")
                      : moment(
                          new Date(new Date().getTime() + 24 * 60 * 60 * 1000),
                          "YYYY-MM-DD"
                        )
                  }
                />
              </Form.Item>
              {/* <TimePicker
                format='HH:mm'
                onChange={(value) =>
                  this.setState({endTime: value})
                }
              /> */}
            </Space>
          </div>
          <label style={{ color: "rgba(0,0,0,.85)" }}>时间参数：</label>
          <Tabs
            type="card"
            style={{
              height: "maxHeight: calc(100% - 62px);",
              marginTop: "6px",
            }}
          >
            <TabPane tab="分" key="2">
              <Radio.Group
                value={radioValue["minute"]}
                onChange={(e) => this.handleRadioChange(e, "minute")}
              >
                <Radio style={radioStyle} value={1}>
                  每分执行
                </Radio>
                <br />

                <Radio style={radioStyle} value="loop">
                  <InputNumber
                    size="small"
                    min={1}
                    type={"hidden"}
                    style={{ display: "none" }}
                    max={59}
                    defaultValue={1}
                    onChange={(e) =>
                      this.handleLoopChange(e, "minute", "start")
                    }
                  />

                  <Space>
                    每
                    <InputNumber
                      size="small"
                      min={1}
                      max={59}
                      defaultValue={this.state.loopValue.minute.end}
                      onChange={(e) =>
                        this.handleLoopChange(e, "minute", "end")
                      }
                    />
                    分执行一次
                  </Space>
                </Radio>
                <br />
                <Radio
                  style={radioStyle}
                  value="point"
                  autoFocus={true}
                  checked={this.state.mCheckbox}
                >
                  <Space>指定</Space>
                  {minuteCheckbox}
                </Radio>
              </Radio.Group>
            </TabPane>
            <TabPane tab="时" key="3">
              <Radio.Group
                onChange={(e) => this.handleRadioChange(e, "hour")}
                value={radioValue["hour"]}
              >
                <Radio style={radioStyle} value={1}>
                  每小时执行
                </Radio>
                <br />
                <Radio style={radioStyle} value="loop">
                  <InputNumber
                    size="small"
                    min={0}
                    type={"hidden"}
                    style={{ display: "none" }}
                    max={23}
                    defaultValue={1}
                    onChange={(e) => this.handleLoopChange(e, "hour", "start")}
                  />
                  <Space>
                    每
                    <InputNumber
                      size="small"
                      min={1}
                      max={59}
                      defaultValue={this.state.loopValue.hour.end}
                      onChange={(e) => this.handleLoopChange(e, "hour", "end")}
                    />
                    时执行一次
                  </Space>
                </Radio>
                <br />
                <Radio
                  style={radioStyle}
                  value="point"
                  checked={!!this.state.cronParams.hour}
                >
                  <Space>指定</Space>
                  {hourCheckbox}
                </Radio>
              </Radio.Group>
            </TabPane>
            <TabPane tab="日" key="4">
              <Radio.Group
                onChange={(e) => this.handleRadioChange(e, "day")}
                value={radioValue["day"]}
              >
                <Radio style={radioStyle} value={1}>
                  每日执行
                </Radio>
                <br />
                {/*              <Radio style={radioStyle} value={2}>
                不指定
              </Radio>*/}
                <br />
                <Radio style={radioStyle} value="loop">
                  <InputNumber
                    size="small"
                    min={1}
                    type={"hidden"}
                    style={{ display: "none" }}
                    max={31}
                    defaultValue={1}
                    onChange={(e) => this.handleLoopChange(e, "day", "start")}
                  />

                  <Space>
                    每
                    <InputNumber
                      size="small"
                      min={1}
                      max={31}
                      defaultValue={this.state.loopValue.day.end}
                      onChange={(e) => this.handleLoopChange(e, "day", "end")}
                    />
                    日执行一次
                  </Space>
                </Radio>
                <Radio
                  style={radioStyle}
                  value="point"
                  checked={!!this.state.cronParams.day}
                >
                  <Space>指定</Space>
                  {dayCheckbox}
                </Radio>
              </Radio.Group>
            </TabPane>
            <TabPane tab="月" key="5">
              <Radio.Group
                onChange={(e) => this.handleRadioChange(e, "month")}
                value={radioValue["month"]}
              >
                <Radio style={radioStyle} value={1}>
                  每月执行
                </Radio>
                <br />
                {/*              <Radio style={radioStyle} value={2}>
                不指定
              </Radio>*/}
                <br />
                <Radio style={radioStyle} value="loop">
                  <InputNumber
                    size="small"
                    type={"hidden"}
                    style={{ display: "none" }}
                    min={1}
                    max={12}
                    defaultValue={1}
                    onChange={(e) => this.handleLoopChange(e, "month", "start")}
                  />

                  <Space>
                    每
                    <InputNumber
                      size="small"
                      min={1}
                      max={12}
                      defaultValue={this.state.loopValue.month.end}
                      onChange={(e) => this.handleLoopChange(e, "month", "end")}
                    />
                    月执行一次
                  </Space>
                </Radio>
                <br />
                <Radio
                  style={radioStyle}
                  value="point"
                  checked={!!this.state.cronParams.month}
                >
                  <Space>指定</Space>
                  {monthCheckbox}
                </Radio>
              </Radio.Group>
            </TabPane>
            <TabPane tab="周" key="6">
              <Radio.Group
                onChange={(e) => this.handleRadioChange(e, "week")}
                value={radioValue["week"]}
              >
                <Radio style={radioStyle} value={1}>
                  每周执行
                </Radio>
                <br />
                {/*              <Radio style={radioStyle} value={2}>
                不指定
              </Radio>*/}
                <br />
                <Radio
                  style={radioStyle}
                  value="point"
                  checked={!!this.state.cronParams.week}
                >
                  <Space>指定</Space>
                  {weekCheckbox}
                </Radio>
              </Radio.Group>
            </TabPane>
          </Tabs>
          <div style={{ margin: "20px 0" }}>
            <Button type="primary" htmlType="submit">
              生成
            </Button>
          </div>
        </Form>
        <div style={{}}>
          <h6 style={{ fontSize: "14px" }}>
            最近5次执行时间
            <span style={{ fontWeight: 200, color: "#666" }}>
              (点击生成按钮后查看)
            </span>
            ：
          </h6>
          {this.state.cronPreview.map((item) => {
            return (
              <p style={{ margin: 0, lineHeight: "20px", color: "#666" }}>
                {item}
              </p>
            )
          })}
        </div>
      </>
    )
  }
}
export default CronGenerator
