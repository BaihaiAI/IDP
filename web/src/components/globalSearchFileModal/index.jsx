import React, {Fragment, useContext, useEffect, useRef, useState} from "react"
import { Input, Modal } from "antd"
import { LoadingOutlined, SearchOutlined } from "@ant-design/icons"
import "./index.less"
import { useSetState } from "ahooks"
import {
  getGlobalKeywordSearch,
  getHistoryOpenFile,
  saveGlobalKeywordSearch,
} from "@/utils/storage"
import { projectId } from "@/store/cookie"
import RecentOpenFileList from "./RecentOpenFileList"
import workspaceApi from "../../services/workspaceApi"
import NoSearchResult from "./NoSearchResult"
import RecentSearchList from "./RecentSearchList"
import SearchFileList from "./searchFileList"
import appContext  from "../../context"
import ModalFooter from "./ModalFooter"
import intl from "react-intl-universal"
import {findFileOrDirName} from "@/utils"

function GlobalSearchFileModal(props) {
  const { globalSearchVisible, setGlobalSearchVisible } = props
  const handleCancel = () => {
    setGlobalSearchVisible(false)
  }
  const appProps = useContext(appContext)

  const appComponentAddTabFile = (key, line, cellId) => {
    const openFile = {
      path: key,
      name: findFileOrDirName(key),
      suffix: key.slice(key.lastIndexOf(".") + 1),
      posLine: line,
      posCellId: cellId,
    }
    appProps.addFileAndHandleIpynb(openFile)
  }



  const [showLocalFileAndSearch, setShowLocalFileAndSearch] = useState(true)
  const [localShowState, setLocalShowState] = useSetState({
    recentOpenFile: getHistoryOpenFile()[projectId] || [],
    recentSearchKeyword: getGlobalKeywordSearch()[projectId] || [],
  })
  const [searchKeyword, setSearchKeyword] = useState("")
  const [searchLoading, setSearchLoading] = useState(false)
  const [searchFileList, setSearchFileList] = useState([])
  const searchRef = useRef()

  useEffect(() => {
    setLocalShowState({
      recentOpenFile: getHistoryOpenFile()[projectId] || [],
      recentSearchKeyword: getGlobalKeywordSearch()[projectId] || [],
    })
  }, [globalSearchVisible])

  useEffect(() => {
    if (!searchKeyword) {
      setShowLocalFileAndSearch(true)
    }
  }, [searchKeyword])

  const getFileList = (searchKeyword) => {
    if (!searchKeyword) {
      return
    }

    setSearchLoading(true)
    const globalKeywordSearch = getGlobalKeywordSearch()
    const searchList = globalKeywordSearch[projectId] || []

    if (searchList.indexOf(searchKeyword) === -1) {
      searchList.unshift(searchKeyword)
      globalKeywordSearch[projectId] = searchList
      saveGlobalKeywordSearch(globalKeywordSearch, projectId)
      setLocalShowState({
        recentSearchKeyword: searchList.slice(0, 5),
      })
    }
    workspaceApi
      .globalKeywordSearch(searchKeyword)
      .then((res) => {
        setSearchFileList(res.data)
        setSearchLoading(false)
        setShowLocalFileAndSearch(false)
      })
      .catch(() => {
        setSearchLoading(false)
      })
  }

  const handlerSearchItemClick = (keyword) => {
    setSearchKeyword(keyword)
    getFileList(keyword)
  }

  return (
    <Modal
      className={"global-search-modal"}
      width={888}
      closable={false}
      visible={globalSearchVisible}
      footer={
        !showLocalFileAndSearch && searchFileList.length === 0 ? null : (
          <ModalFooter
            showLocalFileAndSearch={showLocalFileAndSearch}
            searchFileList={searchFileList}
          />
        )
      }
      onCancel={handleCancel}
    >
      <Input
        ref={searchRef}
        allowClear
        onPressEnter={(event) => {
          if (searchKeyword) {
            getFileList(searchKeyword)
            searchRef.current.blur()
          }
        }}
        onChange={(event) => {
          const value = event.target.value
          setSearchKeyword(value)
        }}
        value={searchKeyword}
        style={{ marginBottom: 8 }}
        placeholder={`${intl.get("SEARCH_FOR_FILES")}...(${intl.get("PRESS_ENTER_TO_SEARCH")})`}
        prefix={
          searchLoading ? (
            <LoadingOutlined />
          ) : (
            <SearchOutlined style={{ color: "#8A8A8A" }} />
          )
        }
        size={"large"}
      />

      {showLocalFileAndSearch ? (
        <Fragment>
          <h3 className={"title"}>{intl.get("RECENTLY_OPENED_FILES")}</h3>
          <RecentOpenFileList
            setGlobalSearchVisible={setGlobalSearchVisible}
            appComponentAddTabFile={appComponentAddTabFile}
            list={localShowState.recentOpenFile}
          />
          <h3 className={"title"}>{intl.get("RECENT_SEARCHES")}</h3>
          <RecentSearchList
            handlerSearchItemClick={handlerSearchItemClick}
            list={localShowState.recentSearchKeyword}
          />
        </Fragment>
      ) : searchFileList.length ? (
        <SearchFileList
          appComponentAddTabFile={appComponentAddTabFile}
          setGlobalSearchVisible={setGlobalSearchVisible}
          searchKeyword={searchKeyword}
          list={searchFileList}
        />
      ) : (
        <NoSearchResult />
      )}
    </Modal>
  )
}

export default GlobalSearchFileModal
