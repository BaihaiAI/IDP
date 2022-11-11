import { IdpMenus } from "@/idp/lib/menu";
import IdpMenu from '@/idp/component/header/idp';

IdpMenus.registerIdpMenu('idps', {
    menuType: 'Menu',
    content: <IdpMenu />,
});
