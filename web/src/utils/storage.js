import {projectId, userId} from "@/store/cookie"
/*url相关的操作*/
function checkImgExists(imgUrl) {
  return new Promise(function (resolve, reject) {
    let ImgObj = new Image()
    ImgObj.src = imgUrl
    ImgObj.onload = function (res) {
      resolve(res)
    }
    ImgObj.onerror = function (err) {
      reject(err)
    }
  })
}
export async function validateImageUrl(url) {
  let isValid = true
  try {
    await checkImgExists(url)
  } catch (err) {
    isValid = false
  }
  return isValid
}

export const defaultAvatarUrl = `/0/api/v1/user/image/profile_photo/${userId}.jpg`

export async function saveUserAvatar(avatar) {
  if (typeof avatar !== "string") {
    return
  }
  const isValid = await validateImageUrl(avatar)
  if (isValid) {
    localStorage.setItem("avatar", avatar)
  }
}

/*historyOpenFile相关的操作*/


export function initHistoryFile() {
  let historyOpenFileObj = JSON.parse(
    window.localStorage.getItem("historyOpenFile")
  )
  if (Array.isArray(historyOpenFileObj)) {
    historyOpenFileObj = { [projectId]: [] }
    window.localStorage.setItem(
      "historyOpenFile",
      JSON.stringify(historyOpenFileObj)
    )
  }
}

// 缓存打开文件
export const handlerSaveHistoryOpenFile = (key, name, status) => {
  let fileName = key.endsWith(name) ? key : key + name
  let historyOpenFileObj = getHistoryOpenFile()
  if (Array.isArray(historyOpenFileObj[projectId])) {
    let openFiles = []
    let focusId
    for (const file of historyOpenFileObj[projectId]) {
      if (file.name) {
        // 删掉异常数据
        if (file.name !== fileName) {
          openFiles.push(file)
        } else {
          // 如果本地缓存中有focusId 则保存时 把focusId也带上
          if (file.focusId) {
            focusId = file.focusId
          }
        }
      }
    }
    openFiles.unshift({ name: fileName, focusId, status })
    if (openFiles.length > 10) {
      openFiles = openFiles.slice(0, 10)
    }
    historyOpenFileObj[projectId] = openFiles
  } else {
    let openFiles = [{ name: fileName, status }]
    historyOpenFileObj[projectId] = openFiles
  }
  saveHistoryOpenFile(historyOpenFileObj)
}

// 重命名和删除文件时删除缓存
export const historyDelFile = (key) => {
  const cacheFileObj = getHistoryOpenFile()
  if (Array.isArray(cacheFileObj[projectId])) {
    let openFiles = []
    for (const file of cacheFileObj[projectId]) {
      if (file.name) {
        // 删掉异常数据
        if (file.name !== key) openFiles.push(file)
      }
    }
    cacheFileObj[projectId] = openFiles
    saveHistoryOpenFile(cacheFileObj)
  }
}


export function getHistoryOpenFile() {
  const historyOpenFileJson = window.localStorage.getItem("historyOpenFile")
  return JSON.parse(historyOpenFileJson || "{}")
}
export function saveHistoryOpenFile(historyOpenFileObj = {}) {
  window.localStorage.setItem(
    "historyOpenFile",
    JSON.stringify(historyOpenFileObj)
  )
}

export function changeHistoryFileOpenOrClose({
  status = "open",
  projectId,
  path,
}) {
  // status 分为open 或者close
  const openFileObj = getHistoryOpenFile()
  const projectList = openFileObj[projectId]
  if (Array.isArray(projectList)) {
    const index = projectList.findIndex((item) => item.name === path)
    if (index === -1) return
    projectList[index].status = status
  }
  saveHistoryOpenFile(openFileObj)
}

export function changeHistoryOpenFileIndex({ projectId, path }) {
  const openFileObj = getHistoryOpenFile()
  const projectList = openFileObj[projectId]
  const newProjectList = []
  let firstFile
  for (let i = 0; i < projectList.length; i++) {
    const element = projectList[i]
    if (element.name !== path) {
      newProjectList.push(element)
    } else {
      firstFile = { ...element }
    }
  }
  // 如果有firstFile的话 firstFile有可能是undefined
  if (firstFile) {
    newProjectList.unshift(firstFile)
  }

  openFileObj[projectId] = newProjectList
  saveHistoryOpenFile(openFileObj)
}

export function putHistoryOpenFile(projectId, projectList = []) {
  const openFileObj = getHistoryOpenFile()
  const oldProjectList = openFileObj[projectId]
  if (oldProjectList && oldProjectList.length > 0) {
    for (let i = 0; i < oldProjectList.length; i++) {
      const oldProjectListElement = oldProjectList[i]
      const findResult = projectList.find(
        (item) => item.name === oldProjectListElement.name
      )
      if (!findResult) {
        oldProjectListElement.status = "close"
      } else {
        oldProjectListElement.status = "open"
      }
    }
  }

  saveHistoryOpenFile(openFileObj)
}

/*global_keyword_search相关的记录*/

/*结构
{
  [projectId]:[string,string]
}

*/

export function getGlobalKeywordSearch() {
  const globalKeywordSearch = window.localStorage.getItem(
    "global_keyword_search"
  )
  return JSON.parse(globalKeywordSearch || "{}")
}

export function saveGlobalKeywordSearch(globalKeywordSearch = {}, projectId) {
  if (Array.isArray(globalKeywordSearch[projectId])) {
    globalKeywordSearch[projectId] = globalKeywordSearch[projectId].slice(0, 5)
  }
  window.localStorage.setItem(
    "global_keyword_search",
    JSON.stringify(globalKeywordSearch)
  )
}



