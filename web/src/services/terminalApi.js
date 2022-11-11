import { projectId, region, teamId } from '../store/cookie';
import request from "./request";
import { terminalPath } from './httpClient'

function getTerminal(options) {
  let url = `${terminalPath}/pid?rows=${options.rows}&cols=${options.cols}&teamId=${teamId}&projectId=${projectId}`;
  return request.get(url);
}

function openVscode() {
  let url = `${terminalPath}/vscode`;
  return request.get(url);
}

const terminalApi = {
  getTerminal,
  openVscode,
};

export default terminalApi;
