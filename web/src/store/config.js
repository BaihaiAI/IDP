import { region, projectId } from "./cookie"

let wsOrigin = (window.location.protocol === 'https:' ? 'wss:' : 'ws:') + `//${window.location.host}`
if (Boolean(process.env.NODE_OPEN)) {
  const pathname = window.location.pathname
  const path = pathname.substring(0, pathname.indexOf('/workspace'))
  if (path !== '') {
    wsOrigin = `${wsOrigin}${path}`
  }
}
export const kernelWsSendUrl = `${wsOrigin}/${region}/api/v1/execute/ws/kernel/execute?projectId=${projectId}`
export const lspWsUrl = `${wsOrigin}/${region}/api/v1/lsp/lsp/`
export const terminalWsUrl = `${wsOrigin}/${region}/api/v1/terminal/socket/`
export const pythonWsUrl = `${wsOrigin}/${region}/api/v2/idp-note-rs/exec_code/exec_python`

let currentEnv = null
export const getCurrentEnv = () => {
  return currentEnv
}
export const setCurrentEnv = (env) => {
  currentEnv = env
}

// ['HOST','SAAS']
export const projectVersion = process.env.REACT_APP_VERSION
