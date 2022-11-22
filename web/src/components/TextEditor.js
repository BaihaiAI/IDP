import React, {useContext} from 'react'
import { connect } from "react-redux"
import contentApi from '../services/contentApi';
import intl from 'react-intl-universal';

import {UnControlled as CodeMirror} from 'react-codemirror2'
import 'codemirror/lib/codemirror';
import 'codemirror/keymap/sublime';
import 'codemirror/addon/hint/show-hint';
import 'codemirror/addon/hint/javascript-hint';
import 'codemirror/addon/fold/foldgutter';
import 'codemirror/addon/fold/foldcode';
import 'codemirror/addon/fold/brace-fold';
import 'codemirror/addon/fold/xml-fold';
import 'codemirror/addon/fold/indent-fold';
import 'codemirror/addon/fold/markdown-fold';
import 'codemirror/addon/fold/comment-fold';
import 'codemirror/addon/hint/sql-hint';
// import 'codemirror/addon/dialog/dialog';
// import 'codemirror/addon/search/searchcursor';
// import 'codemirror/addon/search/search';
// import 'codemirror/addon/scroll/annotatescrollbar';
import 'codemirror/addon/search/match-highlighter';
// import 'codemirror/addon/search/jump-to-line';
import 'codemirror/mode/meta';
import 'codemirror/mode/javascript/javascript';
import 'codemirror/mode/clike/clike';
import 'codemirror/mode/sql/sql';
import 'codemirror/mode/rust/rust';
import 'codemirror/mode/python/python';
import 'codemirror/mode/htmlmixed/htmlmixed';
import 'codemirror/mode/shell/shell'
import 'codemirror/addon/selection/active-line';
import * as codemirror from 'codemirror';
import appContext from "../context"
import {observer} from "mobx-react"
import globalData from "idp/global"
import terminal from '@/idp/lib/terminal';

class TextEditor extends React.Component {
  constructor(props) {
    super(props);
    this.path = this.props.path;
    this.suffix = this.props.suffix;
    this.posLine = this.props.posLine;
    const meta = this.path && codemirror.findModeByFileName(this.path);
    console.log(meta);
    let mode = '';
    let fileType = this.suffix;
    if (meta) {
      mode = meta.mode;
      fileType = meta.name;
    }
    this.state = {
      value: '',
      mode,
      fileType,
      mime: '',
    }
    this.saveTimer = null;  // 当focus时，开启定时保存文件
    this.editor = null; // codemiirror editor
    this.extraKeys = mode === 'python' ? {
      'Tab': (cm) => {
        const spaces = Array(cm.getOption("indentUnit") + 1).join(" ");
        cm.replaceSelection(spaces);
      }
    } : {}
  }

  setStateValue = (value) => {
    const meta = codemirror.findModeByFileName(this.path);
    console.log(meta);
    let mode = '';
    let fileType = this.suffix;
    if (meta) {
      mode = meta.mode;
      fileType = meta.name;
    }

    this.setState({
      value,
      mode,
      fileType,
    });
  }

  getFile = () => {
    const _this = this;    //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
    contentApi.cat({ path: _this.path })
      .then(function (response) {
        // console.log(response.data.data);
        const { content,contentType,mime } = response.data
        let value = content;
        // 以备日后用到
        if ('application/octet-stream' === mime) {
          value = intl.get('TEXT_EDITOR_OPEN_INFO');
        }
        _this.setState({
          value, mime
        }, () => {
          if (_this.posLine && _this.editor) {
            _this.editor.focus()
            _this.editor.setCursor({ line: _this.posLine - 1, ch: 0 });
          }
        });
        // _this.setStateValue(response.content);
      })
      .catch(function (error) {
        console.log(error);
        _this.props.notebookTabRef.current.removeTab(_this.path)
      })
  }

  renderFile = ()=>{
    const { content, mime } = this.props.item
    let value = content;
    // 以备日后用到
    if ('application/octet-stream' === mime) {
      value = intl.get('TEXT_EDITOR_OPEN_INFO');
    }
    this.setState({
      value,
      mime
    }, () => {
      if (this.posLine && this.editor) {
        this.editor.focus()
        this.editor.setCursor({ line: this.posLine - 1, ch: 0 });
      }
    });
  }

  componentWillMount() {
    this.renderFile()
  }

  componentDidMount() {
  }

  componentWillReceiveProps(nextProps) {
    // this.path = nextProps.path;
    // this.getFile(); // 从服务端读取文件
  }

  componentWillUnmount() {
    console.info('componentWillUnmount start......');

    this.saveTimer && clearInterval(this.saveTimer);
    if (!this.props.deleteflag){
      this.saveFile();
    }else{
      console.info(this.props.path + ' has been deleted , it need not to be save again.');
    }
    console.info('componentWillUnmount   end......');
  }

  saveFile = () => {
    console.log(this.state.mime, this.state.mime.startsWith('text'))
    console.log(this.state.mime.indexOf('json') === -1)
    // if (this.state.value === '') return;
    if (!this.state.mime.startsWith('text') && this.state.mime.indexOf('json') === -1) return;
    const _this = this;
    const params = {
      content: _this.state.value,
      path: _this.props.path,
      type: _this.state.fileType,
    };
    contentApi.save(params)
    .then(function (response) {
    })
    .catch(function (err) {
      console.log(err);
    })
  }

  editorFocus = () => {
    if (!this.saveTimer) {
      this.saveTimer = setInterval(() => {
        this.saveFile();
      }, 5000);
    }
  }

  editorBlur = () => {
    this.saveFile();
    this.saveTimer && clearInterval(this.saveTimer);
    this.saveTimer = null;
  }

  editorInputRead = (instance, change) => {
    if (change.text.toString() !== ' ') {
      instance.showHint();
    }
  }

  render() {

    return (
      <div className="main-sql-wrapper" style={{ height: document.clientHeight - 30, overflow: 'scroll' }}>
        {
          this.state.mime.startsWith('image') ? 
            <img style={{ objectFit: 'scale-down', height: this.props.workSpaceHeight ? (this.props.workSpaceHeight) : (terminal.workspaceHeight - 33) }} src={`data:${this.state.mime};base64,${this.state.value}`} /> :
            <CodeMirror
              key={this.props.path}
              height={this.props.workSpaceHeight ? (this.props.workSpaceHeight) : (terminal.workspaceHeight - 93)}
              className="text-editor"
              value={this.state.value}
              editorDidMount={(editor) => {
                this.editor = editor
              }}
              onInputRead={this.editorInputRead}
              onChange={(editor, data, value) => { this.state.value = value }}
              onFocus={this.editorFocus}
              onBlur={this.editorBlur}
              options={{
                lineWrapping: true,
                lineNumbers: true,
                styleActiveLine: true,
                styleActiveSelected: true,
                matchBrackets: true,
                autoCloseBrackets: true,
                theme: 'default',
                keyMap: 'sublime',
                mode: this.state.mode,
                autofocus: true,
                foldGutter: true,
                gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
                highlightSelectionMatches: {
                  // showToken: /\w/,
                  // annotateScrollbar: true,
                },
                hintOptions: {
                  completeSingle: false,
                  alignWithWord: true,
                },
                indentUnit: 4,  // 缩进的空格数
                extraKeys: this.extraKeys,
              }}
            />
        }
        </div>
    );
  }

}


function TextEditorWithAppContext(props){
  const { notebookTabRef } = globalData.appComponentData
  return <TextEditor {...props} notebookTabRef={notebookTabRef} />
}
export default observer(TextEditorWithAppContext);
