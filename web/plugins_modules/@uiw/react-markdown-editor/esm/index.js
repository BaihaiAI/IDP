import _extends from "@babel/runtime/helpers/extends";
import _objectWithoutPropertiesLoose from "@babel/runtime/helpers/objectWithoutPropertiesLoose";
var _excluded = ["prefixCls", "className", "onChange", "toolbars", "toolbarsMode", "visible", "visibleEditor", "previewProps"];
import React, { useState, createRef, useRef, useEffect, useImperativeHandle, useMemo } from 'react';
import CodeMirror from './components/CodeMirror';
import MarkdownPreview from '@uiw/react-markdown-preview';
import ToolBar from './components/ToolBar';
import { getCommands, getModeCommands } from './commands';
import "./index.css";
import { jsx as _jsx } from "react/jsx-runtime";
import { jsxs as _jsxs } from "react/jsx-runtime";
export * from './commands';
export default /*#__PURE__*/React.forwardRef(MarkdownEditor);

function MarkdownEditor(props, ref) {
  var {
    prefixCls = 'md-editor',
    className,
    onChange: _onChange,
    toolbars = getCommands(),
    toolbarsMode = getModeCommands(),
    visible = true,
    visibleEditor = true,
    previewProps = {}
  } = props,
      codemirrorProps = _objectWithoutPropertiesLoose(props, _excluded);

  var [value, setValue] = useState(props.value || '');
  var codeMirror = /*#__PURE__*/createRef();
  var previewContainer = useRef();
  var [editor, setEditor] = useState();
  var container = useRef(null);
  var containerEditor = useRef(null);
  useImperativeHandle(ref, () => ({
    editor: editor,
    preview: previewContainer.current
  }));
  useEffect(() => {
    if (codeMirror.current) {
      setEditor(codeMirror.current.editor);
    }
  }, [codeMirror.current]);
  var toolBarProps = {
    editor,
    preview: previewContainer.current,
    container: container.current,
    containerEditor: containerEditor.current,
    editorProps: props
  };
  var codeEditor = useMemo(() => /*#__PURE__*/_jsx(CodeMirror, _extends({
    visibleEditor: visibleEditor
  }, codemirrorProps, {
    ref: codeMirror,
    onChange: (editor, data) => {
      setValue(editor.getValue());
      _onChange && _onChange(editor, data, editor.getValue());
    }
  })), [visibleEditor, codemirrorProps]);
  return /*#__PURE__*/_jsx("div", {
    ref: container,
    children: /*#__PURE__*/_jsxs("div", {
      className: (prefixCls || '') + " " + (className || ''),
      children: [/*#__PURE__*/_jsx(ToolBar, _extends({}, toolBarProps, {
        toolbars: toolbarsMode,
        mode: true
      })), /*#__PURE__*/_jsx(ToolBar, _extends({}, toolBarProps, {
        toolbars: toolbars
      })), /*#__PURE__*/_jsxs("div", {
        className: prefixCls + "-content",
        children: [/*#__PURE__*/_jsx("div", {
          className: prefixCls + "-content-editor",
          ref: containerEditor,
          children: visibleEditor && codeEditor
        }), /*#__PURE__*/_jsx(MarkdownPreview, _extends({}, previewProps, {
          "data-visible": !!visible,
          className: prefixCls + "-preview",
          ref: instance => {
            if (instance && instance.mdp) {
              previewContainer.current = instance.mdp.current;
            }
          },
          source: value
        }))]
      })]
    })
  });
}
//# sourceMappingURL=index.js.map