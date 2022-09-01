import _extends from "@babel/runtime/helpers/extends";
import _objectWithoutPropertiesLoose from "@babel/runtime/helpers/objectWithoutPropertiesLoose";
var _excluded = ["prefixCls", "className", "onClick", "toolbars", "editor", "mode", "preview", "container", "containerEditor", "editorProps"];
import * as React from "react";
import "./index.css";
import { defaultCommands } from '../../commands';
import { createElement as _createElement } from "react";
import { jsx as _jsx } from "react/jsx-runtime";
export default function ToolBar(props) {
  var {
    prefixCls = 'md-editor',
    className,
    toolbars = [],
    editor,
    mode,
    preview,
    container,
    containerEditor,
    editorProps = {}
  } = props,
      htmlProps = _objectWithoutPropertiesLoose(props, _excluded);

  if (!toolbars || toolbars.length === 0) return null;

  function handleClick(execute) {
    if (execute && editor) {
      execute(editor, editor.getSelection(), editor.getCursor(), {
        preview,
        container
      });
    }
  }

  return /*#__PURE__*/_jsx("div", _extends({
    className: prefixCls + "-toolbar " + (className || '') + " " + (mode ? prefixCls + "-toolbar-mode" : '')
  }, htmlProps, {
    children: [...toolbars].map((command, key) => {
      var buttonProps = {
        type: 'button'
      };
      var obj = typeof command === 'string' ? defaultCommands[command] : command;
      if (!obj) return null;
      buttonProps.children = obj.icon;

      buttonProps.onClick = () => handleClick(obj.execute);

      if (obj.button && typeof obj.button === 'object') {
        var btn = obj.button;
        Object.keys(btn).forEach(key => {
          buttonProps[key] = btn[key];
        });
      } else if (typeof obj.button === 'function') {
        return /*#__PURE__*/React.cloneElement(obj.button(obj, editorProps, {
          preview,
          container,
          containerEditor,
          editor
        }), {
          key
        });
      }

      return /*#__PURE__*/_createElement("button", _extends({}, buttonProps, {
        key: key
      }));
    })
  }));
}
//# sourceMappingURL=index.js.map