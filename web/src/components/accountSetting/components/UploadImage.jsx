import React, { useEffect, useRef } from "react"
import ImgCrop from "antd-img-crop" //引入图片裁剪组件
import { Upload, Button, message, Radio } from "antd"
import userInfoApi from "@/services/userInfoApi"
import { useDispatch } from "react-redux"
import { handleAvatarUrlThunk } from "@/store/features/globalSlice"
import { defaultAvatarUrl } from "@/utils/storage"
import "./UploadImage.less"
import intl from "react-intl-universal"
import PropTypes from "prop-types"



UploadImage.propTypes = {
  updateUI:PropTypes.func,
}
function UploadImage(props) {
  const { updateUI } = props
  const fileRef = useRef()
  const dispatch = useDispatch()

  //根据官方属性定制化裁剪框大小固定的裁剪组件属性
  const ImgCropProps = {
    resize: true, //裁剪是否可以调整大小
    resizeAndDrag: true, //裁剪是否可以调整大小、可拖动
    modalTitle: intl.get("CHANGE_AVATAR"), //弹窗标题
    modalWidth: 600, //弹窗宽度
    rotate: true,
    modalOk: intl.get("OK"),
    modalCancel: intl.get("CANCEL"),
  }
  const customRequest = () => {
    userInfoApi.uploadAvatar(fileRef.current).then((res) => {
      updateUI()
      message.success(intl.get("CHANGED_AVATAR_SUCCESSFULLY"))
      dispatch(handleAvatarUrlThunk(`${defaultAvatarUrl}?temp=${Date.now()}`))
    })
  }
  const beforeUpload = (file) => {
    fileRef.current = file
    return true
  }

  return (
    <div className={'img-crop-container'}>
      <ImgCrop {...ImgCropProps}>
        <Upload
          customRequest={customRequest}
          beforeUpload={beforeUpload}
          accept="image/*"
          listType="picture"
          showUploadList={false}
        >
          <Button className={'antd-btn-active'} >{intl.get("CHANGE_AVATAR")}</Button>
        </Upload>
      </ImgCrop>
    </div>
  )
}
export default UploadImage
