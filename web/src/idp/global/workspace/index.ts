import { action, observable } from "mobx";

const w: any = window;

class WorkSpace {
    constructor() { }

    @action updateWorkSpaceRef(workSpaceRef: any) {
        w.workSpaceRef = workSpaceRef;
    }

    @action exeWorkSpaceRef(parms1, parms2) {
        w.workSpaceRef.onSelect(parms1, parms2)
    }

    @action getWorkSpace(callback) {
        callback(w.workSpaceRef);
    }
}

export default new WorkSpace();