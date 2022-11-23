export function gerModulePermissionList(moduleName) {
  const permissionList = JSON.parse(
    window.localStorage.getItem('permission_list')
    || '[]'
  )
  const findResult = permissionList.find(item=>item.module === moduleName)

  return findResult? findResult.operationList :[]
}
