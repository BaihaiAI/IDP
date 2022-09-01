// go run scripts/get_nightly_k8s_env_team_id.go
package main

// FIXME: "io/ioutil" has been deprecated since Go 1.16
import (
	"bytes"
	"crypto/md5"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"net/http/cookiejar"
	"os"
)

// TODO serde_json rename_all alternative in Go?
type RspWithoutData struct {
	Code    int    `json:"code,omitempty"`
	Message string `json:"message,omitempty"`
}

// type Rsp struct {
//     code int,
//     data struct {
//         user_id int
//     }
// }

func main() {
	base_url := "http://nightly.baihaiai.com"
	user_svc := "/0/api/v1"
	jar, _ := cookiejar.New(nil)
	client := &http.Client{
		Jar: jar,
	}

	// json account+password
	account := "w@go.dev"
	h := md5.New()
	io.WriteString(h, "1234qwer")
	password := fmt.Sprintf("%x", h.Sum(nil))
	// printf can format string
	req_body := make(map[string]string)
	req_body["account"] = account
	req_body["password"] = password

	req_body_json_str, err := json.Marshal(req_body)
	if err != nil {
		panic(err)
	}
	url := fmt.Sprintf("%s%s/user/account/register", base_url, user_svc)
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(req_body_json_str))
	if err != nil {
		panic(err)
	}
	req.Header.Set("Content-Type", "application/json")
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	// defer resp.Body.Close()
	resp_body, _ := ioutil.ReadAll(resp.Body)
	fmt.Printf("resp %d %s\n", resp.StatusCode, url)
	rsp_without_data := RspWithoutData{}
	if err := json.Unmarshal(resp_body, &rsp_without_data); err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", rsp_without_data)
	const REGISTER_CODE_ACCOUNT_EXIST = 41121006
	if rsp_without_data.Code != 200 && rsp_without_data.Code != REGISTER_CODE_ACCOUNT_EXIST {
		fmt.Fprintln(os.Stderr, "register new account fail")
		os.Exit(1)
		return
	}

	url = fmt.Sprintf("%s%s/user/account/login", base_url, user_svc)
	req, _ = http.NewRequest("POST", url, bytes.NewBuffer(req_body_json_str))
	req.Header.Set("Content-Type", "application/json")
	resp, _ = client.Do(req)
	resp_body, _ = ioutil.ReadAll(resp.Body)
	rsp := make(map[string]interface{})
	json.Unmarshal(resp_body, &rsp)
	fmt.Printf("\n%+v\n", rsp)
	// check code field type cast to Int? code, ok := rsp["code"].(Int)
	if resp.StatusCode != 200 && rsp["code"] != 200 {
		fmt.Fprintln(os.Stderr, "login fail")
		os.Exit(1)
	}

	var team_id string
	for _, cookie := range resp.Cookies() {
		if cookie.Name == "teamId" {
			team_id = cookie.Value
			break
		}
	}
	team_id_str := fmt.Sprintf("team_id=%s", team_id)
	old_team_id, err := ioutil.ReadFile("scripts/team_id.sh")
	if err != nil {
		old_team_id_str := string(old_team_id)
		if old_team_id_str == team_id_str {
			os.Exit(0)
		}
	}
	ioutil.WriteFile("scripts/team_id.sh", []byte(team_id_str), 0644)

	url = fmt.Sprintf("%s%s/project/getPage?teamId=%s&current=1&size=10", base_url, user_svc, team_id)
	req, _ = http.NewRequest("GET", url, nil)
	resp, err = client.Do(req)
	if err != nil {
		panic(err)
	}
	resp_body, err = ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	json.Unmarshal(resp_body, &rsp)
	project_id := rsp["data"].(map[string]interface{})["records"].([]interface{})[0].(map[string]interface{})["id"].(float64)
	fmt.Printf("\n%s\nteam_id=%s project_id=%f \n", url, team_id, project_id)
}
