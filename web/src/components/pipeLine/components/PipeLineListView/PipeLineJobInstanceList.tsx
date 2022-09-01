import React, { useContext, useEffect, useState } from "react"
import { useSetState, useUpdateEffect } from "ahooks"
import pipeLineApi from "@/services/otherApi/pipeLine"
import {
  Button,
  Dropdown,
  Form,
  Input,
  Menu,
  message,
  Select,
  Table,
} from "antd"
import { PlusOutlined, SearchOutlined } from "@ant-design/icons"
import "./PipeLineJobInstanceList.less"
import { ColumnsType } from "antd/es/table"
import { SorterResult } from "antd/es/table/interface"
import dayjs from "dayjs"
import { PipeLineContext } from "../../PipeLineHome"
import { StatusEnum, statusType } from "./index"
import classNames from "classnames"
import PipeSearchIcon from "./pipeSearchIcon"
import PubSub from "pubsub-js"
import intl from "react-intl-universal"
import { useDidCache, useDidRecover } from 'react-router-cache-route'
import DAG from "@/components/DAG/DAG"

const { Option } = Select

enum RunTypeEnum {
  Manual = "手动",
  Schedule = "自动",
}

interface TableDataType {
  id: number
  jobId: number
  jobName: string
  jobType?: number
  createtime?: string
  updatetime?: string
  status?: statusType
  tasks?: any[]
  cronConfig?: {
    cronEndDate?: string
    cronExpression?: string
    cronStartDate?: string
  }
  config?: { [prop in string]: any }
  useres?: { [prop in string]: any }
  delFlag?: number

  // 执行计划时间 需要进行转换
  timeCost?: number
  startTime?: string
  endTime?: string
  runType?: "Manual" | "Schedule"
}

function formatSeconds(value) {
  let result = parseInt(value)
  let h =
    Math.floor(result / 3600) < 10
      ? "0" + Math.floor(result / 3600)
      : Math.floor(result / 3600)
  let m =
    Math.floor((result / 60) % 60) < 10
      ? "0" + Math.floor((result / 60) % 60)
      : Math.floor((result / 60) % 60)
  let s =
    Math.floor(result % 60) < 10
      ? "0" + Math.floor(result % 60)
      : Math.floor(result % 60)

  let res = ""
  res += `${h}:`
  res += `${m}:`
  res += `${s}`
  return res
}

export function tagRender(props) {
  const { label, value } = props
  return (
    <span
      className={classNames("status-span", value)}
      style={{ padding: "0 5px" }}
    >
      {label}
    </span>
  )
}

export const statusOptions = [
  /*  { value: "Init", label: "未计划" },
  { value: "Schedule", label: "计划内" },*/
  { value: "Pending", label: "等待中" },
  { value: "Success", label: "成功" },
  { value: "Running", label: "运行中" },
  { value: "Fail", label: "失败" },
  { value: "Kill", label: "被中断" },
]

const sortOptions = [
  { value: "id", label: "工作流实例ID" },
  { value: "start_time", label: "开始时间" },
  { value: "end_time", label: "结束时间" },
]

export const switchIcon = (value: string) => {
  switch (value) {
    case "Init":
      return <PipeSearchIcon.InitIcon />
    case "Schedule":
      return <PipeSearchIcon.ScheduleIcon />
    case "Pending":
      return <PipeSearchIcon.PendingIcon />
    case "Success":
      return <PipeSearchIcon.SuccessIcon />
    case "Running":
      return <PipeSearchIcon.RunningIcon />
    case "Fail":
      return <PipeSearchIcon.FailIcon />
    default:
      return <PipeSearchIcon.ScheduleIcon />
  }
}

export interface SearchInfo {
  sortField: "end_time" | "start_time" | "id" | "jobId" | ""
  sort: "asc" | "desc" | ""
  status: string
}

