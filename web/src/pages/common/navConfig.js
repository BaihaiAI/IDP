import Icons from "../../components/Icons/Icons"
import intl from "react-intl-universal"
import Workspace from "@/pages/workspace"

export default [
    {
        key: 'workspace', // key值，和路由保持一致，必填
        name: () => intl.get("WORKSPACE"),
        iconUnChecked: <Icons.BHFolderIcon style={{ fontSize: 30 }} />, // 未选中的icon， 默认false
        iconChecked: false, // 选中时的icon, 默认false
        menuClassName: {
            margin: '0px'
        }, // 默认{}
        flg: true, // 是否显示
        needCache: true,
        component: Workspace
    }
]
