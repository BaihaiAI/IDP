import AccountSetting from "@components/accountSetting";
import { Drawer } from "antd";
import './index.less';

export default function UserCenter({ setRightDrawer, rightDrawer, avatarUrl, updateAvatarUrl }) {
    return (
        <Drawer
            title={`个人账户`}
            placement="right"
            width="540"
            onClose={() => { setRightDrawer(false) }}
            visible={rightDrawer}
            keyboard={true}
            className='userinfo-modal'
        >

            <AccountSetting avatarUrl={avatarUrl} updateAvatarUrl={updateAvatarUrl} />
        </Drawer>
    )
}
