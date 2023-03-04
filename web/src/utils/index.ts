import moment from "moment"
export { logout } from "./logout.js"
export { refreshPage } from "./refreshPage"
export { locationToProjectListPage } from "./locationToProjectListPage"


// todo 替换json.parse()
const safeJson = (jsonStr = '{}', defaultVal = {}) => {
  try {
    return JSON.parse(jsonStr)
  } catch (error) {
    console.warn(`${jsonStr} is not valid json`)
    return defaultVal
  }
}

export function mergeArray(arr1, ...args) {
  const arr = [...arr1]
  args.forEach((arg) => {
    arg.forEach((item) => {
      if (arr.indexOf(item) === -1) {
        arr.push(item)
      }
    })
  })

  return arr
}

/**
 * 识别当前游览器类型
 * @returns
 */
export function getBrowserType() {
  const userAgent = navigator.userAgent; //取得浏览器的userAgent字符串
  const isOpera = userAgent.indexOf("Opera") > -1;
  if (isOpera) { //判断是否Opera浏览器
    return "Opera"
  }
  if (userAgent.indexOf("Firefox") > -1) { //判断是否Firefox浏览器
    return "Firefox";
  }
  if (userAgent.indexOf("Chrome") > -1) {
    return "Chrome";
  }
  if (userAgent.indexOf("Safari") > -1) { //判断是否Safari浏览器
    return "Safari";
  }
  if (userAgent.indexOf("compatible") > -1 && userAgent.indexOf("MSIE") > -1 && !isOpera) { //判断是否IE浏览器
    return "IE";
  }
  return false
}

/**
 * 获取文件大小
 * @param {*} files
 * @returns
 */
export function getFilesSize(files) {
  // @ts-ignore
  if (!Object.prototype.toString.call(files) === '[object FileList]') return;
  let filesSize = 0;
  for (let i = 0; i < files.length; i++) {
    filesSize = files[i].size + filesSize;
  };
  return filesSize;
}

/**
 * 数字转换百分数
 * @param {*} number 数字
 * @param {*} splic 保留几位数，默认小数后2位数
 * @returns
 */
export function decimalToPercentage(number, splic = 2, unit = true) {
  return (number * 100).toFixed(splic) + (unit ? '%' : '');
}

export function getCurrentDate() {
  // @ts-ignore
  const time = new moment(new Date())
  return time.format("YYYY-MM-DD")
}
export function getCurrentNextDate() {
  // @ts-ignore
  const time = new moment(new Date(new Date().getTime() + 24 * 60 * 60 * 1000))
  return time.format("YYYY-MM-DD")
}
export function findFileOrDirName(key) {
  const arr = key.split('/')
  return arr[arr.length - 1]
}

export function findFileTreeParentKey(key) {
  const arr = key.split('/')
  let parentKey = ''
  for (let i = 1; i < arr.length - 1; i++) {
    parentKey += '/' + arr[i]
  }
  return parentKey
}

export function sliceDocumentString(string) {
  const index1 = string.indexOf('\n')
  const index2 = string.indexOf('\n', index1 + 1)
  return string.slice(index1 + 1, index2)
}

/**
 * 小数转化为百分数
 * @param {*} point 小数
 * @param {*} fixed 小数点位数
 * @param {*} unit 是否带有%单位，默认false：不带有
 * @returns
 */
export function toPercent(point, fixed = 1, unit = false) {
  let percent = Number(point * 100).toFixed(fixed);
  if (unit) percent += "%";
  return percent;
}

/**
 * 根据符号截取指定的字符串
 * @param {*} name
 * @param {*} sign
 * @param {*} index
 */
export function toFixedFileName(name = '/zqk/zqk01.ipynn', sign = '/') {
  const arrs = name.split(sign);
  const filename = arrs[arrs.length - 1];
  return filename;
}

/**
 * 识别当前何种文件类型
 * @param {*} file 文件名称 xxx.py
 * @param {*} checkFile 校验文件格式
 * @param {*} fileMap 文件集，默认有：'img', 'svg', 'txt', 'py', 'ipynb', 'idpnb'
 * @returns
 */
export function filterType(file, checkFile = '', fileMap = []) {
  let fileFlg = false;
  if (!file) return fileFlg;
  const filelist = ['img', 'svg', 'txt', 'py', 'ipynb', 'idpnb'].concat(fileMap);
  const filetype = file.substring(file.lastIndexOf('.') + 1, file.length);
  // @ts-ignore
  if (Object.prototype.toString.call(checkFile) === '[object String]' & checkFile.length > 0) {
    fileFlg = filelist.includes(checkFile) && filetype === checkFile && filelist.includes(filetype);
  }
  return { fileFlg, filetype }
}

/**
 * 
 */
export function goAccountRouter() {
  window.location.href = '/team/myAccount/personalInformation';
}

/*
 * 判断视频文件
 * @param {*} fileName 文件名称 xxx.xx
 * @param {*} suffixs 文件后缀名数组
 */
function suffixWith(fileName: string, suffixs: Array<string>) {
  const name = fileName.toLowerCase();
  for (const suffix of suffixs) {
    if (name.endsWith(suffix)) {
      return true;
    }
  }
  return false;
}
export function fileType(file: string) {
  const video = ['.avi', '.wmv', '.mpg', '.mpeg', '.mov', '.rm', '.ram', '.swf', '.flv', '.mp4'];
  const image = ['.bmp', '.dib', '.pcp', '.dif', '.wmf', '.jpg', '.jpeg', '.tif', '.eps', '.psd', '.cdr', '.iff', '.tga', '.pcd', '.mpt', '.png', '.webp'];

  if (suffixWith(file, video)) {
    return 'video';
  } else if (suffixWith(file, image)) {
    return 'image';
  } else {
    return 'other';
  }
}
