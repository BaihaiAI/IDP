"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const anser_1 = __importDefault(require("anser"));
const escape_carriage_1 = require("escape-carriage");
const React = __importStar(require("react"));
/**
 * Converts ANSI strings into JSON output.
 * @name ansiToJSON
 * @function
 * @param {String} input The input string.
 * @param {boolean} use_classes If `true`, HTML classes will be appended
 *                              to the HTML output.
 * @return {Array} The parsed input.
 */
function ansiToJSON(input, use_classes = false) {
    input = escape_carriage_1.escapeCarriageReturn(fixBackspace(input));
    return anser_1.default.ansiToJson(input, {
        json: true,
        remove_empty: true,
        use_classes,
    });
}
/**
 * Create a class string.
 * @name createClass
 * @function
 * @param {AnserJsonEntry} bundle
 * @return {String} class name(s)
 */
function createClass(bundle) {
    let classNames = "";
    if (bundle.bg) {
        classNames += `${bundle.bg}-bg `;
    }
    if (bundle.fg) {
        classNames += `${bundle.fg}-fg `;
    }
    if (bundle.decoration) {
        classNames += `ansi-${bundle.decoration} `;
    }
    if (classNames === "") {
        return null;
    }
    classNames = classNames.substring(0, classNames.length - 1);
    return classNames;
}
/**
 * Create the style attribute.
 * @name createStyle
 * @function
 * @param {AnserJsonEntry} bundle
 * @return {Object} returns the style object
 */
function createStyle(bundle) {
    const style = {};
    if (bundle.bg) {
        style.backgroundColor = `rgb(${bundle.bg})`;
    }
    if (bundle.fg) {
        style.color = `rgb(${bundle.fg})`;
    }
    switch (bundle.decoration) {
        case 'bold':
            style.fontWeight = 'bold';
            break;
        case 'dim':
            style.opacity = '0.5';
            break;
        case 'italic':
            style.fontStyle = 'italic';
            break;
        case 'hidden':
            style.visibility = 'hidden';
            break;
        case 'strikethrough':
            style.textDecoration = 'line-through';
            break;
        case 'underline':
            style.textDecoration = 'underline';
            break;
        case 'blink':
            style.textDecoration = 'blink';
            break;
        default:
            break;
    }
    return style;
}
/**
 * Converts an Anser bundle into a React Node.
 * @param linkify whether links should be converting into clickable anchor tags.
 * @param useClasses should render the span with a class instead of style.
 * @param bundle Anser output.
 * @param key
 */
function convertBundleIntoReact(linkify, useClasses, clickHandle, errorConfig, bundle, key) {
    const style = useClasses ? null : createStyle(bundle);
    const className = useClasses ? createClass(bundle) : null;
    // 处理错误跳转
    let result;
    let config = errorConfig;
    const errorLineRegex = /(-+)?>\s(\d+)/g;
    const syntaxErrorFlagType = /invalid syntax.+line.(\d+)/g;
    /* start type: namedErrorFlag*/
    if ((result = errorLineRegex.exec(bundle.content)) !== null && config['errorCounter'] == 0) {
        config.namedErrorFlag = true;
        config.info.content = []; //reset
        config.info.errorLine = result[2];
        config.info.content.push(React.createElement("span", { style, key, className }, bundle.content));
        return React.createElement('span');
    }
    if (config.namedErrorFlag) {
        if (/\n/.test(bundle.content)) {
            config.namedErrorFlag = false; // reset
            config.errorCounter++; //only for frist Error line
            return React.createElement("p", { style, key, className: "output-error-line", onClick: () => { clickHandle(config.info); } }, config.info.content);
        }
        config.info.content.push(React.createElement("span", { style, key, className }, bundle.content));
        return React.createElement('span');
    }
    /* end type: namedErrorFlag*/
    /* start type: syntaxErrorFlag */
    if ((result = syntaxErrorFlagType.exec(bundle.content)) !== null) {
        config.info.errorLine = result[1];
        config.syntaxErrorFlag = true;
    }
    if (/\n/.test(bundle.content) && config.syntaxErrorFlag) {
        config.enterNum++;
    }
    if (config.enterNum == 2) {
        config.info.content.push(React.createElement("span", { style, key, className }, bundle.content));
        return React.createElement('span');
    }
    if (config.enterNum == 3 && config.syntaxErrorFlag) {
        config.syntaxErrorFlag = false;
        return React.createElement("p", { style, key, className: "output-error-line", onClick: () => { clickHandle(config.info); } }, config.info.content);
    }
    /*end syntaxErrorFlag*/
    if (!linkify) {
        return React.createElement("span", { style, key, className }, bundle.content);
    }
    const content = [];
    const linkRegex = /(\s|^)(https?:\/\/(?:www\.|(?!www))[^\s.]+\.[^\s]{2,}|www\.[^\s]+\.[^\s]{2,})/g;
    let index = 0;
    let match;
    while ((match = linkRegex.exec(bundle.content)) !== null) {
        const [, pre, url] = match;
        const startIndex = match.index + pre.length;
        if (startIndex > index) {
            content.push(bundle.content.substring(index, startIndex));
        }
        // Make sure the href we generate from the link is fully qualified. We assume http
        // if it starts with a www because many sites don't support https
        const href = url.startsWith("www.") ? `http://${url}` : url;
        content.push(React.createElement("a", {
            key: index,
            href,
            target: "_blank",
        }, `${url}`));
        index = linkRegex.lastIndex;
    }
    if (index < bundle.content.length) {
        content.push(bundle.content.substring(index));
    }
    return React.createElement("span", { style, key, className }, content);
}
function Ansi(props) {
    var _a;
    const { className, useClasses, children, linkify, clickHandle } = props;
    let errorConfig = (_a = props.errorConfig) !== null && _a !== void 0 ? _a : Object.create({
        namedErrorFlag: false,
        syntaxErrorFlag: false,
        errorCounter: 0,
        enterNum: 0,
        info: {
            content: [],
            errorLine: 0,
            type: '',
        }
    });
    return React.createElement("code", { className }, ansiToJSON(children !== null && children !== void 0 ? children : "", useClasses !== null && useClasses !== void 0 ? useClasses : false).map(convertBundleIntoReact.bind(null, linkify !== null && linkify !== void 0 ? linkify : false, useClasses !== null && useClasses !== void 0 ? useClasses : false, clickHandle, errorConfig)));
}
exports.default = Ansi;
// This is copied from the Jupyter Classic source code
// notebook/static/base/js/utils.js to handle \b in a way
// that is **compatible with Jupyter classic**.   One can
// argue that this behavior is questionable:
//   https://stackoverflow.com/questions/55440152/multiple-b-doesnt-work-as-expected-in-jupyter#
function fixBackspace(txt) {
    let tmp = txt;
    do {
        txt = tmp;
        // Cancel out anything-but-newline followed by backspace
        tmp = txt.replace(/[^\n]\x08/gm, "");
    } while (tmp.length < txt.length);
    return txt;
}
