import { UnControlled as CodeMirror } from 'react-codemirror2'
import { useContext, useEffect, useState } from "react";
import appContext from "../../../context"
import contentApi from '../../../services/contentApi';
import './pythonLib.less'

interface Props {
  workSpaceHeight: number
  path: string
  posLine: number
  posCh: number
}

export const PythonLibEditor: React.FC<Props> = (props: Props) => {
  const { path, posLine, posCh, workSpaceHeight } = props
  const { notebookTabRef } = useContext(appContext)
  const [content, setContent] = useState('')
  const [editor, setEditor] = useState<any>()

  useEffect(() => {
    contentApi.fullPathCat({ path: path.substring(path.lastIndexOf('///') + 2) })
      .then(function (response) {
        const { content } = response.data
        setContent(content)
      })
      .catch(function (error) {
        notebookTabRef.current.removeTab(path)
      })
  }, [])

  useEffect(() => {
    if (posLine && content && editor) {
      editor.focus()
      editor.setCursor({ line: posLine, ch: posCh })
    }
  }, [content])

  return (
    <div className='control-bar'>
      <div style={{ height: workSpaceHeight ? (workSpaceHeight - 40) : (document.body.clientHeight - 93), overflow: 'scroll' }}>
        <div className="python-code">
          <CodeMirror
            key={path}
            className="python-code-mirror-max"
            value={content}
            editorDidMount={(editor) => {
              setEditor(editor)
            }}
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
              readOnly: true,
              highlightSelectionMatches: {
                // showToken: /\w/,
                // annotateScrollbar: true,
              },
              indentUnit: 4,  // 缩进的空格数
            }}
          />
        </div>
      </div>
    </div>
  )
}
