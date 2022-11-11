import { createSlice } from "@reduxjs/toolkit"

const initialState = {
  kernelList: {}
}

export const selectKernelList = state => state.kernel.kernelList

export const kernelSlice = createSlice({
  name: 'kernel',
  initialState,
  reducers: {
    updateKernelList: (state, action) => {
      let kernelList = {}
      for (const kernel of action.payload) {
        kernelList[kernel.notebookPath] = kernel
      }
      state.kernelList = kernelList
    },
    removeKernel: (state, action) => {
      delete state.kernelList[action.payload.path]
    },
    removeAllKernel: (state, action) => {
      state.kernelList = {}
    }
  }
})

export const { updateKernelList, removeKernel, removeAllKernel } = kernelSlice.actions
export default kernelSlice.reducer
