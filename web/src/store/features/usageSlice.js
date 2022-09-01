import { createAsyncThunk, createSlice } from "@reduxjs/toolkit"
import dashboardApi from "../../services/dashboard"

const initialState = {
  cpuUsage: 0,
  memoryUsage: 0,
  gpuUsage: 0,
  storageUsage: 0,
}

export const selectCpuUsage = state=>state.usage.cpuUsage
export const selectGpuUsage = state=>state.usage.gpuUsage
export const selectMemoryUsage = state=>state.usage.memoryUsage
export const selectStorageUsage = state=>state.usage.storageUsage


export const getUsageThunk = createAsyncThunk(
  "/usage/getUsageThunk",
  async () => {
    const result = await dashboardApi.taskMonitorTotal()
    const data = result.data
    const cpuUsage = (data.items[0].localUsedPercent * 100).toFixed(0)
    const gpuUsage = (data.items[1].localUsedPercent * 100).toFixed(0)
    const memoryUsage = (data.items[2].localUsedPercent * 100).toFixed(0)
    const storageUsage = (data.items[3].localUsedPercent * 100).toFixed(0)
    return {
      cpuUsage,
      memoryUsage,
      gpuUsage,
      storageUsage,
    }
  }
)

export const usageSlice = createSlice({
  name: "global",
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder.addCase(getUsageThunk.fulfilled, (state, action) => {
      state.cpuUsage = action.payload.cpuUsage
      state.gpuUsage = action.payload.gpuUsage
      state.memoryUsage = action.payload.memoryUsage
      state.storageUsage = action.payload.storageUsage
    })
  },
})

export default usageSlice.reducer
