import { noteApiPath2, manageApiPath } from './httpClient';
import request from "./request";

function getOperatorList({keyword}){
  let url = `${manageApiPath}/admin-rs/code_snippets/list?q=${keyword}`
  return request.get(url)
}

function getOperatorCode({key}){
  let url = `${manageApiPath}/admin-rs/code_snippets/detail?key=${key}`
  return request.get(url)
}

function getSearchOperator({keyword}){
  let url = `${manageApiPath}/admin-rs/code_snippets/search?q=${keyword}`
  return request.get(url)
}

const operatorApi = {
  getOperatorList,
  getOperatorCode,
  getSearchOperator
}

export default operatorApi;