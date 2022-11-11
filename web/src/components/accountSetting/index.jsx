import React, { useEffect, useState } from "react"
import { Button, Form, Input, Modal, Radio, message } from "antd"
import "./index.less"
import userInfoApi from "@/services/userInfoApi"
import resetPassword from "@/services/resetPassword"
import Icons from "@/components/Icons/TeamIcons"
import { useSetState } from "ahooks"
import md5 from "md5"
import UploadImage from "./components/UploadImage"
import { useDebounceFn } from "ahooks"
import intl from "react-intl-universal"
import PropTypes from "prop-types"

const { useForm } = Form

AccountSetting.propTypes = {
  updateAvatarUrl: PropTypes.func,
  avatarUrl: PropTypes.string,
}


function AccountSetting(props) {


  const { avatarUrl } = props

  const updateUI = () => {
    if (typeof props.updateAvatarUrl === 'function') {
      props.updateAvatarUrl()
    }
  }


  const [modifyUserNameANTD] = useForm()
  const [passwordForm] = useForm()
  const [userInfo, setUserInfo] = useState({
    id: "",
    username: "",
    email: "",
    phone: "",
  })

  let [btnStyle, setBtnStyle] = useState(60)
  let [fired, setFired] = useState(false)

  const [yVisible, setYVisible] = useState(false)

  const [passwordInfo, setPasswordInfo] = useSetState({
    visible: false,
    loading: false,
  })

  useEffect(() => {
    getUserInfoAgain()
  }, [])

  const getUserInfoAgain = () => {
    userInfoApi.getUserInfo().then((res) => {
      if (res.data) {
        const { id, email, phone, username } = res.data
        setUserInfo({
          id,
          username,
          email,
          phone,
        })
      }
    })
  }

  const passwordModalCancel = () => {
    setPasswordInfo({
      visible: false,
    })
  }
  const confirmModify = () => {
    modifyUserNameANTD.validateFields().then((value) => {
      const { name } = value
      if (name) {
        const data = {
          username: name,
        }
        userInfoApi
          .updateUsername(data)
          .then((res) => {
            console.log(res)
            cancelModify()
            getUserInfoAgain()
          })
          .catch((err) => {
            console.log(err)
          })
      }
    })
  }
  const cancelModify = () => {
    setYVisible((yVisible) => !yVisible)
  }

  const clickEnterReset = () => {
    passwordForm.validateFields().then((value) => {
      console.log(value)
      const accountVal =
        userInfo.email !== null ? userInfo.email : userInfo.phone
      const data = {
        account: accountVal,
        password: md5(value.confirmPassword),
        activeCode: value.verificationCode,
      }
      resetPassword
        .resetPasswordPlase(data)
        .then((res) => {
          console.log(res)
          setPasswordInfo({
            visible: false,
          })
          message.success(`${intl.get("PASSWORD_HAS_BEEN_UPDATED")}！`)
          passwordForm.resetFields()
        })
        .catch((err) => console.log(err))
    })
  }
  const { run: getVerificationCode } = useDebounceFn(
    () => {
      console.log("------00000-------")
      const accountVal =
        userInfo.email !== null ? userInfo.email : userInfo.phone
      const accountKey = userInfo.email !== null ? "email" : "phone"
      const data = {
        [accountKey]: accountVal,
      }
      setFired(true)
      resetPassword
        .getVerificationCode(data)
        .then((res) => {
          console.log(res)
          countdown()
        })
        .catch((err) => {
          setBtnStyle(60)
          setFired(false)
        })
    },
    {
      wait: 1000,
    }
  )

  const countdown = () => {
    let timer
    if (btnStyle === 0 || btnStyle < 0) {
      setBtnStyle(60)
      setFired(false)
      clearTimeout(timer)
    } else {
      setBtnStyle(btnStyle--)
      console.log(btnStyle)
      timer = setTimeout(() => {
        countdown()
      }, 1000)
    }
  }
  const modifyUserName = () => {
    return (
      <Modal
        wrapClassName={"add-or-update-project-modal"}
        title={
          <span style={{ fontWeight: "bold" }}>
            {intl.get("MODIFY_USERNAME")}
          </span>
        }
        visible={yVisible}
        okButtonProps={{
          size: "large",
        }}
        cancelButtonProps={{
          size: "large",
        }}
        onOk={confirmModify}
        onCancel={cancelModify}
      >
        <Form form={modifyUserNameANTD}>
          <Form.Item
            name={"name"}
            rules={[
              {
                required: true,
                message: intl.get("USERNAME_CAN_NOT_BE_EMPTY"),
              },
            ]}
          >
            <Input
              placeholder={intl.get("PLEASE_ENTER_A_NEW_USERNAME")}
              size={"large"}
            />
          </Form.Item>
        </Form>
      </Modal>
    )
  }

  return (
    <div className="setting">
      <div className="account">
        <div className="change-avatar">
          <div className="portrait">
            <img src={avatarUrl} alt="" />
          </div>
          <UploadImage updateUI={updateUI} />
        </div>
        <div className="information">
          <ul className="info-ul">
            <li className="info-png" style={{ background: `url(${require('@/assets/image/account.svg').default}) 0 no-repeat` }}>
              {intl.get("USER_NAME")}：
              <span>
                {userInfo.username}
                <p
                  className="info-png edit-name"
                  style={{ background: `url(${require('@/assets/image/editblack.svg').default}) 0 no-repeat` }}
                  onClick={() => {
                    setYVisible((yVisible) => !yVisible)
                  }}
                ></p>
              </span>
            </li>
            <li className="info-png" style={{ background: `url(${require('@/assets/image/email.svg').default}) 0 no-repeat` }}>邮&nbsp;&nbsp;&nbsp;箱：{userInfo.email}</li>
            <li className="info-png" style={{ background: `url(${require('@/assets/image/phone.svg').default}) 0 no-repeat` }}>手&nbsp;&nbsp;&nbsp;机：{userInfo.phone}</li>
          </ul>
          <Radio.Group>
            <Radio.Button
              style={{
                borderColor: '#1890ff',
                color: '#1890ff'
              }}
              onClick={() => {
                setPasswordInfo({
                  visible: true,
                })
              }}
            >
              {intl.get("CHANGE_PASSWORD")}
            </Radio.Button>
          </Radio.Group>
        </div>
      </div>
      {/*修改密码暂时注释掉*/}
      <Modal
        wrapClassName={"change-password-modal"}
        title={
          <span style={{ fontWeight: "bold" }}>
            {intl.get("CHANGE_PASSWORD")}
          </span>
        }
        visible={passwordInfo.visible}
        okButtonProps={{ size: "large" }}
        confirmLoading={passwordInfo.loading}
        onCancel={passwordModalCancel}
        cancelButtonProps={{
          size: "large",
        }}
        footer={null}
      >
        <Form form={passwordForm}>
          <Form.Item
            name={"password"}
            rules={[
              {
                required: true,
                message: intl.get("PLEASE_ENTER_A_NEW_PASSWORD"),
              },
              {
                pattern: new RegExp("(?=.*[0-9])(?=.*[a-zA-Z]).{8,30}"),
                message: intl.get(
                  "PASSWORD_MUST_BE_NO_LESS_THAN_8_CHARACTERS AND_MUST_CONTAIN_NUMBERS_AND_LETTERS"
                ),
              },
            ]}

          // extra="密码至少六位,需要包含英文、数字与符号中的两位"
          >
            <Input.Password
              placeholder={intl.get("PLEASE_ENTER_A_NEW_PASSWORD")}
              iconRender={(visible) =>
                visible ? (
                  <Icons.showPwdIcon style={{ fontSize: 20 }} />
                ) : (
                  <Icons.hidePwdIcon style={{ fontSize: 20 }} />
                )
              }
              style={{ height: "40px" }}
            />
          </Form.Item>

          <Form.Item
            name="confirmPassword"
            dependencies={["password"]}
            rules={[
              {
                required: true,
                message: intl.get("PLEASE_ENTER_A_NEW_PASSWORD"),
              },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue("password") === value) {
                    return Promise.resolve()
                  }
                  return Promise.reject(
                    new Error(
                      `${intl.get("THE_TWO_PASSWORDS_DO_NOT_MATCH")},${intl.get(
                        "PLEASE_ENTER_AGAIN"
                      )}`
                    )
                  )
                },
              }),
            ]}
          >
            <Input.Password
              placeholder={intl.get("PLEASE_ENTER_THE_PASSWORD_AGAIN")}
              iconRender={(visible) =>
                visible ? (
                  <Icons.showPwdIcon style={{ fontSize: 20 }} />
                ) : (
                  <Icons.hidePwdIcon style={{ fontSize: 20 }} />
                )
              }
              style={{ height: "40px" }}
            />
          </Form.Item>
          <Form.Item
            name="verificationCode"
            rules={[
              { required: true, message: intl.get("PLEASE_ENTER_VERIFICATION_CODE"), whitespace: true },
              { len: 6, message: intl.get("PLEASE_ENTER_A_6_DIGIT_VERIFICATION_CODE") },
            ]}
            style={{
              width: "328px",
              display: "inline-block",
              marginTop: "1px",
            }}
          >
            <Input placeholder={intl.get("PLEASE_ENTER_A_6_DIGIT_VERIFICATION_CODE")} className="sendinput" />
          </Form.Item>
          <Button
            className="sendvic"
            type="primary"
            onClick={() => getVerificationCode()}
            disabled={fired}
          >
            {btnStyle !== 60 ? `${btnStyle}秒后发送` : "发送验证码"}
          </Button>
          <Form.Item name="verificationCode">
            <Button
              type="primary"
              style={{ height: "40px", width: "450px", borderRadius: "2px" }}
              onClick={() => clickEnterReset()}
            >
              {intl.get("OK")}
            </Button>
          </Form.Item>
        </Form>
      </Modal>
      {modifyUserName()}
    </div>
  )
}

export default AccountSetting
