import { createSlice } from '@reduxjs/toolkit';

const initialState = {
    sourceData: '', // 来源数据
};

export const selectSourceView = (state) => state.workflow.sourceData;

export const workflowSplic = createSlice({
    name: 'workflow',
    initialState,
    reducers: {
        updateSourceView: (state, action) => {
            state.sourceData = action.payload;
        },
    }
});

export const {
    updateSourceView
} = workflowSplic.actions;

export default workflowSplic.reducer;