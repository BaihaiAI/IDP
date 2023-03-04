import React, { useState, useEffect } from 'react';
import { Button } from 'antd';
import intl from 'react-intl-universal';
import ImageEditor from '@toast-ui/react-image-editor';
import theme from './theme';
import { contentLoad } from '@/services/apiConfig'
import './image.less'

interface Props {
  path: string
  workSpaceHeight: number
}

export const Image: React.FC<Props> = (props: Props) => {
  const { path, workSpaceHeight } = props
  const [poe, setPoe] = useState(false)
  const imageSrc = contentLoad({ path })
  const uiSizeHeight = document.body.clientHeight ? document.body.clientHeight - 160 : 600

  useEffect(() => {
    const fileInputs = document.getElementsByClassName('tui-image-editor-load-btn');
    for (const dom of fileInputs) {
      console.log(dom.previousSibling.nodeValue)
      if ('\n                    Load\n                    ' === dom.previousSibling.nodeValue) {
        dom.parentElement.remove()
      }
    }

    return () => {

    }
  }, [])

  try {
    return (
      <div className='control-bar'>
        <div className='control'>
          <div className='control-box'>
            {poe
              ? (<Button key="preview" type="link" onClick={() => setPoe(false)}>{intl.get('PREVIEW')}</Button>)
              : (<Button key="edit" type="link" onClick={() => setPoe(true)}>{intl.get('EDIT')}</Button>)}
          </div>
        </div>
        <div style={{ display: `${poe ? 'block' : 'none'}`, overflow: 'auto'}}>
          <ImageEditor
            includeUI={{
              loadImage: {
                path: imageSrc,
                name: path,
              },
              theme: theme,
              menu: ['resize',
                'crop',
                'flip',
                'rotate',
                'draw',
                'shape',
                'icon',
                'text',
                'mask',
                'filter'],
              // initMenu: 'filter',
              uiSize: {
                // width: '100%',
                height: `${uiSizeHeight}px`,
              },
              menuBarPosition: 'bottom',
            }}
            // cssMaxHeight={uiSizeHeight}
            // cssMaxWidth={700}
            selectionStyle={{
              cornerSize: 20,
              rotatingPointOffset: 70,
            }}
            usageStatistics={true}
          />
        </div>
        <div style={{ display: `${poe ? 'none' : 'block'}` }} className="wrap">
          <img src={imageSrc} className="image" />
        </div>
      </div>
    )
  } catch (error) {
    return (<pre>
      {path}
    </pre>)
  }
}