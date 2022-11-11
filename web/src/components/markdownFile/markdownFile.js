import React, { useState, useEffect } from "react";
import './markdownFile.less'
import { Col, Row } from "antd"
import MarkdownEditor from '@uiw/react-markdown-editor';
import { observer } from "mobx-react";
import Terminal from "@/idp/lib/terminal";


const MarkdownFile = (props,ref) => {
  const { path, name,content } = props.item;
  const [value, setValue] = useState(content)

  return (
    <div className="mdfile"
        style={{
          height: Terminal.workspaceHeight - 60,
        }}>
      <Row>
        <Col span={24}>
          <MarkdownEditor
            value={value}
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
