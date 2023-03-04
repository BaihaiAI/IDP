import contentApi from '../../../services/contentApi';

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
import terminal from '@/idp/lib/terminal';
import { useDispatch } from 'react-redux';
import { setFileContent } from '../../../store/features/filesTabSlice';

export const TextEditor = (props) => {
  const { path, content, suffix, posLine, workSpaceHeight } = props;
  const dispatch = useDispatch();
  const meta = path && codemirror.findModeByFileName(path);
  console.log(meta);
  let mode = '';
  let fileType = suffix;
  if (meta) {
    mode = meta.mode;
    fileType = meta.name;
  }

  const editorInputRead = (editor, change) => {
    if (change.text.toString() !== ' ') {
      editor.showHint();
    }
  }

  // 保存文件
  const saveFile = (editor) => {
    const value = editor.getValue();
    dispatch(setFileContent({ path, value }));
    const params = {
      content: value,
      path: path,
      type: fileType,
    };
    contentApi.save(params)
      .then(function (response) {
      })
      .catch(function (err) {
        console.log(err);
      })
  }
  let saveTimer = null;
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

  return (
    <div className="main-sql-wrapper" style={{ height: document.clientHeight - 30, overflow: 'scroll' }}>
      <CodeMirror
        key={path}
        height={workSpaceHeight ? (workSpaceHeight) : (terminal.workspaceHeight - 93)}
        className="text-editor"
        value={content}
        onInputRead={editorInputRead}
        onFocus={editorFocus}
        onBlur={editorBlur}
        editorDidMount={(editor) => {
          editor.focus();
          if (posLine) {
            editor.setCursor({ line: posLine - 1, ch: 0 });
          }
        }}
        options={{
          lineWrapping: true,
          lineNumbers: true,
          styleActiveLine: true,
          styleActiveSelected: true,
          matchBrackets: true,
          autoCloseBrackets: true,
          theme: 'default',
          keyMap: 'sublime',
          mode: mode,
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
          extraKeys: mode === 'python' ? {
            'Tab': (cm) => {
              const spaces = Array(cm.getOption("indentUnit") + 1).join(" ");
              cm.replaceSelection(spaces);
            }
          } : {},
        }}
      />
    </div>
  );
}
