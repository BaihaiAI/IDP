import { createSlice } from "@reduxjs/toolkit"

const initialState = {
  fileList: {}
}

export const selectFileList = state => state.python.fileList

export const pythonSlice = createSlice({
  name: 'python',
  initialState,
  reducers: {
    addFile: (state, action) => {
      state.fileList[action.payload.path] = { path: action.payload.path, output: '' }
    },
    removeFile: (state, action) => {
      delete state.fileList[action.payload.path]
    },
    addFileOutput: (state, action) => {
      const output = state.fileList[action.payload.path].output
      const nextOutput = action.payload.output
      state.fileList[action.payload.path] = { path: action.payload.path, output: `${output}${nextOutput}` }
    }
  }
})

export const { addFile, removeFile, addFileOutput } = pythonSlice.actions
export default pythonSlice.reducer