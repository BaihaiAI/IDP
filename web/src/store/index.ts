import { configureStore, Selector } from "@reduxjs/toolkit";
import headerViewSlice from "./features/headerViewSlice"
import notebookSlice from "./features/notebookSlice"
import filesTab from "./features/filesTabSlice"
import configSlice from "./features/configSlice"
import global from './features/globalSlice'
import workflow from './features/workflowSplic';
import usage from "./features/usageSlice"
import pythonSlice from "./features/pythonSlice"
import kernelSlice from "./features/kernelSlice"


export const store = configureStore({
  reducer: {
    notebook: notebookSlice,
    headerView: headerViewSlice,
    filesTab,
    config: configSlice,
    global,
    workflow,
    usage,
    python: pythonSlice,
    kernel: kernelSlice
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: false,
    }),
  devTools: process.env.NODE_ENV === "development",
})


// RootState作用是返回store的方法getState的类型 function
export type RootState = ReturnType<typeof store.getState>;

// AppDispatch 作用是拿到Store的dispatch方法的类型 function
export type AppDispatch = typeof store.dispatch;

export type AppSelector = Selector

