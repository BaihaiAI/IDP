import React, { Component } from 'react';
import { Modal, Button } from 'antd';
import workerScript from './worker';

class Idpldle extends Component {

  constructor(props) {
    super(props);
    this.outTime = 0.5 * 3600; // 1h
    this.time = null;
    this.myWorker = null;
  }

  state = {
    idleCount: 0, // 计时器
    refresh: false, // 刷新
    winX: 0, // 鼠标y
    winY: 0, // 鼠标x
    msg: false, // modal
  }

  /**
   * 定时任务
   */
  subTimeIdle = () => {
    const that = this;
    // this.myWorker && this.myWorker.terminate();
    this.myWorker && this.myWorker.postMessage(false);
    this.myWorker = new Worker(workerScript);
    this.myWorker.onmessage = (m) => {
      m.data && that.setState({ refresh: true, idleCount: 0, msg: true }, () => {
        that.myWorker.postMessage(false);
        // that.myWorker.terminate();
      });
    };
    this.myWorker.postMessage(true); // true 激活
  }

  /**
   * 鼠标移入时，触发时间戳方法，同时监听鼠标是否移动，若是移动立即销毁time，并且重新计时
   */
  onmousemove = () => {
    const that = this;
    document.onmousemove = function (event) {
      let x1 = event.clientX;
      let y1 = event.clientY;
      if (x1 !== that.state.winX || y1 !== that.state.winY) {
        that.myWorker.postMessage(false);
        that.setState({ refresh: false, idleCount: 0 });
      }
      that.setState({ winX: x1, winY: y1 });
      if (!that.state.msg) {
        that.myWorker.postMessage(false);
        that.setState({ refresh: false, idleCount: 0 });
      }
    };
  }

  /**
   * 鼠标移出浏览器立即触发时间戳方法
   */
  onmouseout = () => {
    const that = this;
    document.onmouseout = function (event) {
      if (!that.state.msg) {
        that.myWorker.postMessage(false);
        that.setState({ refresh: false, idleCount: 0 });
      }
    }
  }

  /**
   * 销毁上次time，并且更新state值，重新触发新的时间戳方法
   */
  restTimeIdle = () => {
    const that = this;
    new Promise((resolve, reject) => {
      // clearInterval(this.time);
      // that.myWorker.terminate();
      that.myWorker.postMessage(false); // 重新激活
      that.setState({ refresh: false, idleCount: 0 });
      resolve(true);
    }).then(() => {
      that.subTimeIdle();
    })
  }

  /**
   * 监听键盘事件
   */
  onkeydown = () => {
    const that = this;
    document.onkeydown = function (event) {
      // clearInterval(this.time);
      // that.myWorker.terminate();
      that.myWorker.postMessage(false); // 重新激活
      that.setState({ refresh: false, idleCount: 0 });
    };
  }

  handleOk = () => {
    // clearInterval(this.time);
    // this.myWorker.terminate();
    this.myWorker.postMessage(false); // 重新激活
    this.setState({ refresh: false, idleCount: 0, msg: false }, () => {
      window.location.reload();
    });
  }

  componentDidMount() {
    this.subTimeIdle();
    this.onmousemove();
    this.onmouseout();
    this.onkeydown();
  }

  componentWillUnmount() {
    // clearInterval(this.time);
    this.myWorker.terminate();
  }

  render() {
    const { msg } = this.state;

    return (
      <>
        <Modal
          footer={null}
          visible={msg}
          closable={false}
          keyboard={false}
          maskClosable={false}
          width={400}
          onOk={() => this.handleOk()}
          bodyStyle={{ padding: '0px' }}
        >
          <div style={{ background: '#fff', borderRadius: '8px', padding: '24px 24px 10px' }}>
            <div style={{ display: 'flex', justifyContent: 'center', marginBottom: '15px', fontSize: '13px' }}>您已长时间未操作，服务器已断开，请点击刷新重新连接？</div>
            <div style={{ textAlign: 'end' }}><Button size='small' type='primary' style={{ fontSize: '13px', borderRadius: '8px' }} onClick={() => this.handleOk()}>刷新</Button></div>
          </div>
        </Modal>
      </>
    );
  }
}

export default Idpldle;
