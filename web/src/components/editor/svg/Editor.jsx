import { useState, useEffect, useRef } from 'react'
import contentApi from '../../../services/contentApi';

import { UnControlled as CodeMirror } from 'react-codemirror2'
import 'codemirror/lib/codemirror';
import 'codemirror/keymap/sublime';
import 'codemirror/addon/hint/show-hint';
import 'codemirror/addon/fold/foldgutter';
import 'codemirror/addon/fold/foldcode';
import 'codemirror/addon/fold/brace-fold';
import 'codemirror/addon/fold/xml-fold';
import 'codemirror/addon/fold/indent-fold';
import 'codemirror/addon/fold/comment-fold';
import 'codemirror/addon/search/match-highlighter';
import 'codemirror/mode/meta';
import 'codemirror/mode/xml/xml';
import 'codemirror/mode/htmlmixed/htmlmixed';
import { useMemoizedFn } from 'ahooks';

const Editor = (props) => {
  const { path, content, suffix, onChange, deleteflag, posLine, workSpaceHeight } = props;
  const [instance, setInstance] = useState(null);
  const [value, setValue] = useState(content);
  let saveTimer = useRef();  // 当focus时，开启定时保存文件

  const handlerUnMount = useMemoizedFn(() => {
    console.info('useEffectf return function start......');

    saveTimer.current && clearInterval(saveTimer.current);
    console.info(deleteflag, 'delete flag');

    if (!deleteflag) {
      saveFile();
    } else {
      console.info(path + ' has been deleted , it need not to be save again.');
    }
  })

  useEffect(() => {
    return handlerUnMount
  }, [])

  const saveFile = useMemoizedFn(() => {
    if (!instance) return;
    const value = instance.getValue();
    if (value === '') return;
    onChange(value);
    const params = {
      content: value,
      path: path,
      type: suffix,
    };

    contentApi.save(params)
      .then(function (response) {
      })
      .catch(function (err) {
        console.log(err);
      })
  })

  const editorFocus = () => {
    if (!saveTimer.current) {
      saveTimer.current = setInterval(() => {
        saveFile();
      }, 5000);
    }
  }
  const editorBlur = () => {
    saveFile();
    saveTimer.current && clearInterval(saveTimer.current);
    saveTimer.current = null;
  }

  return (
    <div className="main-sql-wrapper" style={{ height: workSpaceHeight ? (workSpaceHeight - 40) : (document.body.clientHeight - 125), overflow: 'scroll' }}>
      <CodeMirror
        key={path}
        height={workSpaceHeight ? (workSpaceHeight) : (document.body.clientHeight - 90)}
        className="text-editor"
        value={value}
        editorDidMount={(instance) => {
          setInstance(instance);
          if (posLine) {
            instance.focus()
            instance.setCursor(posLine - 1, 0);
          }
        }}
        onFocus={editorFocus}
        onBlur={() => editorBlur()}
        options={{
          lineWrapping: true,
          lineNumbers: true,
          styleActiveLine: true,
          styleActiveSelected: true,
          theme: 'default',
          keyMap: 'sublime',
          mode: 'xml',
          autofocus: true,
          foldGutter: true,
          gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
          highlightSelectionMatches: {
            // showToken: /\w/,
            // annotateScrollbar: true,
          },
        }}
      />
    </div>
  )
}

export default Editor;
