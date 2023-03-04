import { noteApiPath2 } from './httpClient';
import { teamId, userId, projectId } from '@/store/cookie';

export const contentLoad = ({ path }) => {
  return `${noteApiPath2}/content/load?path=${path}&projectId=${projectId}&userId=${userId}&teamId=${teamId}`
}