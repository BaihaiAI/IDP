import { createSlice, createAsyncThunk } from "@reduxjs/toolkit"
import contentApi from "../../services/contentApi"
import kernelApi from "../../services/kernelApi"
import variableManagerApi from "../../services/variableManagerApi"
import intl from "react-intl-universal"
import { projectId } from "../cookie"
import { getHistoryOpenFile } from "@/utils/storage"

import { changeOperatorDecision } from './globalSlice'

const initialState = {
  // 将当前的一系列对象 改成数组
  notebookList: [],

  /*  path: "",
  isExecuting: false, // notebook 是否在执行
  notebookJson: {}, // notebook 内容
  cells: [], // cell集合
  metadata: {}, // meta信息
  cellProps: {}, // cell 属性
  variableList: [],*/

  // 回撤 cell
  retractableCellsList: [],
  // 反向撤回 cell
  reverseWithdrawalCellsList: [],
}

/*export const selectIsExecuting = (state) => state.notebook.isExecuting
export const selectNotebookJson = (state) => {
  return { ...state.notebook.notebookJson, cells: state.notebook.cells }
}
export const selectCells = (state) => state.notebook.cells
export const selectMetadata = (state) => state.notebook.metadata
export const selectCellProps = (state) => state.notebook.cellProps
export const selectPath = (state) => state.notebook.path
export const selectVariableList = (state) => state.notebook.variableList*/
export const selectNotebookList = (state) => state.notebook.notebookList
export const selectRetractableCellsList = (state) => state.notebook.retractableCellsList;
export const selectReverseWithdrawalCellsList = (state) => state.notebook.reverseWithdrawalCellsList;

export function getNoteBookIndexFromPath(path, notebookList) {
  return notebookList.findIndex((item) => item.path === path)
}

// 获取variableList 列表
export const variableListAsync = createAsyncThunk(
  "notebook/content/variableListAsync",
  async ({path, inode}) => {
    // const testPath = '/lz/t1.ipynb'
    const response = await variableManagerApi.getResourceVars(path, inode)
    return { path, response }
  }
)

// 读取ipynb文件
export const contentCatAsync = createAsyncThunk(
  "content/cat",
  async (options) => {
    const { path, name, suffix, posLine, posCellId } = options
    const response = await contentApi.cat(options)
    return { response:response.data, path, name, suffix, posLine, posCellId }
  }
)

// 获取ipynb文件状态
export const kernelExecuteStateAsync = createAsyncThunk(
  "kernel/executeState",
  async ({ path }) => {
    const response = await kernelApi.executeState({ path })
    return { response:response.data, path }
  }
)

// 全量覆盖些，使用时慎重
export const contentSaveAsync = createAsyncThunk(
  "content/save",
  async (_, { getState }) => {
    const notebook = getState().notebook
    const { path, notebookJson, cells } = notebook
    if (!path || !cells) return
    const options = {
      content: JSON.stringify({ ...notebookJson, cells: cells }),
      path: path,
      type: "ipynb",
    }
    const response = await contentApi.save(options)
    return response.data
  }
)

export const contentAddCell = createAsyncThunk(
  "content/addCell",
  async ({ path, index, cellType,cells }, { getState, dispatch }) => {
    // if (!path || path !== getState().notebook.path) return
    getState()?.notebook?.retractableCellsList.length !==0 && dispatch(clearPopupList()) && dispatch(resolveClearPopupList())
    let insertFlag = 0, aboveCellIndex = null,underCellIndex =null
    if(index===0){
      insertFlag = 1
      aboveCellIndex = cells.length === 0 ? 1 : cells[index].metadata.index
    }else if(index === cells.length){
      insertFlag = 2
      underCellIndex = cells[index-1].metadata.index
    }else{
      aboveCellIndex =  cells[index-1].metadata.index
      underCellIndex = cells[index].metadata.index
    }

    const options = {
      path,
      index,
      type: cellType,
      insertFlag,
      aboveCellIndex,
      underCellIndex
    }
    const response = await contentApi.cellAdd(options)
    return { data:response.data, path, index }
  }
)

// 撤回cell
export const contentWithdrawCell = createAsyncThunk(
  "content/withdrawCell",
  async ({path, withdrawCell},{getState, dispatch}) =>{
    const response = await contentApi.withdrawCell({path, cell: withdrawCell["deletedCellBody"]})
    dispatch(popUpTheCurrentRecallList(withdrawCell))
    return {data:response.data, path, index: withdrawCell["index"]}
  }
)


