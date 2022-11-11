import React from 'react'
import PropTypes from 'prop-types'
import intl from "react-intl-universal"
import AddFileInput from "../../AddFileInput"
import {Modal} from "antd"

AddFileModal.propTypes = {
  addFile:PropTypes.bool.isRequired,
  submitAddFile:PropTypes.func.isRequired,
  confirmLoading:PropTypes.bool.isRequired,
  resetVisible:PropTypes.func.isRequired,
  setFileValue:PropTypes.func.isRequired,
  fileInputDisabled:PropTypes.bool.isRequired,
  fileValue:PropTypes.string.isRequired,
  displayWarning:PropTypes.string.isRequired,
  fileNameValidator:PropTypes.string.isRequired,
}

function AddFileModal(props) {

  const {
    addFile,
    submitAddFile,
    confirmLoading,
    resetVisible,
    setFileValue,
    fileInputDisabled,
    fileValue,
    displayWarning,
    fileNameValidator,
  } = props


  return (
    <Modal
      title={intl.get("ADD_FILE")}
      visible={addFile}
      onOk={submitAddFile}
      confirmLoading={confirmLoading}
      onCancel={resetVisible}
      destroyOnClose={true}
    >
      <AddFileInput
        placeholder={intl.get("ADD_FILE_PLACEHOLDER")}
        setFileValue={setFileValue}
        disabled={fileInputDisabled}
        onPressEnter={submitAddFile}
        defaultValue={fileValue}
      />
      <div
        style={{
          color: "red",
          fontWeight: 500,
          display:displayWarning ,
        }}
      >
        {fileNameValidator}
      </div>
    </Modal>
  )
}

export default AddFileModal
