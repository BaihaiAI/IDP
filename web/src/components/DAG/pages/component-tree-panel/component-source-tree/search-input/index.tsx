import React, { useState } from "react"
import { Input } from "antd"
import { useDebounceFn } from "ahooks"
// import { useModel } from 'umi'
import styles from "./index.module.less"
import PubSub from "pubsub-js"

const { Search } = Input

export const SearchInput = () => {
  const [value, setValue] = useState<string>("")
  // const { search, setKeyword } = useModel('guide-algo-component')

  const { run: onDebouncedSearch } = useDebounceFn(
    (v: string) => {
      if (v) {
        PubSub.publish("category-tree-search-value-change", v)
      }
    },
    { wait: 500 }
  )

  return (
    <div className={styles.searchInput}>
      <Search
        className={styles.input}
        placeholder="请输入名称或描述"
        value={value}
        allowClear={true}
        onChange={(e) => {
          const v = e.target.value
          setValue(v)
          if (!v) {
            PubSub.publish("category-tree-search-value-change", v)
          }
        }}
        onSearch={onDebouncedSearch}
      />
    </div>
  )
}