// 公共代码部分插入 代码片段
export const InsertCodeSnippet = createAsyncThunk(
  "content/InsertCode",
  async ({path, cells, currentIndex}, {getState, dispatch}) => {
    let data = [];
    for (const cell of cells) {
      const options = {
        path,
        cell,
      }
      const response = await contentApi.withdrawCell(options)
      data.push(response.data)
      dispatch(changeOperatorDecision(false))
    }

    return { data, path, currentIndex }
  }
)

export const contentDelCell = createAsyncThunk(
  "content/delCell",
  async ({ path, index, cellId, bol }, { getState, dispatch }) => {
    // bol 是判断 删除操作还是 反向撤回操作 false 为删除
    // if (!path || path !== getState().notebook.path) return
    const { activePath } = getState()?.filesTab;
    let deletedCellLocation = (getState().notebook.notebookList.filter(cellloc => cellloc.path === activePath))[0];
    let deletedCellBody = (deletedCellLocation.cells.filter(cellbody => cellbody["metadata"].id === cellId))[0];
    const payload = {
      deletedCellBody,
      index
    }
    let reverseLoc = getState().notebook.reverseWithdrawalCellsList;

    // dispatch(depositWithdrawalList(payload))
    if(bol){
      dispatch(depositWithdrawalList(payload))
    }else{
      if(reverseLoc.length){
        dispatch(resolveClearPopupList())
        dispatch(depositWithdrawalList(payload))
      }else{
        dispatch(depositWithdrawalList(payload))
      }
    }

    const options = {
      path,
      index,
      id: cellId,
    }
    const response = await contentApi.cellDel(options)
    return { data:response.data, path }
  }
)

export const contentMoveCell = createAsyncThunk(
  "content/moveCell",
  async ({ path, cellId,neighborCellId }, { getState }) => {
    // if (!path || path !== getState().notebook.path) return
    const options = {
      path,
      id: cellId,
      neighborCellId
    }
    const response = await contentApi.cellMove(options)
    return response.data
  }
)

// 保存版本
export const contentSnapshot = createAsyncThunk(
  "content/snapshot",
  async ({ path }, { getState }) => {

    const notebook = getState().notebook
    // if (!path || path !== notebook.path) return
    const index = getNoteBookIndexFromPath(path, notebook.notebookList)

    const params = {
      path,
      label: intl.get("SAVE_VERSION_AUTO"),
    }
    const response = await contentApi.snapshot(params)
    return response.data
  }
)

// 修改所有cell的输入
export const contentUpdateAllCellSource = createAsyncThunk(
  "content/updateAllCellSource",
  async ({ path }, { getState }) => {
    const notebook = getState().notebook
    // if (!path || path !== notebook.path) return
    const index = getNoteBookIndexFromPath(path, notebook.notebookList)
    const response = await contentApi.cellUpdateField(
      path,
      notebook.notebookList[index].cells,
      ["source", "metadata"]
    )
    return response.data
  }
)

// 修改指定cell的输入
export const contentUpdateCellSource = createAsyncThunk(
  "content/updateCellSource",
  async ({ path, cellId }, { getState }) => {
    const notebook = getState().notebook
    // if (!path || path !== notebook.path) return
    const index = getNoteBookIndexFromPath(path, notebook.notebookList)
    const notebookItem = notebook.notebookList[index]
    let cells = []
    for (const cell of notebookItem.cells) {
      if (cell.metadata.id === cellId) {
        cells.push({ ...cell })
        break
      }
    }
    if (cells.length === 0) return
    const response = await contentApi.cellUpdateField(path, cells, [
      "source",
      "metadata",
    ])
    return response.data
  }
)

// 修改所有cell的输出
export const contentUpdateAllCellOutputs = createAsyncThunk(
  "content/updateAllCellOutputs",
  async ({ path }, { getState }) => {
    const notebook = getState().notebook
    // if (!path || path !== notebook.path) return
    const index = getNoteBookIndexFromPath(path, notebook.notebookList)
    const notebookItem = notebook.notebookList[index]
    const response = await contentApi.cellUpdateField(
      path,
      notebookItem.cells,
      ["outputs"]
    )
    return response.data
  }
)

