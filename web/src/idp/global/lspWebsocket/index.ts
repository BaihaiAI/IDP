import { LspWebsocket } from "../../../components/notebook/lib/LspWebsocket";

function lspWebsocketSafeSend(env) {
    if (!env) return;
    const lspWebsocket = new LspWebsocket();
    lspWebsocket.safeSend(env);
}

export {
    lspWebsocketSafeSend
}