import { noteApiPath2 } from './httpClient';
import { projectId, teamId, userId } from '../store/cookie';
import httpClientV2 from './httpClientV2';
import request from "./request"
import axios from 'axios';

function cat(options) {
  const url = `${noteApiPath2}/content/cat?path=${encodeURIComponent(options.path)}&projectId=${projectId}`
  return request.get(url)
}

function cat2(options){
  const url = `${noteApiPath2}/content/cat?path=${encodeURIComponent(options.path)}&projectId=${projectId}&teamId=${teamId}&userId=${userId}`
  return axios.get(url)
}

function cat2_example({version}){
  console.log("----------")
  const url = `${noteApiPath2}/workspace/file/example`
  return request.post(url, {
    projectId: new Number(projectId),
    teamId,
    version
  })
}

function fullPathCat(options) {
  const url = `${noteApiPath2}/content/fullPathCat?path=${encodeURIComponent(options.path)}&projectId=${projectId}`;
  return request.get(url);
}

function save(options) {
  const url = `${noteApiPath2}/content/save`;
  const data = {
    projectId: Number(projectId),
    content: options.content,
    path: options.path,
    type: options.type,
  }
  return request.post(url, data)
}

function cellAdd(options) {
  const url = `${noteApiPath2}/content/cell`;
  const data = {
    projectId: Number(projectId),
    path: options.path,
    // index: options.index,
    cellType: options.type,
    insertFlag:options.insertFlag,
    aboveCellIndex:options.aboveCellIndex,
    underCellIndex:options.underCellIndex
  };
  return request.post(url,data)
}


function withdrawCell({path, cell}) {
  const url = `${noteApiPath2}/content/cell/add`;
  const data = {
    projectId: Number(projectId),
    path,
    cell,
  }
  return request.post(url, data)
}

function cellUpdate(options) {
  const url = `${noteApiPath2}/content/cell`;
  const data = {
    projectId: Number(projectId),
    path: options.path,
    cells: options.cells,
  };
  return request.put(url, data)
}

function cellUpdateField(path, cells, fields) {
  let nextCells = [];
  for (const cell of cells) {
    let updates = {};
    for (const field of fields) {
      updates[field] = cell[field];
    }
    nextCells.push(
      {
        id: cell.metadata.id,
        updates
      }
    );
  }
  return cellUpdate({
    path: path,
    cells: nextCells,
  });
}

function cellDel(options) {
  const url = `${noteApiPath2}/content/cell?path=${encodeURIComponent(options.path)}&index=${options.index}&id=${options.id}&projectId=${projectId}`;
  return request.delete(url)
}

function cellMove(options) {
  const url = `${noteApiPath2}/content/cell/move`;
  const data = {
    projectId: Number(projectId),
    path: options.path,
    id: options.id,
    neighborCellId: options.neighborCellId,
  };
  return request.put(url, data)
}

function snapshot(options) {
  const url = `${noteApiPath2}/snapshot`;
  const data = {
    projectId: projectId,
    path: options.path,
    label: options.label,
  };
  return request.post(url, data);
}

function snapshotList(options) {
  const url = `${noteApiPath2}/snapshot/list`
  const data = {
    // ${options.path}
    // /store/idp-note/projects/102/notebooks/1.ipynb
    path: [`${options.path}`],
    projectId
  }
  return request.post(url, data);
}

function snapshotRestore(options) {
  const url = `${noteApiPath2}/snapshot/restore`;
  const data = {
    id: options.id,
    path: options.path,
    projectId
  }
  return request.post(url, data)
}

function ipynbPreview(options) {
  const url = `${noteApiPath2}/content/ipynbPreview?path=${encodeURIComponent(options.path)}&projectId=${projectId}`;
  return request.get(url);
}

function taskResult(options) {
  const { path, jobId, jobInstanceId, taskId } = options;
  const qs = `path=${encodeURIComponent(path)}&jobId=${jobId}&jobInstanceId=${jobInstanceId}&taskInstanceId=${taskId}&projectId=${projectId}`;
  const url = `${noteApiPath2}/pipeline/taskResult?${qs}`;
  return request.get(url);
}

function loadShared(options) {
  const qs = `shareId=${options.shareId}&projectId=${projectId}`;
  const url = `${noteApiPath2}/content/loadShared?${qs}`;
  return request.get(url);
}

function viewComparison({leftId, rightId, path}){
  const url = `${noteApiPath2}/snapshot/diff`
  const data = {
    id1: leftId.toString(),
    id2: rightId.toString(),
    path,
    projectId,
  }
  return request.post(url, data)
}

const contentApi = {
  cat2,
  cat2_example,
  cat,
  fullPathCat,
  save,
  cellAdd,
  cellUpdateField,
  cellDel,
  cellMove,
  snapshot,
  snapshotList,
  snapshotRestore,
  ipynbPreview,
  taskResult,
  loadShared,
  withdrawCell,
  viewComparison
}

export default contentApi;
