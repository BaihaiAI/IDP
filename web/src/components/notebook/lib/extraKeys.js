import * as CodeMirror from 'codemirror';
import { LspWebsocket } from './LspWebsocket';
import { store } from '../../../store';
import PubSub from 'pubsub-js'

const cmds = CodeMirror.commands;
const Pos = CodeMirror.Pos;
let lsp = new LspWebsocket({});

cmds.gotoDefine = function (cm) {
    console.log(cm);
    if (cm.options.mode !== 'python') return;
    let target = getTarget(cm);
    if (!target) return;
    console.log(target);
    lsp.gotoDefineRequest(cm.options.path, cm.options.cellId, target.from.line, target.from.ch);
    goto(cm, 10);
};

function goto(cm, count) {
  /*{
    "jsonrpc": "2.0",
    "id": 6,
    "result": [
        {
            "uri": "file:///home/liuzhe/kk.py",
            "range": {
                "start": {
                    "line": 0,
                    "character": 0
                },
                "end": {
                    "line": 0,
                    "character": 1
                }
            }
        }
    ]
  }*/
  
  const result = lsp.gotoDefine.result;
  if (result) {
    const currentCellId = cm.options.cellId;
    if (lsp.gotoDefine.cellId !== currentCellId) return;
    if (result.length > 0) {
      const item = result[0];     // 只跳转到第一个定义
      const start = item.range.start;
      const end = item.range.end;
      const from = { line: start.line, ch: start.character };
      const to = { line: end.line, ch: end.character };
      if (currentCellId === item.cellId) {
        // cm.setSelection(from, to);
        cm.setCursor(from);
      } else {
        let found = false
        const notebookList = store.getState().notebook.notebookList;
        for (const notebook of notebookList) {
          if (notebook.path === cm.options.path) {
            if (item.cellId in notebook.cellProps) {
              const instance = notebook.cellProps[item.cellId].instance;
              if (instance) {
                instance.focus()
                instance.setCursor(from);
              }
              found = true;
            }
            break;
          }
        }
        if (!found) {
          console.log('not found');
          cm.options.gotoLibrary(item);
        }
      }
    }
  } else if (count > 0) {
    count--;
    setTimeout(() => goto(cm, count), 100);
  }
}

function wordAt(cm, pos) {
    var start = pos.ch, end = start, line = cm.getLine(pos.line);
    while (start && CodeMirror.isWordChar(line.charAt(start - 1))) --start;
    while (end < line.length && CodeMirror.isWordChar(line.charAt(end))) ++end;
    return {from: Pos(pos.line, start), to: Pos(pos.line, end), word: line.slice(start, end)};
}

function getTarget(cm) {
    var from = cm.getCursor("from"), to = cm.getCursor("to");
    if (CodeMirror.cmpPos(from, to) == 0) {
      var word = wordAt(cm, from);
      if (!word.word) return;
      from = word.from;
      to = word.to;
    }
    return {from: from, to: to, query: cm.getRange(from, to), word: word};
}

function findAndGoTo(cm, forward) {
    var target = getTarget(cm);
    if (!target) return;
    var query = target.query;
    var cur = cm.getSearchCursor(query, forward ? target.to : target.from);

    if (forward ? cur.findNext() : cur.findPrevious()) {
      cm.setCursor(cur.from());
      // cm.setSelection(cur.from(), cur.to());
    } else {
      cur = cm.getSearchCursor(query, forward ? Pos(cm.firstLine(), 0)
                                              : cm.clipPos(Pos(cm.lastLine())));
      if (forward ? cur.findNext() : cur.findPrevious()) {
        cm.setCursor(cur.from());
        // cm.setSelection(cur.from(), cur.to());
      }
      else if (target.word) {
        cm.setCursor(cur.from());
        // cm.setSelection(target.from, target.to);
      }
    }
};

cmds.runCell = function (cm) {
  cm.options.runCell();
}

cmds.runCellAndGotoNext = function(cm) {
  cm.options.runCellAndGotoNext();
}

cmds.saveVersion = (cm) => {
  cm.options.saveVersion();
}

cmds.codeMirrorBlur = (cm) => {
  cm.display.input.blur()
  const activePath = store.getState().filesTab.activePath;
  PubSub.publish(`noteBookFouce${activePath}`)
}

cmds.currentCellBottomAddNewCell = (cm) => {
  cm.options.currentCellBottomAddNewCell();
}

function replaceWord(cm, word) {
  console.log(cm);
  const cur = cm.getCursor();
  cm.replaceRange(word, cur, cur, '+insert');
  cm.doc.setCursor({ line: cur.line, ch: cur.ch + 1 });
}

cmds.parentheses = function (cm) {
  replaceWord(cm, '()');
};

cmds.tab = function (cm) {
  if (cm.somethingSelected()) {      // 存在文本选择
    cm.indentSelection('add');    // 正向缩进文本
  } else {   // 无文本选择  
      let spaces = Array(cm.getOption("indentUnit") + 1).join(" ");
      cm.replaceSelection(spaces);                 
  }   
}
cmds.reverseIndent = function (cm) {
  if (cm.somethingSelected()) {
    cm.indentSelection('subtract');  // 反向缩进
  } else {
    // cm.indentLine(cm.getCursor().line, "subtract");  // 直接缩进整行
    const cursor = cm.getCursor();
    cm.setCursor({line: cursor.line, ch: cursor.ch - 4});  // 光标回退 indexUnit 字符
  }   
  return ;
}

// 判断是否为Mac
let mac = CodeMirror.keyMap.default === CodeMirror.keyMap.macDefault;
let gotoDefineKey = (mac ? "Cmd" : "Ctrl") + "-B";
let saveKey = (mac ? "Cmd" : "Ctrl") + "-S";
let runCellKey = (mac ? "Cmd" : "Ctrl") + "-Enter";
let nentAddCell = (mac ? "Cmd" : "Ctrl") + "-Alt" + '-Enter'
let extraKeys = {};
extraKeys[gotoDefineKey] = "gotoDefine";
extraKeys[saveKey] = "saveVersion";
extraKeys[runCellKey] = "runCell";
extraKeys["Shift-Enter"] = "runCellAndGotoNext";
extraKeys["Tab"] = "tab";
extraKeys["Esc"] = "codeMirrorBlur";
extraKeys[nentAddCell] = "currentCellBottomAddNewCell";
extraKeys["Shift-Tab"] = "reverseIndent";


export {extraKeys}