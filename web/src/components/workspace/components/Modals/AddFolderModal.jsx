import React from 'react'
import intl from "react-intl-universal"
import AddFolderInput from "../../AddFolderInput"
import {Modal} from "antd"
import PropTypes from "prop-types"

function AddFolderModal(props) {

  const {
    addFolder,
    submitAddFolder,
    confirmLoading,
    resetVisible,
    setInputValue,
    displayWarning,
    fileNameValidator,
    folderValue,
    folderInputDisabled,
  } = props


  return (
    <Modal
      title={intl.get("ADD_FOLDER")}
      visible={addFolder}
      onOk={submitAddFolder}
      confirmLoading={confirmLoading}
      onCancel={resetVisible}
      destroyOnClose={true}
    >
      <AddFolderInput
        placeholder={intl.get("ADD_FOLDER_PLACEHOLDER")}
        defaultValue={folderValue}
        onChange={(e) => setInputValue(e, "folder")}
        onPressEnter={submitAddFolder}
        disabled={folderInputDisabled}
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

AddFolderModal.propTypes = {
  addFolder:PropTypes.bool.isRequired,
  submitAddFolder:PropTypes.func.isRequired,
  confirmLoading:PropTypes.bool.isRequired,
  resetVisible:PropTypes.func.isRequired,
  setInputValue:PropTypes.func.isRequired,
  displayWarning:PropTypes.string.isRequired,
  fileNameValidator:PropTypes.string.isRequired,
  folderValue:PropTypes.string.isRequired,
  folderInputDisabled:PropTypes.bool.isRequired,
}

export default AddFolderModal
