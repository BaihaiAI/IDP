import {createAsyncThunk, createSlice} from "@reduxjs/toolkit"
import {
  changeHistoryOpenFileIndex,
  putHistoryOpenFile,
} from "@/utils/storage"
import { projectId } from "../cookie"

import contentApi from "../../services/contentApi"


const initialState = {
  fileList: [],
  activePath: "",
}

export const findFileListIndex = function (fileList, path) {
  return fileList.findIndex((item) => item.path === path)
}

export const selectTabList = (state) => state.filesTab.fileList
export const selectActivePath = (state) => state.filesTab.activePath


export const addFileAndContentAsync = createAsyncThunk(
  "/addFileContentAsync",
  async ({ name, path, rootPath, suffix, posLine, posCellId,notNeedChangePath }) => {
    const response = await contentApi.cat({ path })
    return {
      response: response.data,
      path,
      name,
      rootPath,
      suffix,
      posLine,
      posCellId,
      notNeedChangePath
    }
  }
)

export const filesTabSlice = createSlice({
  name: "filesTab",
  initialState,
  reducers: {
    // removeChildrenPath

    clearTabsFromList(state, action) {
      const removeList = action.payload
      const isActivePathInRemove = removeList.indexOf(state.activePath) !== -1
      const newList = state.fileList.filter((item) => {
        const path = item.path
        return !removeList.find((removeItem) => removeItem === path)
      })
      if (isActivePathInRemove) {
        state.activePath = newList.length ? newList[0].path : ""
      }
      state.fileList = newList
      putHistoryOpenFile(
        projectId,
        state.fileList.map((item) => ({ name: item.path, status: "open" }))
      )
    },
    clearLeftOrRightFileList(state, action) {
      // type 分为left和right
      const { path, type } = action.payload
      const index = findFileListIndex(state.fileList, path)
      if (
        index === -1 ||
        (index === 0 && type === "left") ||
        (index === state.fileList.length - 1 && type === "right")
      ) {
        return
      }
      const activeIndex = findFileListIndex(state.fileList, state.activePath)
      if (type === "left") {
        state.fileList = state.fileList.slice(index)
        if (activeIndex < index) {
          state.activePath = state.fileList[0].path
        }
      } else if (type === "right") {
        state.fileList = state.fileList.slice(0, index + 1)
        if (activeIndex > index) {
          state.activePath = state.fileList[state.fileList.length - 1].path
        }
      }
      putHistoryOpenFile(
        projectId,
        state.fileList.map((item) => ({ name: item.path, status: "open" }))
      )
    },

    clearOtherAllFileList(state, action) {
      const newActivePath = action.payload
      const index = findFileListIndex(state.fileList, newActivePath)
      if (index === -1) return
      state.fileList = [state.fileList[index]]
      state.activePath = newActivePath
      putHistoryOpenFile(
        projectId,
        state.fileList.map((item) => ({ name: item.path, status: "open" }))
      )
    },

    clearFileList(state, action) {
      state.fileList = []
      state.activePath = ""
      putHistoryOpenFile(projectId, state.fileList)
    },

    updateFileDeleteFlag(state, action){
      const path = action.payload
      const index = findFileListIndex(state.fileList, path)
      if(index===-1) return
      state.fileList[index].deleteFlag = true
    },

    addNewFile(state, action) {

      // name: "index.txt"
      // path: "/index.txt"
      // rootPath: "/store/1555040580472561664/projects/116/notebooks"
      // suffix: "txt"

      const { path, name, suffix, posLine, posCh, posCellId,notNeedChangePath} =
        action.payload
      const index = findFileListIndex(state.fileList, path)
      if (index === -1) {
        state.fileList.push({
          path,
          name,
          suffix,
          posLine,
          posCh,
          posCellId,
          deleteFlag:false
        })
      }
      if(!notNeedChangePath){
        state.activePath = path
      }
    },
    updateFileProp(state, action) {
      const { path, newProps } = action.payload
      const index = findFileListIndex(state.fileList, path)
      if (index === -1) return
      state.fileList[index] = { ...state.fileList[index], ...newProps }
    },
    deleteFile(state, action) {
      const path = action.payload
      const index = findFileListIndex(state.fileList, path)
      state.fileList.splice(index, 1)
    },
    changeActivePath(state, action) {
      const path = action.payload
      state.activePath = path
      changeHistoryOpenFileIndex({ projectId, path })
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(addFileAndContentAsync.fulfilled, (state, action) => {
        const { path, name, rootPath, suffix, response, posLine, posCellId,notNeedChangePath } =
          action.payload
        const index = findFileListIndex(state.fileList, path)
        if (index === -1) {
          // content: "a,b,c,c\n1,2,3,4"
          // contentType: "text"
          // lastModified: "2022-08-17T15:39:34+08:00"
          // length: 15
          // mime: "text/plain"

          const { content, contentType, lastModified, length, mime } = response
          state.fileList.push({
            path,
            name,
            rootPath,
            suffix,
            posLine,
            posCellId,
            content,
            contentType,
            lastModified,
            length,
            mime,
            deleteFlag: false,
          })
        }
        if(!notNeedChangePath){
          state.activePath = path
        }
      })
      .addCase(addFileAndContentAsync.rejected, () => {})
  }

})

export const {
  addNewFile,
  deleteFile,
  changeActivePath,
  updateFileProp,
  clearFileList,
  clearOtherAllFileList,
  clearLeftOrRightFileList,
  clearTabsFromList,
  updateFileDeleteFlag,
} = filesTabSlice.actions

export default filesTabSlice.reducer
