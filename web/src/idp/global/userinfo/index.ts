import * as React from 'react';
import { Component } from 'react';
import { action, observable, toJS } from "mobx"
import { userInfoApi } from "../../../services"
class UserInfoGlobal {

    @observable userInfo: any;

    @action updateUserInfo(userInfo: any) {
        this.userInfo = userInfo;
    }

    async getUserInfo() {
        const result: any = await userInfoApi.getUserInfo();
        if (result.code == 200) {
            this.userInfo = result.data
        }
        return result.data;
    }
}

export default new UserInfoGlobal();
