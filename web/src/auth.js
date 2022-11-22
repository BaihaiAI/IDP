import React from 'react';
import { analysisUrl } from '../../config/auth';
import cookie from 'react-cookies';
import axios from 'axios';

class AuthApp extends React.Component {

    constructor(props) {
        super(props);
        this.authApi();
    }
    state = {}
    authApi = async () => {
        axios.defaults.headers['Content-Type'] = 'application/json; charset=utf-8';
        const code = cookie.load("code");
        const scope = cookie.load("scope");
        const state = cookie.load("state");
        if (code && scope && state) {
            try {
                const result = await axios.get(`/0/api/v1/user/ory/callback?code=${code}&scope=${scope}&state=${state}&redirect_uri=${window.location.origin}/`);
                if (!result.data.code === 200) {
                    setAuthFlg(false);
                    analysisUrl();
                }
            } catch (error) {
                setAuthFlg(false);
                analysisUrl();
            }
        }
    }

    render() {
        return (
            <>
                {this.props.children}
            </>
        );
    }
}

export default AuthApp;