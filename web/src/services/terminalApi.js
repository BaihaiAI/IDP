import { projectId, region, teamId } from '../store/cookie';
import request from "./request";
import { terminalPath, terminalPath2 } from './httpClient'

function getTerminal(options) {
  const { rows, cols, env } = options;
  let url = `${terminalPath2}/${projectId}/pid?rows=${rows}&cols=${cols}&teamId=${teamId}&projectId=${projectId}&env=${env}`;
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
