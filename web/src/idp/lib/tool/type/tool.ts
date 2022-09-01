import React from 'react';
/**
 * menu类型
 */
export type Tool = {
    key: number | string, // 唯一
    label?: string | Element | React.ReactNode | React.ReactElement,
    children?: ToolMap,
    nodeKey?: string,
    items?: Function,
    render?: React.ReactNode | Element | React.ReactElement;
};

export type ToolMap = Array<Tool>;