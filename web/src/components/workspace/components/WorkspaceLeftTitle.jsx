import React,{Fragment, useState} from 'react'
import {Button, Col, Dropdown, Menu, Row, Tooltip} from "antd"
import intl from "react-intl-universal"
import {CaretDownOutlined, PlusOutlined} from "@ant-design/icons"
import Icons from "../../Icons/Icons"
import PropTypes from "prop-types"
import UpDatePlayIDP from './Modals/UpDatePlayIDP'

const {
  BHAddFolderIcon,
  BHAddFileIcon,
  BHRefreshIcon,
  BHUploadFolderIcon,
  BHUploadFileIcon,
} = Icons

function WorkspaceLeftTitle(props) {

  const {
    addFolder,
    addFile,
    handleIsFileTree,
    handleFileChange,
    loadTree
  } = props

  const [visible, setVisible] = useState(false)

  const changeVisible = () => {
    setVisible(!visible)
  }

  const downMenu = (
    <Menu>
      <Menu.Item onClick={addFolder} key="1">
        <Button
          className="workBtn"
          icon={<BHAddFolderIcon />}
          size="small"
          type="text"
          style={{ width: 28 }}
        ></Button>
        {intl.get("ADD_FOLDER")}
      </Menu.Item>
      <Menu.Item onClick={addFile} key="2">
        <Button
          className="workBtn"
          icon={<BHAddFileIcon />}
          size="small"
          type="text"
          style={{ width: 28 }}
        ></Button>
        {intl.get("ADD_FILE")}
      </Menu.Item>
      <Menu.Divider />
      <Menu.Item
        onClick={() => {
          if (!handleIsFileTree()) {
            return
          }
          document.getElementById("chooseFolder").click()
        }}
        key="3"
      >
        <Button
          className="workBtn"
          icon={<BHUploadFolderIcon />}
          size="small"
          type="text"
          style={{ width: 28 }}
        ></Button>
        {intl.get("UPLOAD_FOLDER")}
      </Menu.Item>
      <Menu.Item
        key="4"
        onClick={() => {
          if (!handleIsFileTree()) {
            return
          }
          document.getElementById("chooseFiles").click()
        }}
      >

        <Button
          className="workBtn"
          icon={<BHUploadFileIcon />}
          size="small"
          type="text"
          style={{ width: 28 }}
        ></Button>
        {intl.get("UPLOAD_FILE")}
      </Menu.Item>
      {/* <Menu.Divider />
      <Menu.Item
        key="5"
        onClick={changeVisible}
      >

        <Button
          className="workBtn"
          icon={<BHRefreshIcon />}
          size="small"
          type="text"
          style={{ width: 28 }}
        ></Button>
        更新“玩转IDP”
      </Menu.Item> */}
    </Menu>
  )


  return (
    <Fragment>
      <Row
        style={{
          height: 32,
          lineHeight: "32px",
          justifyContent: "space-between",
        }}
      >
        <Col style={{ paddingLeft: 12, height: 32 }}>
            <span className="resource-manager">
              {intl.get("RESOURCE_MANAGER")}
            </span>
        </Col>
        <Col
          style={{
            whiteSpace: "nowrap",
            height: 35,
            paddingRight: 10,
            paddingBottom: 20,
          }}
          className="resource-buttons"
        >
          <Tooltip placement="bottom" title={intl.get("RELOAD")}>
            <Button
              icon={<BHRefreshIcon />}
              size="small"
              type="text"
              onClick={() => {
                loadTree({ forceLoad: true, loadDataSource: true })
              }}
              style={{ width: 28 }}
            ></Button>
          </Tooltip>
          <Dropdown
            overlay={downMenu}
            placement="bottomLeft"
            destroyPopupOnHid={true}
            // trigger={["hover"]}
          >
            <Button
              size="small"
              type="text"
              style={{
                verticalAlign: "middle",
                marginLeft: 5,
                marginRight: 5,
                marginTop: -2,
              }}
            >
              <div
                className="entry-down-menu"
                onClick={(e) => e.preventDefault()}
              >
                <div className="EDM-left">
                  <PlusOutlined />
                </div>
                <div className="EDM-right">
                  <CaretDownOutlined />
                </div>
              </div>
            </Button>
          </Dropdown>
        </Col>
      </Row>

      <input
        type="file"
        id="chooseFiles"
        multiple={true}
        onChange={() => handleFileChange("file", "chooseFiles")}
        style={{ display: "none" }}
      />

      <input
        type="file"
        id="chooseFolder"
        multiple={true}
        onChange={() => handleFileChange("folder", "chooseFolder")}
        webkitdirectory="true"
        style={{ display: "none" }}
      />
      <UpDatePlayIDP
        changeVisible={changeVisible}
        visible={visible}
      />
    </Fragment>
  )
}

WorkspaceLeftTitle.propTypes = {
  addFolder:PropTypes.func.isRequired,
  addFile:PropTypes.func.isRequired,
  handleIsFileTree:PropTypes.func.isRequired,
  handleFileChange:PropTypes.func.isRequired,
  loadTree:PropTypes.func.isRequired,
}

export default WorkspaceLeftTitle
