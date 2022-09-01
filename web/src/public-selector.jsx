const { useMemo } = require("react");
import { useDispatch, useSelector } from "react-redux";
import { selectActivePath } from 'idpStore/features/filesTabSlice';
import { store } from '@/store';

function useIdpSelector() {
    const _selectActivePath = store.useSelector(selectActivePath);
    return _selectActivePath
}

export default useIdpSelector