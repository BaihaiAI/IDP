import request from "./request"

const prefix = "/0/api/v1/admin-rs"

function getAuditList({ content, status, categoryId, current, size }) {
    const params = {
        content,
        status,
        categoryId,
        current,
        size,
    }
    return request.get(`${prefix}/audit/list`, {
        params
    })
}

// 通知列表
function notificationList({ viewFlag, size, current }) {
    const url = `${prefix}/message/list`
    return request.get(url, {
        params: {
            viewFlag,
            size,
            current
        }
    })
}

// 改变为已读(read) 或者 删除(delete)
function changeStatusOrDetele({ id, viewFlag }) {
    const url = `${prefix}/message/update`;
    return request.post(url, {
        id,
        viewFlag,
    })
}

function getAuditDetail(id) {
    return request.get(`${prefix}/audit/get-detail`, {
        params: {
            id
        }
    })
}

function updateAudit({
    id,
    status,
    opinion
}) {
    return request.post(`${prefix}/audit/update`, {
        id,
        status,
        opinion
    })
}

const auditApi = {
    getAuditList,
    getAuditDetail,
    updateAudit,
    notificationList,
    changeStatusOrDetele
}

export default auditApi