// 修改指定cell的输出
export const contentUpdateCellOutputs = createAsyncThunk(
  "content/updateCellOutputs",
  async ({ path, cellId }, { getState }) => {
    const notebook = getState().notebook
    // if (!path || path !== notebook.path) return
    const index = getNoteBookIndexFromPath(path, notebook.notebookList)
    const notebookItem = notebook.notebookList[index]
    let cells = []
    for (const cell of notebookItem.cells) {
      if (cell.metadata.id === cellId) {
        cells.push(cell)
        break
      }
    }
    if (cells.length === 0) return
    const response = await contentApi.cellUpdateField(path, cells, ["outputs"])
    return response.data
  }
)

export const updateNotebookListFromTabListAsync = createAsyncThunk(
  "content/updateNotebookListFromTabList",
  (_, { getState }) => {
    const fileList = getState().filesTab.fileList
    return fileList
  }
)

const judgmentIsExecuting = (cellProps) => {
  let isExecuting = false
  for (const key in cellProps) {
    isExecuting = isExecuting || cellProps[key].state !== "ready"
  }
  return isExecuting
}

// 判断文件对应的kernel是否被暂停
const judgmentIsPaused = (cellProps) => {
  let isPaused = false
  for (const key in cellProps) {
    isPaused = isPaused || cellProps[key].state === "paused"
    if (isPaused) break
  }
  return isPaused
}

/*const updateCellIndex = (cells) => {
  for (let i = 0; i < cells.length; i++) {
    cells[i].metadata.index = i
  }
}*/

