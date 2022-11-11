import webSocket from '../../../services/websoket';
import { kernelWsSendUrl } from '../../../store/config';

const timeout = 5000;

class kernelWsSend {
  constructor(props = {}) {
    if (!kernelWsSend.instance) {
      let param = {};
      param.socketUrl = kernelWsSendUrl;
      param.timeout = timeout;
      param.socketMessage = props.handleSocketMessage;
      param.handleHeartbeat = props.handleSocketMessage;
      this.ws = new webSocket(param);
      this.sendMessage = this.ws.sendMessage;
      this.ws.connection();
      this.ws.heartbeat();
      kernelWsSend.instance = this;
    }
    return kernelWsSend.instance;
  }


  parseMessage = (message) => {
    let outputs = [];
    const msgType = message.msgType;
    let isOutputs = false;
    switch (msgType) {
      case 'input_request':
        outputs.push(this.parseRequest(message));
        isOutputs = true;
        break;
      case 'stream':
        outputs.push(this.parseStdout(message));
        isOutputs = true;
        break;
      case 'error':
        outputs.push(this.parseError(message));
        isOutputs = true;
        break;
      case 'execute_result':
        outputs.push(this.parseExecuteResult(message));
        isOutputs = true;
        break;
      case 'display_data':
        outputs.push(this.parseExecuteDisplay(message));
        isOutputs = true;
        break;
      case 'execute_reply':
        return { execution_count: Math.floor(message.content.execution_count) };
      case 'duration':
        const duration = message.content.duration;
        const executionTime = `${duration}`;
        return { execution_time: executionTime };
      case 'runtime_error':
        outputs.push({
          ename: 'RuntimeError',
          evalue: message.content.message,
          traceback: [],
        });
        isOutputs = true;
        break;
      case 'comm_msg':
        const data = message.content.data;
        let tmpData = {};
        for (const key in data) {
          if (key === 'state') {
            if ('value' in data[key] && typeof data[key]['value'] === 'string' && data[key]['value'] !== '') {
              tmpData[key] = data[key];
              isOutputs = true;
            }
          }
        }

        outputs.push({
          output_type: message.msgType,
          data: tmpData,
        });
        break;
      default:
        break;
    }
    if (isOutputs) {
      return { outputs };
    } else {
      return {};
    }
  }

  parseRequest = (message) => {
    return {
      output_type: message.msgType,
      prompt: message.content.prompt,
      password: message.content.password,
    };
  }

  parseStdout = (message) => {
    return {
      output_type: message.msgType,
      name: message.content.name,
      text: this.formatText(message.content.text),
    };
  }

  parseError = (message) => {
    return {
      output_type: message.msgType,
      ename: message.content.ename,
      evalue: message.content.evalue,
      traceback: message.content.traceback,
    };
  }

  parseExecuteResult = (message) => {
    const data = message.content.data;
    let tmpData = {};
    for (const key in data) {
      tmpData[key] = this.formatText(data[key]);
    }
    return {
      output_type: message.msgType,
      metadata: message.content.metadata,
      execution_count: Math.floor(message.content['execution_count']),
      data: tmpData,
    };
  }

  parseExecuteDisplay = (message) => {
    const data = message.content.data;
    let tmpData = {};
    for (const key in data) {
      if (key === 'image/png') {
        tmpData[key] = data[key];
      } else if (typeof(data[key]) === 'string') {
        if (data[key].indexOf('\n') !== -1) {
          tmpData[key] = this.formatText(data[key]);
        } else {
          tmpData[key] = data[key];
        }
      }
    }
    return {
      output_type: message.msgType,
      metadata: message.content.metadata,
      data: tmpData,
    };
  }

  formatText = (value) => {
    let text = [];
    const arr = value.split('\n');
    for (let i = 0; i < arr.length - 1; i++) {
      if (arr[i] === '') {
        text[text.length - 1] = text[text.length - 1] + '\n';
      } else {
        text.push(`${arr[i]}\n`);
      }
    }
    if (arr.length > 0 && arr[arr.length - 1] !== '') {
      text.push(arr[arr.length - 1]);
    }
    return text;
  }
}


export { kernelWsSend }
