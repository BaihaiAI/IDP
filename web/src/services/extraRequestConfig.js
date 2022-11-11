

// request额外的全局配置、
const extraRequestConfig = {
  needErrMsg:true
}

export function needRequestErrMsg() {
  extraRequestConfig.needErrMsg = true
}

export function unNeedRequestErrMsg() {
  extraRequestConfig.needErrMsg = false
}



export default extraRequestConfig