export const notebookSlice = createSlice({
  name: "notebook",
  initialState,
  reducers: {
    // 存入删除（可以回撤的）cell
    depositWithdrawalList(state, action) {
      const { payload } = action;
      state.retractableCellsList.push(payload)
    },
    // 回撤操作 弹出最后一位 并将最后一位存入 反回撤 数组
    popUpTheCurrentRecallList(state, action) {
      const { payload } = action;
      state.reverseWithdrawalCellsList.push(payload);
      state.retractableCellsList.pop();
    },
    // 清空 可撤回/ 的数组
    clearPopupList(state, action) {
      state.retractableCellsList = [];

    },
    // 清空 可反向撤回 的数组
    resolveClearPopupList(state, action){
      state.reverseWithdrawalCellsList = [];
    },
    // 反向回撤操作 ， 弹出最后最后一位
    popUpTheCurrentReverseWithdrawal(state, action){
      state.reverseWithdrawalCellsList.pop()
    },

    clearOtherAllNotebookList(state, action) {
      const path = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return
      state.notebookList = [state.notebookList[index]]
    },

    clearNotebookList(state, action) {
      state.notebookList = []
    },

    resetNotebookState: (state, action) => {
      // payload中 需要有path
      const path = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return
      state.notebookList.splice(index, 1)

      /*      state.path = ""
      state.isExecuting = false
      state.notebookJson = {}
      state.cells = []
      state.metadata = {}
      state.cellProps = {}
      state.variableList = []*/
    },
    updatePath: (state, action) => {
      const { path, newPath } = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return
      state.notebookList[index].path = newPath
    },
    updateNotebookJson: (state, action) => {
      // action中包含path和notebookJson
      const { path, notebookJson } = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return

      const metadata = notebookJson.metadata
      const cells = notebookJson.cells
      let cellProps = {}
      let focusOne = false
      for (const cell of notebookJson.cells) {
        const cellType = cell["cell_type"]
        let focus = false
        if (!focusOne && (cellType === "code" || cellType === "sql")) {
          focusOne = true
          focus = true
        }
        const cellProp = {
          state: "ready",
          focus: focus,
          cellType: cellType,
        }
        cellProps[cell.metadata.id] = cellProp
      }

      state.notebookList[index] = {
        ...state.notebookList[index],
        notebookJson,
        metadata,
        cells,
        cellProps,
      }
    },
    updateCells: (state, action) => {
      // action中包含path和cells
      const { path, cells } = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return
      state.notebookList[index].cells = cells
    },
    updateCell: (state, action) => {
      // action中包含path和cellId和cell
      const { cellId, cell, path,isError } = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return
      const notebookItem = { ...state.notebookList[index] }
      for (let i = 0; i < notebookItem.cells.length; i++) {
        if (notebookItem.cells[i].metadata.id === cellId) {
          notebookItem.cells[i] = cell
          break
        }
      }
      if(isError){
        notebookItem.cellProps[cellId].hasExecuted = false
      }
      state.notebookList[index] = notebookItem
    },
    updateCellSource: (state, action) => {
      // action中包含path和cellId和source
      const { cellId, source, path } = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return

      const notebookItem = { ...state.notebookList[index] }
      for (let cell of notebookItem.cells) {
        if (cell.metadata.id === cellId) {
          cell["source"] = [...source]
          break
        }
      }
      state.notebookList[index] = notebookItem
    },
    updateCellMetadata: (state, action) => {
      // action中包含path和cellId和metadata
      const { cellId, metadata, path } = action.payload
      const index = getNoteBookIndexFromPath(path, state.notebookList)
      if (index === -1) return

      const notebookItem = { ...state.notebookList[index] }
      for (let cell of notebookItem.cells) {
        if (cell.metadata.id === cellId) {
          cell["metadata"] = { ...cell["metadata"], ...metadata }
          break
        }
      }
      state.notebookList[index] = notebookItem
    },
    delCell: (state, action) => {
      // action中包含index和path
      const { index, path } = action.payload
      const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
      if (notebookIndex === -1) return

      const notebookItem = { ...state.notebookList[notebookIndex] }
      // 删除 cellProps中对应的数据
      const cellId = notebookItem.cells[index]?.metadata.id
      delete notebookItem.cellProps[cellId]
      // 重新调用 judgmentIsExecuting方法
      notebookItem.isExecuting = judgmentIsExecuting(notebookItem.cellProps)
      notebookItem.cells.splice(index, 1)
      // updateCellIndex(notebookItem.cells)
      state.notebookList[notebookIndex] = notebookItem
    },
    moveCell: (state, action) => {
      // action中包含originIndex  targetIndex path
      const { originIndex, targetIndex, path } = action.payload
      const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
      if (notebookIndex === -1) return

      const notebookItem = { ...state.notebookList[notebookIndex] }

      const tmpCell = {
        ...notebookItem.cells[originIndex],
        metadata: {
          ...notebookItem.cells[originIndex].metadata,
          index: notebookItem.cells[targetIndex].metadata.index,
        },
      }
      notebookItem.cells[originIndex] = {
        ...notebookItem.cells[targetIndex],
        metadata: {
          ...notebookItem.cells[targetIndex].metadata,
          index: notebookItem.cells[originIndex].metadata.index,
        },
      }
      notebookItem.cells[targetIndex] = tmpCell
      state.notebookList[notebookIndex] = notebookItem
    },
    updateCellProps: (state, action) => {
      // action中包含 cellProps path
      const { cellProps, path } = action.payload
      const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
      if (notebookIndex === -1) return

      const notebookItem = { ...state.notebookList[notebookIndex] }
      notebookItem.cellProps = cellProps
      notebookItem.isExecuting = judgmentIsExecuting(cellProps)
      notebookItem.isPaused = judgmentIsPaused(cellProps)
      state.notebookList[notebookIndex] = notebookItem
    },
    updateCellProp: (state, action) => {
      // action中包含 cellProp cellId path

      const { cellId, cellProp, path } = action.payload
      // console.log(cellId,'执行了获取焦点操作')
      const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
      if (notebookIndex === -1) return

      const notebookItem = { ...state.notebookList[notebookIndex] }

      let cellProps = { ...notebookItem.cellProps }
      let cells = state.notebookList[notebookIndex].cells

      if (cellId in cellProps) {
        cellProps[cellId] = { ...cellProps[cellId], ...cellProp }

        if (cellProp.focus) {
          for (const key in cellProps) {
            if (key !== cellId) {
              cellProps[key].focus = false
            }
          }
        }
        notebookItem.cellProps = cellProps
        notebookItem.isExecuting = judgmentIsExecuting(cellProps)
        notebookItem.isPaused = judgmentIsPaused(cellProps)
        state.notebookList[notebookIndex] = notebookItem
      }
      if(cellProps[cellId].state === "executing"){
        for(let key in cellProps){
          cellProps[key].focus = false;
        }
        cellProps[cellId].focus = true
      }
    },
    updateCellPropState: (state, action) => {
      const { path, state: cellState} = action.payload
      const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
      if (notebookIndex === -1) return

      const notebookItem = { ...state.notebookList[notebookIndex] }
      let cellProps = { ...notebookItem.cellProps }
      for (const key in cellProps) {
        cellProps[key].state = cellState
      }
      notebookItem.cellProps = cellProps
      notebookItem.isExecuting = judgmentIsExecuting(cellProps)
      notebookItem.isPaused = judgmentIsPaused(cellProps)
      state.notebookList[notebookIndex] = notebookItem
    },
    updateCellPropFocus: (state, action) => {
      // action中包含  cellId path
      const { cellId, path } = action.payload
      const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
      if (notebookIndex === -1) return

      const notebookItem = { ...state.notebookList[notebookIndex] }
      for (const key in notebookItem.cellProps) {
        notebookItem.cellProps[key].focus = key === cellId
      }
      state.notebookList[notebookIndex] = notebookItem
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(variableListAsync.fulfilled, (state, action) => {
        const {
          payload: {
            response: { data },
            path,
          },
        } = action
        const index = getNoteBookIndexFromPath(path, state.notebookList)
        if (index === -1) return
        // console.log(data)
        state.notebookList[index].variableList = JSON.parse(data)
      })
      .addCase(contentCatAsync.fulfilled, (state, action) => {
        const { path, name, suffix, posLine, posCellId } = action.payload

        if(action.payload.response.contentType!=='notebook'){
          return
        }

        // 去掉json.parse 目前content本身就是对象了
        const content = action.payload.response.content
        const stringContent = JSON.stringify(content)
        const notebookJson = content
        const metadata = content.metadata
        const cells = content.cells
        let cellProps = {}

        let focusOne = false
        for (const cell of content.cells) {
          const cellType = cell["cell_type"]
          let focus = false
          if (!focusOne && (cellType === "code" || cellType === "sql")) {
            focusOne = true
            focus = true
          }
          const cellProp = {
            state: "ready",
            focus: focus,
            cellType: cellType,
          }
          cellProps[cell.metadata.id] = cellProp
        }
        // 如果在local中 存在 该notebook信息 并且存在focusId 并且focusId在当前的cellProp中是一个对象
        // 则替换 cellProp中的 focusId
        let historyOpenFileObj = getHistoryOpenFile()

        const findResult =
          Array.isArray(historyOpenFileObj[projectId]) &&
          historyOpenFileObj[projectId].find(
            (item) => item.name === content.path && item.focusId
          )
        if (findResult && cellProps[findResult.focusId]) {
          Object.keys(cellProps).forEach((id) => {
            cellProps[id].focus = id === findResult.focusId
          })
        }

        // 如果是搜索打开的文件定位到具体的cell和行号
        if (posLine && posCellId) {
          for (const id in cellProps) {
            if (id === posCellId) {
              cellProps[id].focus = true
              cellProps[id].posLine = posLine
            } else {
              cellProps[id].focus = false
            }
          }
        }

        // 存入notebookList对应的item中
        const index = getNoteBookIndexFromPath(path, state.notebookList)
        if (index === -1) {
          state.notebookList.push({
            isExecuting: false,
            variableList: [],
            notebookJson,
            metadata,
            cells,
            cellProps,
            path,
            name,
            suffix,
            content: stringContent,
          })
        } else {
          state.notebookList[index] = {
            isExecuting: false,
            variableList: [],
            notebookJson,
            metadata,
            cells,
            cellProps,
            path,
            name,
            suffix,
            content: stringContent,
          }
        }
      })
      .addCase(contentCatAsync.rejected, (state, err) => {

      })
      .addCase(kernelExecuteStateAsync.fulfilled, (state, action) => {
        const { response, path } = action.payload
        let isExecuting = false
        const index = getNoteBookIndexFromPath(path, state.notebookList)
        if (index === -1) return

        let cellProps = state.notebookList[index].cellProps
        for (const cell of response) {
          if (cell.cellId in cellProps) {
            let cellState = cell.state
            if ("RUNNING" === cellState) {
              cellState = "executing"
            } else if ("PENDING" === cellState) {
              cellState = "pending"
            } else if ("PAUSED" === cellState) {
              cellState = "paused"
            } else {
              cellState = "ready"
            }
            cellProps[cell.cellId].state = cellState
            isExecuting = isExecuting || cellState !== "ready"
          }
        }
        state.notebookList[index].cellProps = cellProps
        state.notebookList[index].isExecuting = isExecuting
        state.notebookList[index].isPaused = judgmentIsPaused(cellProps)
      })
      .addCase(contentSaveAsync.fulfilled, (state, action) => {
        return action.payload
      })
      .addCase(contentAddCell.fulfilled, (state, action) => {
        const {
          data,
          path,
          index
        } = action.payload
        const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
        let cell = data
        if (notebookIndex === -1) return

        // sql cell 补充dfName和datasource
        if (cell.cell_type === "sql") {
          let metadata = cell.metadata;
          if (!metadata.dfName || metadata.dfName === "") {
            metadata.dfName = 'df_0'
          }
          if (!metadata.dataSource || metadata.dataSource === "") {
            metadata.dataSource = 'local_csv'
          }
        }

        // visualization 补df_name字段
        if (cell.cell_type === "visualization") {
          let metadata = cell.metadata;
          if (!metadata.df_name) {
            metadata.df_name = ''
          }
        }

        const notebook = { ...state.notebookList[notebookIndex] }
        notebook.cells.splice(index, 0, cell)
        // updateCellIndex(notebook.cells)

        // 更新cellProp
        let nextCellprops = { ...notebook.cellProps }
        for (const key in nextCellprops) {
          nextCellprops[key].focus = false
        }
        const cellProp = {
          focus: true,
          state: "ready",
          type: cell["cell_type"],
        }
        nextCellprops[cell.metadata.id] = cellProp
        notebook.cellProps = nextCellprops
        state.notebookList[notebookIndex] = notebook
      })
      .addCase(contentWithdrawCell.fulfilled, (state, action) => {
        const {
          data,
          path,
          index
        } = action.payload


        const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
        const cell = data
        if (notebookIndex === -1) return

        const notebook = { ...state.notebookList[notebookIndex] }
        console.log(notebook, "notebook--------------")
        notebook.cells.splice(index, 0, cell)
        // updateCellIndex(notebook.cells)

        // 更新cellProp
        let nextCellprops = { ...notebook.cellProps }
        for (const key in nextCellprops) {
          nextCellprops[key].focus = false
        }
        const cellProp = {
          focus: true,
          state: "ready",
          type: cell["cell_type"],
        }
        nextCellprops[cell.metadata.id] = cellProp
        notebook.cellProps = nextCellprops
        state.notebookList[notebookIndex] = notebook
      })
      .addCase(InsertCodeSnippet.fulfilled, (state, action) => {
        const {
          data,
          path,
          currentIndex,
        } = action.payload
        // console.log(action.payload)
        const notebookIndex = getNoteBookIndexFromPath(path, state.notebookList)
        if (notebookIndex === -1) return
        const notebook = { ...state.notebookList[notebookIndex] }
        for (let i = 0; i < data.length; i++) {
          const cell = data[i]
          const nextIndex = currentIndex + 1 + i
          notebook.cells.splice(nextIndex, 0, cell)
          // updateCellIndex(notebook.cells)

          // 更新cellProp
          let nextCellprops = { ...notebook.cellProps }
          for (const key in nextCellprops) {
            nextCellprops[key].focus = false
          }
          const cellProp = {
            focus: true,
            state: "ready",
            type: cell["cell_type"],
          }
          nextCellprops[cell.metadata.id] = cellProp
          notebook.cellProps = nextCellprops
        }

        state.notebookList[notebookIndex] = notebook
      })
      .addCase(contentDelCell.fulfilled, (state, action) => {})
      .addCase(contentSnapshot.fulfilled, (state, action) => {})
      .addCase(
        updateNotebookListFromTabListAsync.fulfilled,
        (state, action) => {
          const fileList = action.payload
          const newNotebookList = []
          for (let i = 0; i < state.notebookList.length; i++) {
            const notebook = state.notebookList[i]
            if (fileList.find((item) => item.path === notebook.path)) {
              newNotebookList.push(notebook)
            }
          }
          state.notebookList = newNotebookList
        }
      )
  },
})

export const {
  resetNotebookState,
  updatePath,
  updateNotebookJson,
  updateCellProps,
  updateCellProp,
  updateCellPropState,
  updateCellPropFocus,
  updateCell,
  updateCellSource,
  updateCellMetadata,
  delCell,
  moveCell,
  depositWithdrawalList,
  popUpTheCurrentRecallList,
  clearPopupList,
  popUpTheCurrentReverseWithdrawal,
  resolveClearPopupList
} = notebookSlice.actions

export default notebookSlice.reducer

