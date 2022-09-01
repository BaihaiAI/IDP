import { noteApiPath2} from './httpClient'
import request from "./request"

function health() {
  const url = `${noteApiPath2}/state/health`;
  return request.get(url);
}

const probeApi = { health };
export default probeApi;
