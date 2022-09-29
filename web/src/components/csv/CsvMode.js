import React, {useState, useEffect, useContext} from 'react'
import { Button, message } from 'antd';
import intl from 'react-intl-universal';
import Preview from './Preview';
import EditCsv from './EditCsv';
import './csvmode.less'
import {observer} from "mobx-react"
function CsvMode(props, ref){


  const { workSpaceHeight,item } = props;
  let { content:value, mime } = item

  // 以备日后用到
  if ('application/octet-stream' === mime) {
    value = intl.get('TEXT_EDITOR_OPEN_INFO');
  }

  const path = props.item.path;
  const posLine = props.item.posLine
  const [poe, setPoe] = useState(false)
  const [content, setContent] = useState(value)

  useEffect(() => {
    if (posLine) {
      setPoe(true)
    }
  }, [])

  const onChange = (value) => {
    setContent(value);
  }

  const previewOrEdit = () => {
    return poe? (
      <EditCsv
        ref={props.ref}
        key={props.item.path}
        content={content}
        {...props.item}
        onChange={onChange}
        deleteflag={props.deleteflag}
        workSpaceHeight={workSpaceHeight}
      />
    ):(
      <Preview
        content={content}
      />
    )
  }
  return(
    <div className='control-bar'>
      <div className='control'>
        <div className='control-box'>
          {poe
            ? (<Button key="preview" type="link" onClick={() => setPoe(false)}>{intl.get('PREVIEW')}</Button>)
            : (<Button key="edit" type="link" onClick={() => setPoe(true)}>{intl.get('EDIT')}</Button>)}
        </div>
      </div>
      {/* {previewOrEdit()} */}
      <div style={{display: `${poe? 'block' : 'none'}`}}>
        <EditCsv
          ref={props.ref}
          key={props.item.path}
          content={content}
          {...props.item}
          onChange={onChange}
          deleteflag={props.deleteflag}
          workSpaceHeight={workSpaceHeight}
        />
      </div>
      <div style={{display: `${poe? 'none' : 'block'}`}}>
        <Preview
          content={content}
        />
      </div>
    </div>
  )
}

export default observer(React.forwardRef((CsvMode)))
