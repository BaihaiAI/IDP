import { useEffect, useState } from 'react';
import { Table } from 'antd';
import './csvmode.less'
function Preview(props) {
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
    if(tablecontent.length){
      setTotal(tablecontent.length)

      let { height } = document.getElementsByClassName("csv-preview")[0].getBoundingClientRect()
      height = height - (39 + 40)
      height = Math.floor(height / 39)
      setPageSize(height)
    }
  }, [tablecontent])

  useEffect(() => {
    let res = content.split("\n")
    let resTitle = [],
      resBody = [];
    if (res.length === 0 || res[0].trim() === '') return
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
        height: document.body.clientHeight - 159,
        overflow: 'auto',
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
          current: currentPage,
          hideOnSinglePage: true,
          total,
          pageSize,
          onChange: (page, pageSize) => {
            setCurrentPage(page)
          },
        }}


        />
    </div>
  )
}
export default (Preview)
