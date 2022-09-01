import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  lineNumbers: true,          // 编辑器显示行号
  collapseAllInput: false,    // 折叠所有输入
  collapseAllOutput: false,   // 折叠所有输出
  autoWarpOutput: true, // 默认面板中输出字符是否换行，默认换行
  sneltoetsListVisible: false, // 快捷键组建是否显示
  feedbackView:false,
};

export const selectLineNumbers = (state) => state.headerView.lineNumbers;
export const selectCollapseAllInput = (state) => state.headerView.collapseAllInput;
export const selectCollapseAllOutput = (state) => state.headerView.collapseAllOutput;
export const selectAutoWarpOutput = state => state.headerView.autoWarpOutput;
export const selectSneltoetsListVisible = (state) => state.headerView.sneltoetsListVisible;
export const selectFeedbackView = state=>state.headerView.feedbackView

export const headerViewSlice = createSlice({
  name: 'headerView',
  initialState,
  reducers: {
    openFeedbackView:(state)=>{
      state.feedbackView = true
    },
    closeFeedbackView:(state)=>{
      state.feedbackView = false
    },
    updateLineNumbers: (state, action) => {
      state.lineNumbers = action.payload;
    },
    updateCollapseAllInput: (state, action) => {
      state.collapseAllInput = action.payload;
    },
    updateCollapseAllOutput: (state, action) => {
      state.collapseAllOutput = action.payload;
    },
    updateAutoWarpOutput: (state, action) => {
      state.autoWarpOutput = action.payload;
    },
    updateSneltoetsListVisible: (state, action) => {
      state.sneltoetsListVisible = action.payload;
    },
  }
});

export const {
  openFeedbackView,
  closeFeedbackView,
  updateLineNumbers,
  updateCollapseAllInput,
  updateCollapseAllOutput,
  updateAutoWarpOutput,
  updateSneltoetsListVisible
} = headerViewSlice.actions;
export default headerViewSlice.reducer;
