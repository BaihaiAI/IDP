import { shopApiPath, preventInOpen } from './httpClient';
import request from './request';

export const resourceInfo = () => {
  return preventInOpen(() => request.get(`${shopApiPath}/resource/info`));
}