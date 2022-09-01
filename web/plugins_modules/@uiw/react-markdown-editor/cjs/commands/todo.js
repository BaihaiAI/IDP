"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.todo = void 0;

var _react = _interopRequireDefault(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var todo = {
  name: 'todo',
  keyCommand: 'todo',
  button: {
    'aria-label': 'Add todo List'
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsx)("svg", {
    width: "18",
    height: "18",
    viewBox: "0 0 1024 1024",
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M224 800.256L223.712 224H320v31.68c0 35.456 28.64 64.32 63.872 64.32h256.256A64.16 64.16 0 0 0 704 255.68V224l96-0.256L800.256 800 224 800.256zM640 192.32L640.128 256 384 255.68V192.32L383.872 192 640 192.32zM799.84 160H695.04c-11.072-19.04-31.424-32-54.912-32h-256.256c-23.488 0-43.808 12.928-54.912 32H223.712A63.776 63.776 0 0 0 160 223.744v576.512C160 835.392 188.608 864 223.744 864h576.512A63.84 63.84 0 0 0 864 800.256V223.744A64 64 0 0 0 799.84 160zM619.072 429.088l-151.744 165.888-62.112-69.6a32 32 0 1 0-47.744 42.624l85.696 96a32 32 0 0 0 23.68 10.688h0.192c8.96 0 17.536-3.776 23.616-10.4l175.648-192a32 32 0 0 0-47.232-43.2"
    })
  }),
  execute: function execute(editor, selection, position) {
    var value = selection ? "- [ ] ".concat(selection) : '- [ ] ';
    editor.replaceSelection(value);
    position.ch = !!selection ? position.ch : position.ch + 6;
    editor.setCursor(position.line, position.ch);
    editor.focus();
  }
};
exports.todo = todo;
//# sourceMappingURL=todo.js.map