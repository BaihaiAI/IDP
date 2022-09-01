import globalSlice, { handleAvatarUrlThunk, changeOperatorDecision, changeOperatorKey, changeUpdateList } from './globalSlice';
import filesTabSlice, { selectActivePath } from 'idpStore/features/filesTabSlice';
import notebookSlice, { InsertCodeSnippet } from 'idpStore/features/notebookSlice'

export {
    handleAvatarUrlThunk,
    changeOperatorDecision,
    changeOperatorKey,
    changeUpdateList,
    InsertCodeSnippet,
    selectActivePath,
    globalSlice,
    filesTabSlice,
    notebookSlice
}