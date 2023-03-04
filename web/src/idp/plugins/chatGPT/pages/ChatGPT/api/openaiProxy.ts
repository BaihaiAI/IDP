import axios from 'axios';
import { terminalPath } from '@/services/httpClient';

const generateAnswer = ({ prompt, onEvent }) => {
  const path = `${terminalPath}/chatGPT/sendMsg`;
  const body = {
    text: prompt
  };
  return axios.post(path, body, {
    onDownloadProgress: onEvent
  });
}

const openaiProxy = { generateAnswer };
export default openaiProxy;