import React from 'react';
import { contentLoad } from '@/services/apiConfig'
import './video.less'

interface Props {
  path: string
}

export const Video: React.FC<Props> = (props: Props) => {
  const { path } = props
  try {
    return (<div className="wrap">
      <video controls className="video">
        <source src={contentLoad({ path })} type="video/mp4"></source>
      </video>
    </div>)
  } catch (error) {
    return (<pre>
      {path}
    </pre>)
  }
}