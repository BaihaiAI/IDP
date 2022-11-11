import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  clientHeight: document.body.clientHeight,   // 屏幕高度
  clientWidth: document.body.clientWidth,     // 屏幕宽度
  siderWidth: 349,                            // 左侧菜单宽度
};

export const selectClientHeight = (state) => state.config.clientHeight;
export const selectclientWidth = (state) => state.config.clientWidth;
export const selectSiderWidth = (state) => state.config.siderWidth;

export const configSlice = createSlice({
  name: 'config',
  initialState,
  reducers: {
    updateClientHeight: (state, action) => {
      state.clientHeight = action.payload;
    },
    updateClientWidth: (state, action) => {
      state.clientWidth = action.payload;
    },
    updateSideWidth: (state, action) => {
      state.siderWidth = action.payload;
    }
  }
});

export const {
  updateClientHeight, updateClientWidth, updateSideWidth
} = configSlice.actions;
export default configSlice.reducer;