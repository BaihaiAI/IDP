import { useEffect, useState } from 'react';
import { Table } from 'antd';
import './csvmode.less'
import { observer } from 'mobx-react';
import terminal from '@/idp/lib/terminal';
import pages from '@/idp/plugins/headerIdp/pages';
function Preview(props) {
  // 这个文件如果要改动，问一下杨华彬再改，会有意想不到的bug
  const { content } = props
  const [tableHeader, setTableHeader] = useState([])
  const [tablecontent, setTableContent] = useState([])

  const [total, setTotal] = useState(0)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)


  const compare = (value1, value2) => {
    if (!isNaN(parseInt(value1)) && !isNaN(parseInt(value2))) {
      return parseInt(value1) - parseInt(value2)
    } else {
      return value1.localeCompare(value2)
    }
  }

  useEffect(() => {
    console.log(currentPage, 'currentPage')
  }, [currentPage])

  useEffect(() => {
      setTotal(tablecontent.length)

      let { height } = document.getElementsByClassName("csv-preview")[0].getBoundingClientRect()
      height = height - (39 + 40)
      height = Math.floor(height / 39)
      height = height > 0 ? height : 20;
      setPageSize(height)
  }, [])

  useEffect(() => {
    let res = content.split("\n")
    let resTitle = [],
      resBody = [];
    if (res.length === 0 || res[0].trim() === '') {
      setTableHeader([]);
      setTableContent([]);
      return;
    }
    const titles = res[0].split(',')
    for (let i = 0; i < titles.length; i++) {
      const name = titles[i];
      resTitle.push({
        title: name,
        dataIndex: i,
        key: i,
        sorter: (a, b) => compare(a[i], b[i]),
      })
    }

    for (let i = 1; i < res.length; i++) {
      if (res[i].trim() === '') continue
      const elementArr = res[i].split(',')
      const obj = { key: 'body-' + i }
      for (let j = 0; j < titles.length; j++) {
        obj[j] = elementArr[j]
      }
      resBody.push(obj)
    }
    setTableHeader(resTitle)
    setTableContent(resBody)
  }, [content])

  return (
    <div
      className='csv-preview'
      style={{
        width: '100%',
        height: terminal.workspaceHeight - 75 - 90,
        overflow: 'scroll',
      }}>
      <Table
        columns={tableHeader}
        dataSource={tablecontent}
        size="small"
        // pagination={{
        //   showQuickJumper: true,
        //   hideOnSinglePage: true,
        //   showSizeChanger: true,
        // }}
        pagination={{
          // defaultCurrent: 1,
          // current: Math.floor(currentPage),
          // hideOnSinglePage: true,
          // total,
          pageSize,
          // onChange: (page, pageSize) => {
          //   setCurrentPage(page)
          //   setPageSize(pageSize)
          // },
        }}
        />
    </div>
  )
}
export default observer(Preview)