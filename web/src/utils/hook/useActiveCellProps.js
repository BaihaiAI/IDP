import {useMemo} from "react"
import {getNoteBookIndexFromPath} from "../../store/features/notebookSlice"
import {store} from "../../store"
import {findFileListIndex} from "../../store/features/filesTabSlice"

export function useActiveCellProps(activePath) {
  const notebookList = store.getState().notebook.notebookList
  return useMemo(() => {
    const index = getNoteBookIndexFromPath(activePath, notebookList)
    if (index === -1) {
      return {}
    } else {
      return notebookList[index].cellProps
    }
  }, [notebookList, activePath])
}
export function useActiveCells(activePath) {
  const notebookList = store.getState().notebook.notebookList
  return useMemo(() => {
    const index = getNoteBookIndexFromPath(activePath, notebookList)
    if (index === -1) {
      return []
    } else {
      return notebookList[index].cells
    }
  }, [notebookList, activePath])
}


export function useActiveVariableList(activePath) {
  const notebookList = store.getState().notebook.notebookList
  return useMemo(() => {
    const index = getNoteBookIndexFromPath(activePath, notebookList)
    if (index === -1) {
      return []
    } else {
      return notebookList[index].variableList
    }
  }, [notebookList, activePath])
}
// export function useActiveNotebookJson(activePath) {
//   const notebookList = store.getState().notebook.notebookList
//   return useMemo(() => {
//     const index = getNoteBookIndexFromPath(activePath, notebookList)
//     if (index === -1) {
//       return {}
//     } else {
//       return notebookList[index].notebookJson
//     }
//   }, [notebookList, activePath])
// }

export function useNotebookItem(path) {
  const notebookList = store.getState().notebook.notebookList
  return useMemo(() => {
    const index = getNoteBookIndexFromPath(path, notebookList)
    if (index === -1) {
      return {}
    } else {
      return notebookList[index]
    }
  }, [notebookList, path])
}


export function getTabInfoFn(path) {
  const fileList = store.getState().filesTab.fileList
  return () => {
    const index = findFileListIndex(fileList, path)
    if (index === -1) {
      return {}
    } else {
      return fileList[index]
    }
  }
}
