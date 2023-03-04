import { UnControlled as CodeMirror } from 'react-codemirror2'
import * as codeMirror from 'codemirror';
import { useContext, useEffect, useState } from "react";
import axios from 'axios';
import appContext from "../../../context"
import contentApi from '../../../services/contentApi';
import { TopToolBar } from "./TopToolBar";
import { Output } from './Output';
import { region, teamId, projectId } from '../../../store/cookie';
import { terminalWsUrl, getCurrentEnv, setCurrentEnv } from '../../../store/config';
import "./python.less";
import { message, Modal } from 'antd';
import { selectFileList, addFile, removeFile, addFileOutput } from '../../../store/features/pythonSlice';
import { useDispatch, useSelector } from 'react-redux';
import globalData from "idp/global"
import terminalApi from '../../../services/terminalApi'
import environmentAPI from '../../../services/environment'
import { observer } from 'mobx-react';
import resourceControl from '../../../idp/global/resourceControl';
import { setFileContent } from '../../../store/features/filesTabSlice';

interface Props {
  workSpaceHeight: number
  path: string
  posLine: number
  item:{
    [p in string]:any
  }
}

export const PythonEditor: React.FC<Props> = observer((props: Props) => {
  const { path, posLine, workSpaceHeight,item } = props
  const [content, setContent] = useState(item.content)
  const dispatch = useDispatch()
  const FileList = useSelector(selectFileList)
  const [status, setStatus] = useState('ready')

  const saveFile = (editor) => {
    const value = editor.getValue()
    const params = {
      content: value,
      path,
      type: 'python',
    };
    dispatch(setFileContent({path, value}));
    contentApi.save(params)
      .then(function (response) {
      })
      .catch(function (err) {
        console.log(err);
      })
  }

  let saveTimer = null
  const editorFocus = (editor) => {
    if (!saveTimer) {
      saveTimer = setInterval(() => {
        saveFile(editor);
      }, 5000);
    }
  }
  const editorBlur = (editor) => {
    saveFile(editor);
    saveTimer && clearInterval(saveTimer);
    saveTimer = null;
  }

  const editorInputRead = (instance, change) => {
    if (change.origin === '+input' && change.text.toString() !== ' ') {
      instance.showHint();
    }
  }

  const [ws, setWs] = useState(null);
  const doRun = (parameters: string) => {
    if (resourceControl.machineStatus !== 'running') {
      Modal.warning({
        title: '未申请运行资源，请先在工具栏中配置CPU/GPU/内存后申请资源'
      });
      return;
    }
    if (!ws || ws.readyState !== 1) {
      ws && ws.close();
      asyncInitSysEnv()
    }
    dispatch(addFile({ path }))
    const dir = path.substring(0, path.lastIndexOf('/'))
    const fileName = path.substring(path.lastIndexOf('/') + 1)
    ws.send(`pushd .${dir};python3 ${fileName}${parameters};popd\n`)
    // ws.send(JSON.stringify({
    //   teamId,
    //   projectId,
    //   path,
    // }))
    setStatus('executing')
    setShowOutput(true)
    document.getElementById('python-output').scrollIntoView(true)
  }
  const doStop = () => {
    ws.send('\x03')
    ws.send('popd\n');
    setStatus('ready')
  }
  const initSysEnv = async (currentEnv) => {
    return await terminalApi.getTerminal({ env: currentEnv }).then((res: any) => {
      if (res.code === 20000000) {
        return res.data.pid
      } else {
        console.log(res.message);
        message.error(res.message);
        return null
      }
    }).catch(() => {
      message.error('运行机器未启动，请先启动运行机器');
      resourceControl.getRuntimeStatus(null);
      return null
    });
  }
  const invalidOutput = (data: string) => {
    return data.indexOf('python ') > -1 || data.indexOf('source activate ') > -1
      || data.indexOf('clear') > -1 || data.indexOf('cd notebooks') > -1 || data.indexOf('[H[2J') > -1
      || data.indexOf('gitconfig') > -1 || data.indexOf('source /root/') > -1
      || data.indexOf('idp-raycluster') > -1 || data.indexOf('root@') > -1 
      || (data.indexOf('notebooks') > -1 && data.indexOf('~') > -1)
      || data.indexOf('pushd') > -1 || data.indexOf('popd') > -1
  }
  const isComplete = (data: String) => {
    return data === 'complete' || data.indexOf('bash-') !== -1 || data.indexOf('idp-raycluster') !== -1
      || (data.startsWith('~') && data.indexOf('notebooks') !== -1) || data.indexOf('idp-develop') !== -1 
      || data.indexOf('idp-kernel-') !== -1
  }
  async function asyncInitSysEnv() {
    let currentEnv = getCurrentEnv();
    await environmentAPI.getEnvironmentName()
      .then(res => {
        const data = res.data
        setCurrentEnv(data)
        currentEnv = data
      })
      .catch(err => {
        console.log(err)
      })
    let ws = null;
    const pid = await initSysEnv(currentEnv);
    if (!pid) return;
    ws = new WebSocket(terminalWsUrl + pid);
    // ws = new WebSocket(pythonWsUrl)
    setWs(ws);
    ws.onopen = () => {
      if (ws.readyState === 1 && currentEnv) {
        // ws.send(`source /root/.bash_profile\n`)
        // ws.send(`source activate ${currentEnv} \n`);
      }
    }
    ws.onmessage = (e: any) => {
      if (isComplete(e.data)) {
        setStatus('ready')
      } else if (!invalidOutput(e.data)) {
        dispatch(addFileOutput({ path, output: e.data }))
      }
    }
  }

  useEffect(() => {
    dispatch(addFile({ path }))
    return () => {
      dispatch(removeFile({ path }))
    };
  }, [])

  useEffect(() => { 
    if (!ws && resourceControl.machineStatus === 'running') {
      asyncInitSysEnv()
    }

    return () => {
      ws && ws.close();
    };
  }, [resourceControl.machineStatus, ws]);

  const setExtraKeys = () => {
    const mac = codeMirror.keyMap.default === codeMirror.keyMap.macDefault;
    let extraKeys = {
      'Tab': (cm) => {
        const spaces = Array(cm.getOption("indentUnit") + 1).join(" ");
        cm.replaceSelection(spaces);
    }}
    const saveKey = mac ? 'Cmd-S' : 'Ctrl-S';
    extraKeys[saveKey] = saveFile
    return extraKeys;
  }

  const [showOutput, setShowOutput] = useState(false)

  return (
    <div className='control-bar'>
      <div className='control'>
        <TopToolBar doRun={doRun} doStop={doStop} status={status} />
      </div>
      <div style={{ height: document.body.clientHeight - 121, overflow: 'scroll' }}>
        <div className="python-code">
          <CodeMirror
            key={path}
            className={showOutput ? 'python-code-mirror' : 'python-code-mirror-max'}
            value={content}
            editorDidMount={(editor) => {
              if (posLine) {
                editor.focus()
                editor.setCursor({ line: posLine - 1, ch: 0 })
              }
            }}
            onInputRead={editorInputRead}
            onFocus={editorFocus}
            onBlur={editorBlur}
            options={{
              lineWrapping: true,
              lineNumbers: true,
              styleActiveLine: true,
              autoCloseBrackets: true,
              theme: 'default',
              keyMap: 'sublime',
              mode: 'python',
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
              extraKeys: setExtraKeys()
            }}
          />
        </div>
        <div id="python-output" style={{ backgroundColor: 'white', display: showOutput ? '' : 'none' }}>
          <Output value={FileList[path] ? FileList[path].output : ''} height="200px" />
        </div>
      </div>
    </div>
  )
})
