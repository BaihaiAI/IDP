import React, { useContext, useEffect, useState } from "react"
import {
  Button,
  Input,
  Select,
  Form,
  Table,
  Dropdown,
  Menu,
  Modal,
  message,
} from "antd"
import { PlusOutlined, SearchOutlined, WarningFilled } from "@ant-design/icons"
import { ColumnsType } from "antd/es/table"
import pipeLineApi from "@/services/otherApi/pipeLine"
import { useSetState, useUpdateEffect } from "ahooks"
import "./PipeLineJobList.less"
import { PipeLineContext } from "../../PipeLineHome"
import { SearchInfo, switchIcon, tagRender } from "./PipeLineJobInstanceList"
import { SorterResult } from "antd/es/table/interface"
import classNames from "classnames"
import { StatusEnum, statusType } from "./index"
import PubSub from "pubsub-js"
import intl from "react-intl-universal"
import { useDidCache, useDidRecover } from 'react-router-cache-route'
import DAG from "@/components/DAG/DAG"

const { Option } = Select

const statusOptions = [
  { value: "Init", label: "未计划" },
  { value: "Schedule", label: "计划内" },
  /*  { value: "Pending", label: "等待中" },
  { value: "Success", label: "成功" },
  { value: "Running", label: "运行中" },
  { value: "Fail", label: "失败" },*/
]

const sortOptions = [
  { value: "job_id", label: "工作流ID" },
  { value: "cronStartDate", label: "计划开始时间" },
  { value: "cronEndDate", label: "计划结束时间" },
]

interface TableDataType {
  jobId: number
  jobName: string
  jobType: number
  createtime: string
  updatetime: string
  status: statusType
  tasks: any[]
  config: { [prop in string]: any }
  useres: { [prop in string]: any }
  delFlag: number
  cronConfig: {
    cronEndDate: string
    cronExpression: string
    cronStartDate: string
  } | null
  cronStartDate: string
  cronEndDate: string
  cronExpressionDisp: string | null
}

