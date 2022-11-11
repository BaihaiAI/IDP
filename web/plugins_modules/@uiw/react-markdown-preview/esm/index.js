import _objectWithoutPropertiesLoose from "@babel/runtime/helpers/objectWithoutPropertiesLoose";
import _extends from "@babel/runtime/helpers/extends";
var _excluded = ["prefixCls", "className", "source", "style", "onScroll", "onMouseOver", "warpperElement"];
import React, { useImperativeHandle } from 'react';
import ReactMarkdown from 'react-markdown';
import gfm from 'remark-gfm';
import slug from 'rehype-slug';
import headings from 'rehype-autolink-headings';
import rehypeRaw from 'rehype-raw';
import rehypeAttrs from 'rehype-attr'; // @ts-ignore

import rehypePrism from '@mapbox/rehype-prism';
import rehypeRewrite from 'rehype-rewrite';
import "./styles/markdown.css";
import "./styles/markdowncolor.css";
import { jsx as _jsx } from "react/jsx-runtime";
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

var rehypeRewriteHandle = (node, index, parent) => {
  if (node.type === 'element' && parent && parent.type === 'root' && /h(1|2|3|4|5|6)/.test(node.tagName)) {
    var child = node.children && node.children[0];

    if (child && child.properties && child.properties.ariaHidden === 'true') {
      child.properties = _extends({
        class: 'anchor'
      }, child.properties);
      child.children = [octiconLink];
    }
  }
};

export default /*#__PURE__*/React.forwardRef((props, ref) => {
  var {
    prefixCls = 'wmde-markdown wmde-markdown-color',
    className,
    source,
    style,
    onScroll,
    onMouseOver,
    warpperElement = {}
  } = props,
      other = _objectWithoutPropertiesLoose(props, _excluded);

  var mdp = /*#__PURE__*/React.createRef();
  useImperativeHandle(ref, () => _extends({}, props, {
    mdp
  }), [mdp, props]);
  var cls = (prefixCls || '') + " " + (className || '');
  return /*#__PURE__*/_jsx("div", _extends({
    ref: mdp,
    onScroll: onScroll,
    onMouseOver: onMouseOver
  }, warpperElement, {
    className: cls,
    style: style,
    children: /*#__PURE__*/_jsx(ReactMarkdown, _extends({}, other, {
      rehypePlugins: [[rehypePrism, {
        ignoreMissing: true
      }], rehypeRaw, slug, headings, [rehypeRewrite, {
        rewrite: rehypeRewriteHandle
      }], [rehypeAttrs, {
        properties: 'attr'
      }], ...(other.rehypePlugins || [])],
      remarkPlugins: [...(other.remarkPlugins || []), gfm],
      children: source || ''
    }))
  }));
});
//# sourceMappingURL=index.js.map