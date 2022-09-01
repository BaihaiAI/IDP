import React from "react"
import VersionDiffPatch from './VersionDiffPatch'
import "./VersionPanel.less"
import {useNotebookItem} from "../../../utils/hook/useActiveCellProps"
export const Context = React.createContext();

function RenderVersionPanel(props) {
  const {
    activeTabKey,
    showSaveVersion,
    setShowSaveVersion,
    showVersionDrawer,
    setShowVersionDrawer,
  } = props
  const notebookItem = useNotebookItem(activeTabKey)
  if(activeTabKey){
    return (
      <Context.Provider value={[ showVersionDrawer]}>
        <VersionDiffPatch
          path={activeTabKey}
          setShowVersionDrawer={setShowVersionDrawer}
          isExecuting={notebookItem.isExecuting}
          showSaveVersion={showSaveVersion}
          setShowSaveVersion={setShowSaveVersion}
        />
      </Context.Provider>
    )
  }
  return null
}

export default RenderVersionPanel;