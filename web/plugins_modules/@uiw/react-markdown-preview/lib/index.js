"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = void 0;

var _toConsumableArray2 = _interopRequireDefault(require("@babel/runtime/helpers/toConsumableArray"));

var _objectWithoutProperties2 = _interopRequireDefault(require("@babel/runtime/helpers/objectWithoutProperties"));

var _objectSpread2 = _interopRequireDefault(require("@babel/runtime/helpers/objectSpread2"));

var _react = _interopRequireWildcard(require("react"));

var _reactMarkdown = _interopRequireDefault(require("react-markdown"));

var _remarkGfm = _interopRequireDefault(require("remark-gfm"));

var _rehypeSlug = _interopRequireDefault(require("rehype-slug"));

var _rehypeAutolinkHeadings = _interopRequireDefault(require("rehype-autolink-headings"));

var _rehypeRaw = _interopRequireDefault(require("rehype-raw"));

var _rehypeAttr = _interopRequireDefault(require("rehype-attr"));

var _rehypePrism = _interopRequireDefault(require("@mapbox/rehype-prism"));

var _rehypeRewrite = _interopRequireDefault(require("rehype-rewrite"));

var _jsxRuntime = require("react/jsx-runtime");

var _excluded = ["prefixCls", "className", "source", "style", "onScroll", "onMouseOver", "warpperElement"];
var octiconLink = {
  type: 'element',
  tagName: 'svg',
  properties: {
    class: 'octicon octicon-link',
    viewBox: '0 0 16 16',
    version: '1.1',
    width: '16',
    height: '16',
    ariaHidden: 'true'
  },
  children: [{
    type: 'element',
    tagName: 'path',
    children: [],
    properties: {
      fillRule: 'evenodd',
      d: 'M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z'
    }
  }]
};

var rehypeRewriteHandle = function rehypeRewriteHandle(node, index, parent) {
  if (node.type === 'element' && parent && parent.type === 'root' && /h(1|2|3|4|5|6)/.test(node.tagName)) {
    var child = node.children && node.children[0];

    if (child && child.properties && child.properties.ariaHidden === 'true') {
      child.properties = (0, _objectSpread2.default)({
        class: 'anchor'
      }, child.properties);
      child.children = [octiconLink];
    }
  }
};

var _default = /*#__PURE__*/_react.default.forwardRef(function (props, ref) {
  var _props$prefixCls = props.prefixCls,
      prefixCls = _props$prefixCls === void 0 ? 'wmde-markdown wmde-markdown-color' : _props$prefixCls,
      className = props.className,
      source = props.source,
      style = props.style,
      onScroll = props.onScroll,
      onMouseOver = props.onMouseOver,
      _props$warpperElement = props.warpperElement,
      warpperElement = _props$warpperElement === void 0 ? {} : _props$warpperElement,
      other = (0, _objectWithoutProperties2.default)(props, _excluded);

  var mdp = /*#__PURE__*/_react.default.createRef();

  (0, _react.useImperativeHandle)(ref, function () {
    return (0, _objectSpread2.default)((0, _objectSpread2.default)({}, props), {}, {
      mdp: mdp
    });
  }, [mdp, props]);
  var cls = "".concat(prefixCls || '', " ").concat(className || '');
  return /*#__PURE__*/(0, _jsxRuntime.jsx)("div", (0, _objectSpread2.default)((0, _objectSpread2.default)({
    ref: mdp,
    onScroll: onScroll,
    onMouseOver: onMouseOver
  }, warpperElement), {}, {
    className: cls,
    style: style,
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)(_reactMarkdown.default, (0, _objectSpread2.default)((0, _objectSpread2.default)({}, other), {}, {
      rehypePlugins: [[_rehypePrism.default, {
        ignoreMissing: true
      }], _rehypeRaw.default, _rehypeSlug.default, _rehypeAutolinkHeadings.default, [_rehypeRewrite.default, {
        rewrite: rehypeRewriteHandle
      }], [_rehypeAttr.default, {
        properties: 'attr'
      }]].concat((0, _toConsumableArray2.default)(other.rehypePlugins || [])),
      remarkPlugins: [].concat((0, _toConsumableArray2.default)(other.remarkPlugins || []), [_remarkGfm.default]),
      children: source || ''
    }))
  }));
});

exports.default = _default;
module.exports = exports.default;
//# sourceMappingURL=index.js.map