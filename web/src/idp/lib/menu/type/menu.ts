
import { Tool } from '@/idp/lib/tool/type/tool';
import { Node } from '@/idp/lib/common/type/node';
import Events from '@/idp/lib/common/type/events';

export type Menu = {
    content: string | Element | React.ReactNode,
    nodeKey?: Node, // 挂载节点，唯一
    menuType: 'Menu' | 'Tool',
    render?: Element | React.ReactNode, // 渲染render
    children?: Tool,
    onClick?: (info: Events) => void;
}