function PipeLineJobInstanceList(props) {
  const { taskOrJobKey } = props
  const { addTabPane, pipeHomeTabActiveKey, tabKeyCount } =
    useContext(PipeLineContext)
  const [jobInstanceList, setJobInstanceList] = useState<TableDataType[]>([])
  const [paginationInfo, setPaginationInfo] = useSetState({
    current: 1,
    size: 10,
    total: 0,
  })
  const [searchText, setSearchText] = useState("")

  const [searchInfo, setSearchInfo] = useSetState<SearchInfo>({
    sortField: "",
    sort: "",
    status: "",
  })
  const { current, size, total } = paginationInfo
  const columns: ColumnsType<TableDataType> = [
    {
      title: `${intl.get("WORKFLOW_INSTANCE")}ID`,
      dataIndex: "id",
      align: "center",
      className: "black-color",
      sorter: true,
      render(value, record) {
        return (
          <span
            onClick={openJobInstance(record, "view")}
            style={{ cursor: "pointer" }}
          >
            {value}
          </span>
        )
      },
    },
    {
      title: intl.get("WORKFLOW"),
      dataIndex: "jobName",
      align: "center",
      className: "black-color",
      render(value, record) {
        return (
          <span
            onClick={openJobInstance(record, "edit")}
            style={{ cursor: "pointer" }}
          >
            {value}
          </span>
        )
      },
    },
    {
      title: intl.get("START_METHOD"),
      dataIndex: "runType",
      align: "center",
      render(value) {
        return <span>{RunTypeEnum[value] || ""}</span>
      },
    },

    {
      title: intl.get("OPERATING_STATUS"),
      dataIndex: "status",
      align: "center",
      render(value) {
        return (
          <span className={classNames("status", value)}>
            {StatusEnum[value]}
          </span>
        )
      },
    },
    {
      title: intl.get("RUNTIME"),
      dataIndex: "timeCost",
      align: "center",
      className: "black-opacity-color",
      render(value, record) {
        return <span>{value ? formatSeconds(value) : ""}</span>
      },
    },
    {
      title: intl.get("STARTING_TIME"),
      dataIndex: "startTime",
      align: "center",
      className: "black-opacity-color",
      sorter: true,
      render(value, record) {
        return (
          <span>
            {value ? dayjs(new Date(value)).format("YYYY-MM-DD HH:mm:ss") : ""}
          </span>
        )
      },
    },
    {
      title: intl.get("END_TIME"),
      dataIndex: "endTime",
      align: "center",
      className: "black-opacity-color",
      sorter: true,
      render(value, record) {
        return (
          <span>
            {value ? dayjs(new Date(value)).format("YYYY-MM-DD HH:mm:ss") : ""}
          </span>
        )
      },
    },
    {
      title: intl.get("OPERATE"),
      align: "center",
      width: 160,
      render(text, record, index) {
        const status = record.status
        const statusArr = ["Success", "Fail", "Kill"]

        const isShowMore = !statusArr.includes(status)

        const overlayContent = (
          <Menu style={{ textAlign: "center" }}>
            <Menu.Item onClick={killJobInstanceById(record.id, record.status)}>
              <Button style={{ padding: "4px 0" }} type={"link"}>
                {intl.get("SUB_MENU_STOP")}
              </Button>
            </Menu.Item>
            {/*            <Menu.Item>
              <Button style={{ padding: "4px 0" }} type={"link"}>
                继续运行
              </Button>
            </Menu.Item>
            <Menu.Item>
              <Button style={{ padding: "4px 0" }} type={"link"}>
                重新运行
              </Button>
            </Menu.Item>

            <Menu.Item>
              <Button style={{ padding: "4px 0" }} type={"link"}>
                取消任务
              </Button>
            </Menu.Item>*/}
          </Menu>
        )
        return (
          <div style={{ textAlign: "center" }}>
            <Button
              onClick={openJobInstance(record, "view")}
              className={"run-work-btn"}
              style={{
                padding: "4px 5px",
                color: "#1890FF",
              }}
              type={"link"}
            >
              {intl.get("SEE_DETAILS")}
            </Button>
            {isShowMore ? (
              <Dropdown arrow overlay={overlayContent}>
                <Button
                  className={"more-btn"}
                  style={{ padding: "4px 2px", color: "#1890FF" }}
                  type={"link"}
                >
                  {intl.get("MORE")}
                </Button>
              </Dropdown>
            ) : null}
          </div>
        )
      },
    },
  ]

  useDidRecover(() => {
    if (pipeHomeTabActiveKey === "pipeLineListView") {
      getJobInstanceList()
    }
  })

  // done
  useUpdateEffect(() => {
    if (pipeHomeTabActiveKey === "pipeLineListView") {
      getJobInstanceList()
    }
  }, [pipeHomeTabActiveKey])

  useEffect(() => {
    getJobInstanceList()
  }, [current, size, searchInfo.sort, searchInfo.sortField, searchInfo.status])

  useEffect(() => {
    const refreshJobInstanceListSubscribe = PubSub.subscribe(
      "refreshJobInstanceList",
      (msg, data) => {
        getJobInstanceList()
      }
    )

    return () => {
      PubSub.unsubscribe(refreshJobInstanceListSubscribe)
    }
  }, [])

  const getJobInstanceList = () => {
    const { current, size } = paginationInfo
    const { sortField, sort, status } = searchInfo
    const iDName = searchText
    setJobInstanceList([])
    pipeLineApi
      .getJobInstanceList({ current, size, sort, sortField, status, iDName })
      .then((res) => {
        const list: TableDataType[] = res.data.records.map((item) => {
          return {
            ...item,
          }
        })
        setJobInstanceList(list)
        setPaginationInfo({
          total: res.data.total,
        })
      })
  }

  const openJobInstance = (record: TableDataType, mode: "view" | "edit") => {
    return () => {
      const { jobId, jobName, id } = record;
      if (mode === "view") {
        addTabPane({
          key: "viewJonInstance" + id,
          title: `${intl.get("WORKFLOW_INSTANCE")}${id}`,
          content: (
            <DAG
              experimentId={jobId + ""}
              experimentInstanceId={`${id}`}
              mode={mode}
            />
          ),
        })
      } else {
        addTabPane({
          key: "editJonInstance" + jobId,
          title: `${intl.get("EDIT")}${jobName}`,
          content: <DAG experimentId={jobId + ""} mode={mode} />,
        })
      }
    }
  }

  const killJobInstanceById = (id: number, status: statusType) => {
    return () => {
      if (status === "Running") {
        pipeLineApi.killJobInstanceById(id).then((res) => {
          message.success(`${intl.get("SUB_MENU_STOP")}${intl.get("SUCCESS")}`)
          getJobInstanceList()
        })
      } else {
        message.warning(
          `${intl.get("NOT_CURRENTLY_RUNNING")} ${intl.get(
            "THIS_OPERATION_IS_NOT_POSSIBLE"
          )}`
        )
      }
    }
  }

  const tableChange = (
    pagination,
    filters,
    sorter: SorterResult<TableDataType>
  ) => {
    let sortField
    let sort
    switch (sorter.field) {
      case "id":
        sortField = "id"
        break
      case "startTime":
        sortField = "start_time"
        break
      case "endTime":
        sortField = "end_time"
        break
    }
    switch (sorter.order) {
      case undefined:
        sort = ""
        break
      case "ascend":
        sort = "asc"
        break
      case "descend":
        sort = "desc"
        break
    }

    setSearchInfo({
      sortField,
      sort,
    })
  }

  return (
    <div className={"pipe-line-job-instance-list-view"}>
      <div className={"header-search"}>
        <Form layout={"inline"}>
          <Input.Group style={{ width: 320 }} compact>
            <Button
              onClick={() => {
                getJobInstanceList()
              }}
              icon={
                <SearchOutlined style={{ color: "white", fontSize: "22px" }} />
              }
              style={{
                width: "56px",
                height: "40px",
                textAlign: "center",
                backgroundColor: "#1890FF",
              }}
              type="primary"
            />
            <Input
              allowClear
              value={searchText}
              onChange={(event) => {
                setSearchText(event.target.value)
              }}
              onPressEnter={() => {
                getJobInstanceList()
              }}
              placeholder={intl.get("SEARCH_FOR_WORKFLOW_INSTANCE_ID_OR_WORKFLOW_NAME")}
              style={{ width: "264px", height: "40px" }}
            />
          </Input.Group>

          <Select
            placeholder={intl.get("STATUS")}
            style={{ minWidth: 120, height: 40, marginLeft: 16 }}
            size={"large"}
            allowClear
            mode="multiple"
            tagRender={tagRender}
            onChange={(values: any[]) => {
              setSearchInfo({
                status: values.join(","),
              })
            }}
          >
            {statusOptions.map((item) => {
              return (
                <Option key={item.value} value={item.value}>
                  <span style={{ paddingLeft: 5 }}>
                    {switchIcon(item.value)}
                    &nbsp; &nbsp;
                    {item.label}
                  </span>
                </Option>
              )
            })}
          </Select>

          <Select
            placeholder={intl.get("SORT_TYPE")}
            style={{ minWidth: 145, height: 40, marginLeft: 16 }}
            size={"large"}
            allowClear
            options={sortOptions}
            onChange={(value: any) => {
              setSearchInfo({
                sortField: value,
              })
            }}
            value={searchInfo.sortField || undefined}
          />
        </Form>

        <div>
          <Button
            size={"large"}
            icon={<PlusOutlined style={{ color: "white" }} />}
            type={"primary"}
            onClick={() => {
              addTabPane({
                key: "createJonInstance" + tabKeyCount,
                title: intl.get("CREATE_A_WORKFLOW") + tabKeyCount,
                content: <DAG experimentId={"0"} />,
              })
            }}
          >
            {intl.get("CREATE_A_WORKFLOW")}
          </Button>
        </div>
      </div>

      <div className={"pipe-line-job-instance-table"}>
        <Table
          scroll={{ y: 360 }}
          onChange={tableChange}
          pagination={{
            showQuickJumper: true,
            current,
            total,
            pageSize: size,
            onChange: (page, pageSize) => {
              setPaginationInfo({ current: page, size: pageSize })
            },
            pageSizeOptions: ["10", "20", "30"],
          }}
          rowKey={"id"}
          dataSource={jobInstanceList}
          columns={columns}
        />
      </div>
    </div>
  )
}

export default PipeLineJobInstanceList
