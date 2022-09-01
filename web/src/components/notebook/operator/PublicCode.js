
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
import 'codemirror/addon/search/match-highlighter';
import 'codemirror/mode/meta';
import 'codemirror/mode/python/python';
import 'codemirror/mode/htmlmixed/htmlmixed';
import 'codemirror/addon/selection/active-line';
import './publiccode.less'
function PublicCode(props){
  const { codeFragment, lineBreak, bindkey } = props;
  return (
    <div className="public-code">
      <CodeMirror
        key={bindkey}
        className="code-editor"
        value={codeFragment}
        options={{
          lineWrapping: lineBreak? true : false,
          lineNumbers: true, // 显示行号
          styleActiveLine: true, // 显示选中行的样式
          styleActiveSelected: true,
          matchBrackets: true, //括号匹配
          keyMap: 'sublime',
          mode: 'python',
          autofocus: true, // 自动获取焦点
          foldGutter: true, // 启用行槽中的代码折叠
          gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"], //在行槽中添加行号显示器、折叠器、语法检测器
          readOnly: true, // 只读
          indentUnit: 4,  // 缩进的空格数
          scrollbarStyle: null,//隐藏滚动条样式
          theme:"juejin"
        }}
      />
    </div>
  )
}

export default PublicCode;