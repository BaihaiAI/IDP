import React from 'react'
import PropTypes from 'prop-types'
import {Input, Modal} from "antd"
import intl from "react-intl-universal"

RenameModal.propTypes = {
  rename:PropTypes.bool.isRequired,
  submitRename:PropTypes.func.isRequired,
  confirmLoading:PropTypes.bool.isRequired,
  resetVisible:PropTypes.func.isRequired,
  renameValue:PropTypes.string.isRequired,
  setInputValue:PropTypes.func.isRequired,
  renameInputDisabled:PropTypes.bool.isRequired,
  displayWarning:PropTypes.string.isRequired,
  fileNameValidator:PropTypes.string.isRequired,
}

function RenameModal(props) {

  const {
    rename,
    submitRename,
    confirmLoading,
    resetVisible,
    renameValue,
    setInputValue,
    renameInputDisabled,
    displayWarning,
    fileNameValidator,
  } = props

  return (
    <Modal
      title={intl.get("RENAME")}
      visible={rename}
      onOk={submitRename}
      confirmLoading={confirmLoading}
      onCancel={resetVisible}
    >
      <Input
        placeholder={intl.get("ADD_FILE_PLACEHOLDER")}
        value={renameValue}
        onChange={(e) => {
          setInputValue(e, "rename")
        }}
        onPressEnter={submitRename}
        disabled={renameInputDisabled}
        onFocus={(event) => event.target.select()}
      />
      <div
        style={{
          color: "red",
          fontWeight: 500,
          display: displayWarning,
        }}
      >
        {fileNameValidator}
      </div>
    </Modal>
  )
}

export default RenameModal
