import React from 'react';
import { Options } from 'react-markdown';
import './styles/markdown.less';
import './styles/markdowncolor.less';
export declare type MarkdownPreviewProps = {
    prefixCls?: string;
    className?: string;
    source?: string;
    style?: React.CSSProperties;
    warpperElement?: React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
    onScroll?: (e: React.UIEvent<HTMLDivElement>) => void;
    onMouseOver?: (e: React.MouseEvent<HTMLDivElement>) => void;
} & Omit<Options, 'children'>;
export declare type MarkdownPreviewRef = {
    mdp: React.RefObject<HTMLDivElement>;
} & MarkdownPreviewProps;
declare const _default: React.ForwardRefExoticComponent<{
    prefixCls?: string | undefined;
    className?: string | undefined;
    source?: string | undefined;
    style?: React.CSSProperties | undefined;
    warpperElement?: React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement> | undefined;
    onScroll?: ((e: React.UIEvent<HTMLDivElement, UIEvent>) => void) | undefined;
    onMouseOver?: ((e: React.MouseEvent<HTMLDivElement, MouseEvent>) => void) | undefined;
} & Omit<import("react-markdown/lib/react-markdown").ReactMarkdownOptions, "children"> & React.RefAttributes<MarkdownPreviewRef>>;
export default _default;
