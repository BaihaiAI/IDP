import React from "react"
import "./NoSearchResult.less"
import exclamationMark from "../../assets/感叹号.svg"
import intl from "react-intl-universal"

function NoSearchResult(props) {
  return (
    <div className={"show-no-search-result-wrapper"}>
      <img src={exclamationMark} alt="" />
      <h2>{intl.get("NO_MATCHING_RESULTS_WERE_FOUND")}</h2>
      <h3>{intl.get("PLEASE_TRY_SEARCHING_FOR_OTHER_KEYWORDS")}</h3>
    </div>
  )
}

export default NoSearchResult
