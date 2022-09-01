import _extends from "@babel/runtime/helpers/extends";
import 'codemirror/mode/markdown/markdown';
import React, { Component } from 'react';
import "./codemirror.css";
import "./index.css";
import { jsx as _jsx } from "react/jsx-runtime";
var SERVER_RENDERED = typeof navigator === 'undefined' || global.PREVENT_CODEMIRROR_RENDER === true;
var cm;

if (!SERVER_RENDERED) {
  // tslint:disable-next-line: no-var-requires
  cm = require('codemirror');
}

export default class ReactCodeMirror extends Component {
  // public editor!: Doc | Editor | EditorFromTextArea | Editor;
  constructor(props) {
    super(props);
    this.textarea = void 0;
    this.editor = void 0;

    if (SERVER_RENDERED) {
      return;
    }

    if (this.props.editorWillMount) {
      this.props.editorWillMount();
    }
  }

  render() {
    return /*#__PURE__*/_jsx("textarea", {
      ref: instance => this.textarea = instance
    });
  }

  componentDidMount() {
    if (SERVER_RENDERED) {
      return;
    }

    var {
      options
    } = this.props;

    if (this.props.defineMode) {
      if (this.props.defineMode.name && this.props.defineMode.fn) {
        cm.defineMode(this.props.defineMode.name, this.props.defineMode.fn);
      }
    }

    var editorOption = _extends({
      tabSize: 2,
      lineNumbers: true
    }, options, {
      mode: 'markdown'
    }); // 生成codemirror实例


    this.editor = cm.fromTextArea(this.textarea, editorOption);
    this.renderCodeMirror(this.props);
  }

  componentDidUpdate(prevProps) {
    var {
      value,
      width,
      height
    } = this.props;

    if (this.editor.getValue() !== value && value !== prevProps.value) {
      this.editor.setValue(value || '');
    }

    if (width !== prevProps.width || height !== prevProps.height) {
      // Setting Size
      this.editor.setSize(width, height);
    }
  }

  shouldComponentUpdate(nextProps, nextState) {
    return nextProps.value !== this.props.value || nextProps.options !== this.props.options || nextProps.height !== this.props.height || nextProps.width !== this.props.width;
  } // 将 props 中所有的事件处理函数映射并保存


  getEventHandleFromProps() {
    var propNames = Object.keys(this.props);
    var eventHandle = propNames.filter(prop => {
      return /^on+/.test(prop);
    });
    var eventDict = {};
    eventHandle.forEach(ele => {
      eventDict[ele] = ele.replace(/^on[A-Z]/g, s => s.slice(2).toLowerCase());
    });
    return eventDict;
  }

  renderCodeMirror(props) {
    var {
      value,
      width,
      height
    } = props; // 获取CodeMirror用于获取其中的一些常量
    // 事件处理映射

    var eventDict = this.getEventHandleFromProps();
    Object.keys(eventDict).forEach(event => {
      var handle = this.props[event];
      this.editor.on(eventDict[event], handle);
    }); // Init value

    this.editor.setValue(value || ''); // this.editor.setOption(name, editorOption.mode);

    if (width || height) {
      // Setting size
      this.editor.setSize(width, height);
    }
  }

}
ReactCodeMirror.defaultProps = {
  height: '100%',
  options: {
    lineNumbers: true,
    mode: 'markdown',
    tabSize: 2
  },
  value: '',
  width: '100%'
};
//# sourceMappingURL=index.js.map