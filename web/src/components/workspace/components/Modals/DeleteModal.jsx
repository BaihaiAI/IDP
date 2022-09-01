import React, {useCallback, useEffect, useMemo, useState} from 'react'
import PropTypes from 'prop-types'
import {Modal} from "antd"
import intl from "react-intl-universal"

import { useHotkeys } from "react-hotkeys-hook"

DeleteModal.propTypes = {
  deleteVisible:PropTypes.bool.isRequired,
  submitDelete:PropTypes.func.isRequired,
  confirmLoading:PropTypes.bool.isRequired,
  resetVisible:PropTypes.func.isRequired,
  selectedName:PropTypes.string.isRequired,
}

function DeleteModal(props) {

  const {
    deleteVisible,
    submitDelete,
    confirmLoading,
    resetVisible,
    selectedName,
  } = props

  useHotkeys('enter', () => {
    submitDelete()
  })

  return (
    <Modal
      title={intl.get("DELETE")}
      visible={deleteVisible}
      onOk={submitDelete}
      confirmLoading={confirmLoading}
      onCancel={resetVisible}
      autoFocusButton="ok"
    >
      {/* <div style={{color: 'red', fontWeight: 500}}>{intl.get('DELETE_CONFIRM_DESCRIPTION')}"{this.state.selectedName}"{this.state.deleteWarnMsg}?</div> */}
      <div style={{ color: "red", fontWeight: 500 }}>
        {intl.get("DELETE_CONFIRM_DESCRIPTION")}"{selectedName}"?
      </div>
    </Modal>
  )
}

export default DeleteModal
