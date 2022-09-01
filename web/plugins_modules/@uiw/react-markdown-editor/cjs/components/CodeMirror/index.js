"use strict";

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = void 0;

var _objectSpread2 = _interopRequireDefault(require("@babel/runtime/helpers/objectSpread2"));

var _classCallCheck2 = _interopRequireDefault(require("@babel/runtime/helpers/classCallCheck"));

var _createClass2 = _interopRequireDefault(require("@babel/runtime/helpers/createClass"));

var _possibleConstructorReturn2 = _interopRequireDefault(require("@babel/runtime/helpers/possibleConstructorReturn"));

var _assertThisInitialized2 = _interopRequireDefault(require("@babel/runtime/helpers/assertThisInitialized"));

var _inherits2 = _interopRequireDefault(require("@babel/runtime/helpers/inherits"));

var _createSuper2 = _interopRequireDefault(require("@babel/runtime/helpers/createSuper"));

var _defineProperty2 = _interopRequireDefault(require("@babel/runtime/helpers/defineProperty"));

require("codemirror/mode/markdown/markdown");

var _react = _interopRequireWildcard(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var SERVER_RENDERED = typeof navigator === 'undefined' || global.PREVENT_CODEMIRROR_RENDER === true;
var cm;

if (!SERVER_RENDERED) {
  // tslint:disable-next-line: no-var-requires
  cm = require('codemirror');
}

var ReactCodeMirror = /*#__PURE__*/function (_Component) {
  (0, _inherits2.default)(ReactCodeMirror, _Component);

  var _super = (0, _createSuper2.default)(ReactCodeMirror);

  // public editor!: Doc | Editor | EditorFromTextArea | Editor;
  function ReactCodeMirror(props) {
    var _this;

    (0, _classCallCheck2.default)(this, ReactCodeMirror);
    _this = _super.call(this, props);
    (0, _defineProperty2.default)((0, _assertThisInitialized2.default)(_this), "textarea", void 0);
    (0, _defineProperty2.default)((0, _assertThisInitialized2.default)(_this), "editor", void 0);

    if (SERVER_RENDERED) {
      return (0, _possibleConstructorReturn2.default)(_this);
    }

    if (_this.props.editorWillMount) {
      _this.props.editorWillMount();
    }

    return _this;
  }

  (0, _createClass2.default)(ReactCodeMirror, [{
    key: "render",
    value: function render() {
      var _this2 = this;

      return /*#__PURE__*/(0, _jsxRuntime.jsx)("textarea", {
        ref: function ref(instance) {
          return _this2.textarea = instance;
        }
      });
    }
  }, {
    key: "componentDidMount",
    value: function componentDidMount() {
      if (SERVER_RENDERED) {
        return;
      }

      var options = this.props.options;

      if (this.props.defineMode) {
        if (this.props.defineMode.name && this.props.defineMode.fn) {
          cm.defineMode(this.props.defineMode.name, this.props.defineMode.fn);
        }
      }

      var editorOption = (0, _objectSpread2.default)((0, _objectSpread2.default)({
        tabSize: 2,
        lineNumbers: true
      }, options), {}, {
        mode: 'markdown'
      }); // 生成codemirror实例

      this.editor = cm.fromTextArea(this.textarea, editorOption);
      this.renderCodeMirror(this.props);
    }
  }, {
    key: "componentDidUpdate",
    value: function componentDidUpdate(prevProps) {
      var _this$props = this.props,
          value = _this$props.value,
          width = _this$props.width,
          height = _this$props.height;

      if (this.editor.getValue() !== value && value !== prevProps.value) {
        this.editor.setValue(value || '');
      }

      if (width !== prevProps.width || height !== prevProps.height) {
        // Setting Size
        this.editor.setSize(width, height);
      }
    }
  }, {
    key: "shouldComponentUpdate",
    value: function shouldComponentUpdate(nextProps, nextState) {
      return nextProps.value !== this.props.value || nextProps.options !== this.props.options || nextProps.height !== this.props.height || nextProps.width !== this.props.width;
    } // 将 props 中所有的事件处理函数映射并保存

  }, {
    key: "getEventHandleFromProps",
    value: function getEventHandleFromProps() {
      var propNames = Object.keys(this.props);
      var eventHandle = propNames.filter(function (prop) {
        return /^on+/.test(prop);
      });
      var eventDict = {};
      eventHandle.forEach(function (ele) {
        eventDict[ele] = ele.replace(/^on[A-Z]/g, function (s) {
          return s.slice(2).toLowerCase();
        });
      });
      return eventDict;
    }
  }, {
    key: "renderCodeMirror",
    value: function renderCodeMirror(props) {
      var _this3 = this;

      var value = props.value,
          width = props.width,
          height = props.height; // 获取CodeMirror用于获取其中的一些常量
      // 事件处理映射

      var eventDict = this.getEventHandleFromProps();
      Object.keys(eventDict).forEach(function (event) {
        var handle = _this3.props[event];

        _this3.editor.on(eventDict[event], handle);
      }); // Init value

      this.editor.setValue(value || ''); // this.editor.setOption(name, editorOption.mode);

      if (width || height) {
        // Setting size
        this.editor.setSize(width, height);
      }
    }
  }]);
  return ReactCodeMirror;
}(_react.Component);

exports.default = ReactCodeMirror;
(0, _defineProperty2.default)(ReactCodeMirror, "defaultProps", {
  height: '100%',
  options: {
    lineNumbers: true,
    mode: 'markdown',
    tabSize: 2
  },
  value: '',
  width: '100%'
});
module.exports = exports.default;
//# sourceMappingURL=index.js.map