function PipeLineJobList(props) {
  const { taskOrJobKey } = props

  const { addTabPane, tabKeyCount, pipeHomeTabActiveKey } =
    useContext(PipeLineContext)
  const [jobList, setJobList] = useState<TableDataType[]>([])
  const [paginationInfo, setPaginationInfo] = useSetState({
    current: 1,
    size: 10,
    total: 0,
  })

  const [searchInfo, setSearchInfo] = useSetState<SearchInfo>({
    sortField: "",
    sort: "",
    status: "",
  })
  const [searchText, setSearchText] = useState("")

  const { current, size, total } = paginationInfo

  useDidRecover(() => {
    if (pipeHomeTabActiveKey === "pipeLineListView") {
      getJobList()
    }
  })

  useUpdateEffect(() => {
    if (pipeHomeTabActiveKey === "pipeLineListView") {
      getJobList()
    }
  }, [pipeHomeTabActiveKey])

  useEffect(() => {
    getJobList()
  }, [current, size, searchInfo.sort, searchInfo.sortField, searchInfo.status])

  const openEditJobInstance = (record: TableDataType) => {
    return () => {
      const { jobId, jobName } = record
      addTabPane({
        key: "editJonInstance" + jobId,
        title: `${intl.get("EDIT")}${jobName}`,
        content: <DAG experimentId={jobId + ""} />,
      })
    }
  }

  const columns: ColumnsType<TableDataType> = [
    {
      title: `${intl.get("WORKFLOW")}ID`,
      dataIndex: "jobId",
      align: "center",
      className: "black-color",
      sorter: true,
      render(value, record) {
        return (
          <span
            onClick={openEditJobInstance(record)}
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
            onClick={openEditJobInstance(record)}
            style={{ cursor: "pointer" }}
          >
            {value}
          </span>
        )
      },
    },

    {
      title: intl.get("STATUS"),
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
      title: intl.get("IMPLEMENTATION_PLAN"),
      dataIndex: "cronExpressionDisp",
      render(value) {
        return <span>{value}</span>
      },
    },
    {
      title: intl.get("PLAN_START_TIME"),
      dataIndex: "cronStartDate",
      align: "center",
      className: "black-opacity-color",
      sorter: true,
      render(value, record) {
        return <span>{value}</span>
      },
    },
    {
      title: intl.get("PLANNED_END_TIME"),
      dataIndex: "cronEndDate",
      align: "center",
      className: "black-opacity-color",
      sorter: true,
      render(value, record) {
        return <span>{value}</span>
      },
    },
    {
      title: intl.get('OPERATE'),
      align: "center",
      width: 160,
      render(text, record, index) {
        const overlayContent = (
          <Menu style={{ textAlign: "center" }}>
            <Menu.Item onClick={openEditJobInstance(record)}>
              <Button
                style={{ padding: "4px 0", color: "#3793EF" }}
                type={"link"}
              >
                {intl.get("EDIT_WORKFLOW")}
              </Button>
            </Menu.Item>
            <Menu.Item
              onClick={() => {
                pipeLineApi.cloneJob(record.jobId).then((res) => {
                  message.success(intl.get("CLONE_WORKFLOW_SUCCEEDED"))
                  getJobList()
                })
              }}
            >
              <Button
                style={{ padding: "4px 0", color: "#3793EF" }}
                type={"link"}
              >
                {intl.get("COPY_AS_NEW_WORKFLOW")}
              </Button>
            </Menu.Item>
            <Menu.Item
              onClick={() => {
                Modal.confirm({
                  maskClosable: true,
                  closable: true,
                  centered: true,
                  title: intl.get("CAN_T_BE_RECOVERED_AFTER_DELETION"),
                  icon: <WarningFilled />,
                  okText: intl.get("CONFIRM_DELETION"),
                  okType: "primary",
                  okButtonProps: {},
                  cancelText: intl.get('CANCEL'),
                  onOk() {
                    pipeLineApi.deleteJob(record.jobId).then((res) => {
                      message.success(intl.get("DELETE_SUCCEEDED"))
                      if (jobList.length === 1 && current !== 1) {
                        setPaginationInfo({
                          current: current - 1,
                        })
                      } else {
                        getJobList()
                      }
                    })
                  },
                })
              }}
            >
              <Button
                style={{ padding: "4px 0", color: "#FF4D4F" }}
                type={"link"}
              >
                {intl.get("DELETE")}
              </Button>
            </Menu.Item>
          </Menu>
        )
        return (
          <div style={{ textAlign: "center" }}>
            <Button
              className={"run-work-btn"}
              style={{
                padding: "4px 5px",
                color: "#1890FF",
              }}
              type={"link"}
              onClick={() => {
                pipeLineApi.runJob(record.jobId).then((res) => {
                  message.success(intl.get("RUN_SUCCESSFULLY"))
                  PubSub.publish("refreshJobInstanceList")
                })
              }}
            >
              {intl.get("EXECUTE_WORKFLOW")}
            </Button>
            <Dropdown arrow overlay={overlayContent}>
              <Button
                style={{ padding: "4px 2px", color: "#1890FF" }}
                type={"link"}
              >
                {intl.get('MORE')}
              </Button>
            </Dropdown>
          </div>
        )
      },
    },
  ]

  const getJobList = () => {
    const { current, size } = paginationInfo
    const { sortField, sort, status } = searchInfo
    const jobIdName = searchText
    setJobList([])
    pipeLineApi
      .getJobList({ current, size, sortField, sort, status, jobIdName })
      .then((res) => {
        const list: TableDataType[] = res.data.records.map((item) => {
          return {
            ...item,
          }
        })
        setJobList(list)
        setPaginationInfo({
          total: res.data.total,
        })
      })
  }

  const tableChange = (
    pagination,
    filters,
    sorter: SorterResult<TableDataType>
  ) => {
    let sortField
    let sort

    switch (sorter.field) {
      case "jobId":
        sortField = "job_id"
        break
      case "cronStartDate":
        sortField = "cronStartDate"
        break
      case "cronEndDate":
        sortField = "cronEndDate"
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
    <div className={"pipe-line-job-list-view"}>
      <div className={"header-search"}>
        <Form layout={"inline"}>
          <Input.Group style={{ width: 320 }} compact>
            <Button
              onClick={() => {
                getJobList()
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
                getJobList()
              }}
              placeholder={intl.get("SEARCH_FOR_WORKFLOW_ID_OR_WORKFLOW_NAME")}
              style={{ width: "264px", height: "40px" }}
            />
          </Input.Group>

          <Select
            mode="multiple"
            placeholder={intl.get('STATUS')}
            style={{ minWidth: 120, height: 40, marginLeft: 16 }}
            size={"large"}
            allowClear
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
                    &nbsp;&nbsp;
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
                content: <DAG experimentId={"0_" + (new Date().getTime())} />,
              })
            }}
          >
            {intl.get("CREATE_A_WORKFLOW")}
          </Button>
        </div>
      </div>

      <div className={"pipe-line-job-table"}>
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
          rowKey={"jobId"}
          dataSource={jobList}
          columns={columns}
        />
      </div>
    </div>
  )
}

export default PipeLineJobList
