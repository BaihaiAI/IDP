import React, { useState, useEffect } from "react";
import './markdownFile.less'
import { Col, Row } from "antd"
import MarkdownEditor from '@uiw/react-markdown-editor';
import { observer } from "mobx-react";
import Terminal from "@/idp/lib/terminal";


const MarkdownFile = (props) => {

    const { content } = props.item;
    const mdRef = React.useRef();

    const updateMarkdownEditorHeight = () => {
        const cmsDefault = document.querySelector('#wrokspace_file_md .cm-s-default');
        const dpreve = document.querySelector('#wrokspace_file_md .md-editor-preview');
        let markdownEditorHeight = process.env.NODE == 'dev ' ? Terminal.workspaceHeight - 177 : (Terminal.workspaceHeight - 177 + 157);
        if (dpreve) {
            dpreve.style.setProperty('height', `${markdownEditorHeight}px`, 'important');
        }
        if (cmsDefault) {
            cmsDefault.style.setProperty('height', `${markdownEditorHeight}px`, 'important');
        }
    }

    useEffect(() => {
        console.log('@');
        updateMarkdownEditorHeight();
    }, [Terminal.workspaceHeight]);

    return (
        <div id='wrokspace_file_md' className="mdfile" >
            <Row>
                <Col span={24}>
                    <MarkdownEditor
                        value={content}
                        visible={false} //开启预览
                        initialMode={true} // 判断是否是 "插件初始化" 模式
                        options={{ lineNumbers: false }}
                    />
                </Col>
            </Row>
        </div>
    )
}


export default observer(React.forwardRef(MarkdownFile))
