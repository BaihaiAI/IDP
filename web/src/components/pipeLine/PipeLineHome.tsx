import React, {useState, createContext, useEffect} from "react"
import { Tabs } from "antd"
import "./PipeLineHome.less"
import PipeLineListView from "./components/PipeLineListView"
import intl from "react-intl-universal"
import Pubsub from "pubsub-js"

interface PaneData {
  title: string
  content: JSX.Element
  key: string
  closable?: boolean
}

interface PipeLineContextApi {
  addTabPane: (paneData: PaneData) => void
  tabKeyCount: number
  pipeHomeTabActiveKey: string
}

const { TabPane } = Tabs
export const PipeLineContext = createContext<Partial<PipeLineContextApi>>({})

let tabKeyCount = 0

function PipeLineHome() {
  const initialPanes: PaneData[] = [
    {
      title: intl.get("WORKFLOW_INSTANCES_AND_WORKFLOW_LISTS"),
      content: <PipeLineListView />,
      key: "pipeLineListView",
      closable: false,
    },
  ]

  const [activeKey, setActiveKey] = useState(initialPanes[0].key)
  const [panes, setPanes] = useState(initialPanes)

  useEffect(() => {
    const subscriber = Pubsub.subscribe('addTabPane',(msg,paneData)=>{
      addTabPane(paneData)
    })
    return () => {
      Pubsub.unsubscribe(subscriber)
    }
  }, [])

  const onChange = (activeKey) => {
    setActiveKey(activeKey)
  }

  const onEdit = (targetKey, action) => {
    switch (action) {
      case "remove":
        remove(targetKey)
        break
    }
  }

  const addTabPane = (paneData: PaneData) => {
    const findResult = panes.find((item) => item.key === paneData.key)
    const newActiveKey = paneData.key
    if (!findResult) {
      const newPanes = [...panes]
      newPanes.push(paneData)
      tabKeyCount++
      setPanes(newPanes)
    }
    setActiveKey(newActiveKey)
  }
/*  const changeTabPaneTitle = (key: string,title:string) => {
    return (title: string) => {
      setPanes((panes) => {
        return panes.map((pane) => {
          if (pane.key !== key) {
            return pane
          } else {
            return {
              ...pane,
              title,
            }
          }
        })
      })
    }
  }*/
  const remove = (targetKey) => {
    let newActiveKey = activeKey
    let lastIndex
    panes.forEach((pane, i) => {
      if (pane.key === targetKey) {
        lastIndex = i - 1
      }
    })
    const newPanes = panes.filter((pane) => pane.key !== targetKey)
    if (newPanes.length && newActiveKey === targetKey) {
      if (lastIndex >= 0) {
        newActiveKey = newPanes[lastIndex].key
      } else {
        newActiveKey = newPanes[0].key
      }
    }
    setActiveKey(newActiveKey)
    setPanes(newPanes)
  }

  return (
    <div>
      <div id={"pipe-line-tabs-container"} style={{ minHeight: document.body.clientHeight - 58}}>
        <PipeLineContext.Provider
          value={{ addTabPane, tabKeyCount, pipeHomeTabActiveKey: activeKey }}
        >
          <Tabs
            hideAdd
            type="editable-card"
            onChange={onChange}
            activeKey={activeKey}
            onEdit={onEdit}
            className={"pipe-line-home-tabs"}
          >
            {panes.map((pane) => (
              <TabPane tab={pane.title} key={pane.key} closable={pane.closable}>
                {pane.content}
              </TabPane>
            ))}
          </Tabs>
        </PipeLineContext.Provider>
      </div>
    </div>
  )
}

export default PipeLineHome
