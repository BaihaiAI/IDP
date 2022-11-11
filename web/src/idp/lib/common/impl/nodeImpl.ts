
import { action, observable } from 'mobx';
import { Node } from '@/idp/lib/common/type/node';

class NodeImpl {
    @observable nodeMap = [];

    @action updateNode(node: Node) {
        this.nodeMap = this.nodeMap.concat([node]);
    }
}

export default new NodeImpl();