import {defaultAvatarUrl, validateImageUrl} from "../storage"
import {useState} from "react"
import {useMemoizedFn} from "ahooks"


export const useAvatarUrl = ()=>{
  const [avatarUrl, setAvatarUrl] = useState('/static/custom.png')

  const getAvatar = useMemoizedFn(async () => {
    const avatarUrl = `${defaultAvatarUrl}?temp=${Date.now()}`
    const isValid = await validateImageUrl(avatarUrl)
    if (isValid) {
      setAvatarUrl(avatarUrl)
    }
  })
  const updateAvatarUrl = ()=>{
    setAvatarUrl(`${defaultAvatarUrl}?temp=${Date.now()}`)
  }


  return {
    avatarUrl,
    getAvatar,
    updateAvatarUrl
  }
}
