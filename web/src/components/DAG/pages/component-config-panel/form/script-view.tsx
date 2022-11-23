import { Drawer, Space, Spin } from "antd"
import React, {useContext, useEffect, useState} from "react"
import contentApi from "idpServices/contentApi"
import { HtmlView } from "./html-view"
import { UnControlled as CodeMirror } from 'react-codemirror2';
import { EditOutlined } from "@ant-design/icons";
import globalData from "idp/global"
import {observer} from "mobx-react"

export interface Props {
  path: string
  visible: boolean
  onClose: Function
  expGraph?: any
}

export const ScriptView: React.FC<Props> = observer(({ path, visible, onClose, expGraph }) => {
  const [scriptVisible, setScriptVisible] = useState(visible)
  const [loading, setLoading] = useState(true);
  const [scriptContent, setScriptContent] = useState('')
  const suffix = path.slice(path.lastIndexOf('.'))
  const {workspaceRef} = globalData.appComponentData

  useEffect(() => {
    setScriptVisible(visible)
    if (visible) {
      showScript()
    }
  }, [visible])


  const showScript = () => {
    setScriptVisible(true);
    if (suffix === '.ipynb' || suffix === '.idpnb') {
      contentApi.ipynbPreview({ path }).then((res: any) => {
        const {content} = res.data
        setLoading(false);
        setScriptContent(content)
      }).catch((err: any) => {
        setLoading(false);
        setScriptContent(String(err))
        console.log(err)
      })
    } else {
      contentApi.cat({ path: path }).then((res: any) => {
        const { content } = res.data
        setLoading(false);
        setScriptContent(content)
      }).catch((err: any) => {
        setLoading(false);
        setScriptContent(String(err.message))
        console.log(err)
      })
    }
  }


  const handleClose = () => {
    setScriptVisible(false)
    onClose()
  }

  const view = (scriptContent: string) => {
    switch (suffix) {
      case '.ipynb':
      case '.idpnb':
        return (
          <HtmlView
            id="scriptContent"
            html={scriptContent}
          />
        )
      case '.py':
        return (
          <CodeMirror
            value={scriptContent}
            options={{
              readOnly: 'nocursor',
              theme: 'xq-light',
              mode: 'python',
            }}
          />
        )
      case '.sh':
        return (
          <CodeMirror
            value={scriptContent}
            options={{
              readOnly: 'nocursor',
              theme: 'xq-light',
              mode: 'shell',
            }}
          />
        )
      default:
        return (
          <HtmlView
            id="scriptContent"
            html={`<pre>${scriptContent}</pre>`}
          />
        )
    }
  }

  const title = () => {
    return (
      <Space>
        脚本内容
        <a style={{ fontSize: 12 }} onClick={() => {
          if (expGraph) {
            expGraph.saveExperimentGraphSync();
          }
          const node = {
            key: path,
            name: path.substring(path.lastIndexOf("/")),
            isLeaf: true,
            fileType: "FILE",
          }
          const info = {
            node: node,
          }
          handleClose()
          workspaceRef.onSelect([path], info)
        }}>编辑<EditOutlined /></a>
      </Space>
    )
  }

  return (
    <Drawer
      title={title()}
      placement="right"
      visible={scriptVisible}
      maskClosable={false}
      width={1000}
      onClose={handleClose}
      zIndex={2000}
      className={"script-view-right"}
    >
      <Spin
        spinning={loading}
        size="large"
        style={{margin:"0 auto",top:`${(document.body.clientHeight-57)/2}px`}}
      >
        {
          view(scriptContent)
        }
      </Spin>
    </Drawer>
  )
})
