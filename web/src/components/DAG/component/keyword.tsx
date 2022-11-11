import React from 'react'
import './keyword.less'

interface Props {
  raw: string
  keyword: string
  className?: string
}

export const Keyword: React.FC<Props> = (props) => {
  const { raw, keyword, className } = props
  if (keyword) {
    const regex = new RegExp(keyword, 'ig')
    const arr = raw.split(regex)
    return (
      <span
        className={
          className ? "keywordWrapper " + className : "keywordWrapper"
        }
      >
        {arr.map((section, index) =>
          index !== arr.length - 1 ? (
            <span key={section + index}>
              {section}
              <strong>{keyword}</strong>
            </span>
          ) : (
            section
          ),
        )}
      </span>
    )
  }
  return null
}
