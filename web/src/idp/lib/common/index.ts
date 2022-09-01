import { toJS } from "mobx";
import NodeImpl from '@/idp/lib/common/impl/nodeImpl';
import { Node } from '@/idp/lib/common/type/node';
/**
 * menu节点key
 */
export namespace Nodes {
    // 匹配节点
    export function someNode(name: string) {
        const nodeMap = toJS(NodeImpl.nodeMap);
        const nodeflg = nodeMap.some(it => it.nodeKey === name);
        if (nodeflg) {
            return false;
        }
        return true;
    }

    // 更新节点node
    export function updateNode(node: Node) {
        NodeImpl.updateNode(node);
    }
}