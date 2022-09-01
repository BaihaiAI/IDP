import { createAsyncThunk, createSlice } from "@reduxjs/toolkit"
import { validateImageUrl } from "../../utils/storage"
// import defaultAvatarImg from "../../assets/image/portrait.svg"
import defaultAvatarImg from "@/assets/portrait.svg"

const initialState = {
  avatarUrl: defaultAvatarImg,
  operatorDecision: false,
  operatorKey: "",
  updateList: ""
}

export const selectAvatarUrl = (state) => state.global.avatarUrl
export const selectOperatorDecision = (state) => state.global.operatorDecision;
export const selectOperatorKey = (state) => state.global.operatorKey;
export const selectUpdateList = (state) => state.global.updateList;

export const handleAvatarUrlThunk = createAsyncThunk(
  "global/handleAvatarUrlThunk",
  async (url) => {
    console.log(url,'-----------')
    const isValid = await validateImageUrl(url)
    if (isValid) {
      console.log(url,'----------------')
      return url
    } else {
      return Promise.reject("error")
    }
  }
)

export const globalSlice = createSlice({
  name: "global",
  initialState,
  reducers: {
    changeOperatorDecision: (state, action) => {
      state.operatorDecision = action.payload
    },
    changeOperatorKey: (state, action) => {
      state.operatorKey = action.payload
    },
    changeUpdateList: (state, action) => {
      state.updateList = new Date().getTime()
    }
  },
  extraReducers: (builder) => {
    builder.addCase(handleAvatarUrlThunk.fulfilled, (state, action) => {
      state.avatarUrl = action.payload
    })
  },
})

export const {
  changeOperatorDecision,
  changeOperatorKey,
  changeUpdateList
} = globalSlice.actions

export default globalSlice.reducer
