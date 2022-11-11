import react, {useEffect, useState, useRef} from 'react';
import { useSelector, useDispatch } from "react-redux"
import {
  selectOperatorDecision,
  changeOperatorDecision,
  selectOperatorKey,
  } from '@/store/features/globalSlice'
import {
  InsertCodeSnippet
} from '@/store/features/notebookSlice'
import {
  selectActivePath
} from '@/store/features/filesTabSlice'

import PublicCode from './PublicCode';
import operatorApi from '@/services/operatorApi';
import { store } from '@/store'
import MarkdownEditor from "@uiw/react-markdown-editor"


import { Button, Radio, Tooltip } from 'antd';
import './decisionClassification.less'




function DecisionClassification(props){
  const visible = useSelector(selectOperatorDecision)
  const dispatch = useDispatch()
  const [codeFragment, setCodeFragment] = useState([])
  const [recover, setRecover] = useState([])
  const [lineBreak, setLineBreak] = useState(0)
  const [width, setWidth] = useState(0)
  const [description, setDescription] = useState({})
  const key = useSelector(selectOperatorKey)
  const path = useSelector(selectActivePath)
  const markdownCellRef = useRef()

  useEffect(() => {
    if(key !== "") getCode()
  }, [key])
  useEffect(() => {
    if(visible){
      const { width } = document.getElementsByClassName("decisiontree")[0].getBoundingClientRect()
      setWidth(width)
    }
  }, [visible])
  async function getCode(){
    let arr=[];
    let data = await operatorApi.getOperatorCode({key})
    .then(res => {
      const { description } = res.data;
      setDescription(description)
      return res.data.cells
    })
    data.forEach(prop => {
      arr.push(prop.join(''))
    })
    setCodeFragment(arr);
    setRecover(data)

  }



  function getPreAndNextIndex() {
    let currentIndex = 0
    let preIndex = 0
    let nextIndex = 0

    for(let val of Object.values(store.getState().notebook.notebookList)){
      if(val.path === path){
        if(val.cells.length === 0){
          preIndex = 1
          currentIndex = 0
        }else{
          let key;
          for(let prop of Object.keys(val.cellProps)){
            if(val.cellProps[prop]["focus"]){
              key = prop
            }
          }
          for(let i=0; i < val.cells.length; i++){
            if(val.cells[i].metadata.id === key){
              if(val.cells[i] === val.cells[val.cells.length - 1]){
                preIndex = val.cells[i].metadata.index
                nextIndex = ((val.cells[i].metadata.index + 1) * 2) - val.cells[i].metadata.index
                currentIndex = i;
              }else{
                preIndex = val.cells[i].metadata.index
                nextIndex = val.cells[i+1].metadata.index
                currentIndex = i
              }
            }
          }
        }
        break
      }
    }
    return {preIndex, nextIndex, currentIndex}
  }

  function insertCodes() {
    const {preIndex, nextIndex, currentIndex} = getPreAndNextIndex()
    let cells = [], starindex = preIndex;
    for (let i = 0; i < recover.length; i++) {
      let cell = {
        cell_type: 'code',
        metadata: {
          // index: preIndex + ((preIndex + nextIndex) / (100 - i))
          index: (starindex + nextIndex) / 2
        },
        source: recover[i],
        path
      }
      cells.push(cell)
      starindex = (starindex + nextIndex) / 2;
    }
    dispatch(InsertCodeSnippet({path, cells, currentIndex}))
  }

  const handleBlur = () => {
    console.log(markdownCellRef.current.editor)
  }




  return (
    visible? (
      <div className='decisiontree-warp'>
        <div className='decisiontree'>
          <div className='dec-top' style ={{width: `${width - 32}px`}}>
            <p className='p1'>{description.title}</p>
            <p className='p3' onClick={() => visible? dispatch(changeOperatorDecision(false)) : null}>
              <Button>收起＞</Button>
            </p>
            <Tooltip placement="top" title={"在Notebook中插入该代码片段"}>
              <p className='p2'><Button
                onClick={() => insertCodes()}
              >使用</Button></p>
            </Tooltip>
          </div>
          <div className='dec-content' onClick={(e) => e.stopPropagation()}>
            {/* {description.body} */}
            <MarkdownEditor
              value={description.body}
              visible={true} //开启预览
              options={{ lineNumbers: false }}
              ref={markdownCellRef}
              onBlur={handleBlur}
            />
          </div>
          <div className='dec-linebreak'>
            <Radio.Group defaultValue={lineBreak} value={lineBreak}>
              <Radio
                value={1}
                onClick={() => {
                  if(lineBreak)setLineBreak(0);
                  else setLineBreak(1);
                }}
              >自动换行</Radio>
            </Radio.Group>
          </div>
          <div className='dec-code'>
            {codeFragment.map((item,key) => (
              <div key={key}>
                <PublicCode
                  codeFragment={item}
                  lineBreak={lineBreak}
                  bindkey={key}/>
              </div>
            ))}
            {/* <PublicCode
              codeFragment={codeFragment}
              lineBreak={lineBreak}/> */}
          </div>
        </div>
      </div>) : null

  )
}
export default DecisionClassification
