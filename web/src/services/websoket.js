/**
 * 参数：[socketOpen|socketClose|socketMessage|socketError] = func，[socket连接成功时触发|连接关闭|发送消息|连接错误]
 * timeout：连接超时时间
 * @type {module.webSocket}
 */

const heartbeatTimeout = 25000;

class webSocket {
    constructor(param = {}) {
        this.param = param;
        this.reconnectCount = 0;
        this.socket = null;
        this.taskRemindInterval = null;
        this.isSucces = false;
        this.lastReceiveTime = this.timestamp();
        this.heartbeatInterval = 0;
    }
    connection = () => {
        // console.log(`connection time ${this.timestamp()}`);
        this.socket && this.socket.close();
        let { socketUrl, timeout = 5000 } = this.param;
        // 检测当前浏览器是什么浏览器来决定用什么socket
        if ('WebSocket' in window) {
            // console.log('Connect WebSocket');

            this.socket = new WebSocket(socketUrl);
        }
        else if ('MozWebSocket' in window) {
            // console.log('MozWebSocket');

            // this.socket = new MozWebSocket(socketUrl);
        }
        else {
            // console.log('SockJS');

            // this.socket = new SockJS(socketUrl);
        }
        this.socket.onopen = this.onopen;
        this.socket.onmessage = this.onmessage;
        this.socket.onclose = this.onclose;
        this.socket.onerror = this.onerror;
        this.socket.sendMessage = this.sendMessage;
        this.socket.closeSocket = this.closeSocket;
        this.socketMessage = this.param.socketMessage;
        this.socketError = this.param.socketError;
        this.handleHeartbeat = this.param.handleHeartbeat;
        // 检测返回的状态码 如果socket.readyState不等于1则连接失败，关闭连接
        if (timeout) {
            let time = setTimeout(() => {
                if (this.socket && this.socket.readyState !== 1) {
                    // console.log('open websocket failed');
                } else {
                    this.isSucces = true;
                    // console.log('open websocket success');
                }
                clearTimeout(time);
            }, timeout);
        }
    };

    // 连接成功触发
    onopen = () => {
        // console.log('websocket open success...');
        let { socketOpen } = this.param;
        this.isSucces = true  //连接成功将标识符改为false
        this.lastReceiveTime = this.timestamp();
        socketOpen && socketOpen();
        this.handleHeartbeat && this.handleHeartbeat(true);
    };
    // 后端向前端推得数据
    onmessage = (msg) => {
        // 更新心跳时间
        this.lastReceiveTime = this.timestamp();
        if ('pong' !== msg.data) {
            this.socketMessage && this.socketMessage(msg.data);
        }
    };
    // 关闭连接触发
    onclose = (e) => {
        this.isSucces = false;
        // console.log('关闭socket收到的数据:' + e.code);
        // console.log(this.socket);
        let { socketClose } = this.param;
        socketClose && socketClose(e);
        // 根据后端返回的状态码做操作
        if (e.code == '4500') {
            // console.log('websocket close ' + e.code);
        } else {
            // this.reConnect();
        }
    };
    onerror = (e) => {
        // socket连接报错触发
        // console.log('websocket error ' + e.code);
        this.socketError && this.socketError(e);
        if (e.code == '1006') {
            // console.log('websokcet error 1006')
            // this.reConnect();
        } else if (!e.code) {
            // console.log('websokcet error undefined');
        }
    };
    sendMessage = (value) => {
        // 向后端发送数据
        if (!this.socket || this.socket.readyState !== 1) {
            this.connection();
            this.socketError && this.socketError('socket error when send message!');
            return false;
        } else {
            this.socket.send(JSON.stringify(value));
            return true;
        }
    };

    // 获取当前时间戳
    timestamp = () => {
        return new Date().getTime();
    }

    // 心跳检测
    heartbeat = () => {
        setInterval(() => {
            // console.log(`socket state: ${this.socket.readyState}`);
            if (this.socket && this.socket.readyState === 1) {
                this.socket.send("ping");
                const now = this.timestamp();
                this.heartbeatInterval = now - this.lastReceiveTime;
                if (this.heartbeatInterval > heartbeatTimeout) {
                    // console.log({ now, lastReceiveTime: this.lastReceiveTime });
                    this.handleHeartbeat && this.handleHeartbeat(false);
                    this.connection();
                }
            } else {
                this.handleHeartbeat && this.handleHeartbeat(false);
                this.connection();
            }
        }, 4000);
    }
};

export default webSocket;
