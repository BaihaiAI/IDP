"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.quote = void 0;

var _react = _interopRequireDefault(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var quote = {
  name: 'quote',
  keyCommand: 'quote',
  button: {
    'aria-label': 'Add quote text'
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsx)("svg", {
    width: "12",
    height: "12",
    viewBox: "0 0 512 512",
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M512 80v128c0 137.018-63.772 236.324-193.827 271.172-15.225 4.08-30.173-7.437-30.173-23.199v-33.895c0-10.057 6.228-19.133 15.687-22.55C369.684 375.688 408 330.054 408 256h-72c-26.51 0-48-21.49-48-48V80c0-26.51 21.49-48 48-48h128c26.51 0 48 21.49 48 48zM176 32H48C21.49 32 0 53.49 0 80v128c0 26.51 21.49 48 48 48h72c0 74.054-38.316 119.688-104.313 143.528C6.228 402.945 0 412.021 0 422.078v33.895c0 15.762 14.948 27.279 30.173 23.199C160.228 444.324 224 345.018 224 208V80c0-26.51-21.49-48-48-48z"
    })
  }),
  execute: function execute(editor, selection, position) {
    var value = selection ? "> ".concat(selection) : '> ';
    editor.replaceSelection(value);
    position.ch = !!selection ? position.ch : position.ch + 2;
    editor.setCursor(position.line, position.ch);
    editor.focus();
  }
};
exports.quote = quote;
//# sourceMappingURL=quote.js.map