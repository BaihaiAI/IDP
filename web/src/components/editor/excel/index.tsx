import React from 'react';
import { Tabs } from 'antd';
import './excel.less'

interface Props {
  path: string
  item: {
    [p in string]: any
  }
}

export const ExcelEditor: React.FC<Props> = (props: Props) => {
  try {
    const content = JSON.parse(props.item.content)
    const items = []
    for (const key in content) {
      items.push({
        label: key,
        children: <div
          className="excel-view-table"
          dangerouslySetInnerHTML={{ __html: `<div id={key}>${content[key]}<div>` }}></div>,
      })
    }
    return (<div className="excel-view">
      <Tabs
        size="small"
        tabPosition="bottom"
        tabBarStyle={{ marginTop: 2, height: 24 }}
      >
        {items.map((value) => <Tabs.TabPane tab={value.label} key={value.label}>{value.children}</Tabs.TabPane>)}
      </Tabs>
    </div>)
  } catch (error) {
    return (<pre>
      {props.item.content}
    </pre>)
  }
}