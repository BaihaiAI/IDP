import * as CodeMirror from 'codemirror';
import { remove } from 'jszip';

var Mousetrap = require('mousetrap');

const mac = CodeMirror.keyMap.default === CodeMirror.keyMap.macDefault
const deleteNodeKey = (mac ? 'command' : 'ctrl') + '+backspace'
const saveSnapshotKey = (mac? 'command' : 'ctrl') + '+s'
const addbottomCellKey = (mac? 'command': 'ctrl') + (mac? '+option' : '+alt') + '+enter'
const runCellKey = (mac? 'command' : 'ctrl') + '+enter'
const globalSearch = (mac? 'command' : 'ctrl') + '+p'

const addKeyListener = (keys, callback) => {
  Mousetrap.bind(keys, callback, 'keydown')
}
const removeKeyListener = (keys) => {
  Mousetrap.unbind(keys, 'keydown')
}

// fileTree Delete file
export const addDeleteNodeKey = (callback) => {
  addKeyListener(deleteNodeKey, callback)
}
export const removeDeleteNodeKey = () => {
  removeKeyListener(deleteNodeKey)
}

// NoteBookonFocuse
export const NoteBookonFocuse = ({
  enterCallback,
  downIncrementCallback, 
  upIncrementCallback, 
  saveSnapshot,
  rebootKernel,
  stopAllCellKey,
  deleteCell,
  addDownCell,
  selectUnitAbove,
  selectUnitBelow,
  runCurrentCellfocusNextCell,
  runCurrentCell,
  withdrawCell,
  reverseWithdrawal}) => {
  // 进入编辑模式
  addKeyListener('enter', enterCallback)
  // 向下增加单元格
  addKeyListener('a', downIncrementCallback)
  // 向上增加单元格
  addKeyListener('b', upIncrementCallback)
  // 保存快照（保存）
  addKeyListener(saveSnapshotKey, saveSnapshot)
  // 重启 NoteBook 内核 
  addKeyListener('0 0',rebootKernel)
  // 暂停 note book cell 运行
  addKeyListener('i i', stopAllCellKey)
  // 删除 cell
  addKeyListener('d d', deleteCell)
  // 是在下方添加新的 cell
  addKeyListener(addbottomCellKey, addDownCell)
  // 选中上方单元
  addKeyListener('up',selectUnitAbove)
  // 选中下方单元
  addKeyListener('down', selectUnitBelow)
  // 运行本单元 选中下一个单元
  addKeyListener('shift+enter', runCurrentCellfocusNextCell)
  // 运行本单元
  addKeyListener(runCellKey, runCurrentCell)
  // 撤回cell cell级别操作
  addKeyListener('z', withdrawCell)
  // 反撤回 cell 级别操作
  addKeyListener('shift+z', reverseWithdrawal)
}
export const NoteBookonBlur = () => {
  removeKeyListener('enter')
  removeKeyListener('a')
  removeKeyListener('b')
  removeKeyListener(saveSnapshotKey)
  removeKeyListener('0 0')
  removeKeyListener('i i')
  removeKeyListener('d d')
  removeKeyListener(addbottomCellKey)
  removeKeyListener('up')
  removeKeyListener('down')
  removeKeyListener('shift+enter')
  removeKeyListener(runCellKey)
  removeKeyListener('z')
  removeKeyListener('shift+z')
}


// OperatorFocuse
export const OperatorFocus = ({
  selectDown,
  selectUp
}) => {
  addKeyListener("down", selectDown)
  addKeyListener('up', selectUp)
}
export const OperatorBlur = () => {
  removeKeyListener('up')
  removeKeyListener('down')
}

// Global Focuse
export const GloablFocus = ({
  openGlobalSearch
}) => {
  addKeyListener(globalSearch,openGlobalSearch)
}
export const GloablBlur = () => {
  removeKeyListener(globalSearch)
}