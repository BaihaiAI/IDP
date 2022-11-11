import React, {Fragment} from 'react'
import {Tooltip} from "antd"
import intl from "react-intl-universal"
import Icons from "../../Icons/Icons"
import PropTypes from "prop-types"

function CollapseWorkspaceLeft(props) {

  const {
    hanClickDown,
    siderWidth,
    handleClickShrink,
  } = props

  return (
    <Fragment>
      <div
        className="sidebar-handle"
        onMouseDown={hanClickDown}
      ></div>

      <Tooltip
        placement="bottom"
        title={
          siderWidth !== 1
            ? intl.get("COLLAPSE_SIDEBAR")
            : intl.get("EXPAND_SIDEBAR")
        }
      >
        <div
          className={
            siderWidth !== 1
              ? "sidebar-btn itemcenter"
              : "sidebar-btn is360 itemcenter"
          }
          onClick={handleClickShrink}
        >
          <div className="sidebar-btn-icon">
            <Icons.FileTreeShow />
          </div>
        </div>
      </Tooltip>

    </Fragment>
  )
}

CollapseWorkspaceLeft.propTypes = {
  hanClickDown:PropTypes.func.isRequired,
  handleClickShrink:PropTypes.func.isRequired,
  siderWidth:PropTypes.number.isRequired,
}

export default CollapseWorkspaceLeft
