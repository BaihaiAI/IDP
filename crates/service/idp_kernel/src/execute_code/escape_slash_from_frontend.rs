// Copyright 2022 BaihaiAI, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/**
e.g. these code would add extra slash when transfer from frontend to server
```text
a = 1 \
    + 2
```
*/
pub(crate) fn escape_slash_from_frontend(code_from_frontend: String) -> String {
    let code = code_from_frontend.into_bytes();
    let mut ret = Vec::with_capacity(code.len());

    let mut has_single_quote = false;
    let mut has_double_quote = false;

    // replace if not a string literal
    // .replace(r"\n", "\n").replace(r"\\", r"\")
    for byte in code {
        match byte {
            b'\'' => {
                has_single_quote = !has_single_quote;
            }
            b'"' => {
                has_double_quote = !has_double_quote;
            }
            b'\\' => {
                // replace(r"\\", r"\")
                if !(has_single_quote || has_double_quote) {
                    if let Some(last) = ret.last() {
                        if b'\\'.eq(last) {
                            continue;
                        }
                    }
                }
            }
            b'n' => {
                if !(has_single_quote || has_double_quote) {
                    // replace(r"\n", "\n")
                    if let Some(last) = ret.last_mut() {
                        if b'\\'.eq(last) {
                            *last = b'\n';
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        ret.push(byte);
    }
    unsafe { String::from_utf8_unchecked(ret) }
}

#[test]
fn test_escape_slash_from_frontend() {
    const CODE_FROM_FRONTEND: &str = r#"def getHistoryTradeInfo(stockCode):\n    download_url = "http://quotes.money.163.com/service/chddata.html?code=0" + stockCode + "&start=" + getStockStartDate(stockCode) + "&end=" + time.strftime("%Y%m%d") + "&fields=TCLOSE;HIGH;LOW;TOPEN;LCLOSE;CHG;PCHG;TURNOVER;VOTURNOVER;VATURNOVER;TCAP;MCAP"\n    data = requests.get(download_url)\n    #print(data.apparent_encoding)\n    print(type(data.content.decode('GB2312')))\n    with open( stockCode + '.csv', 'wb') as f:\n    #f.write(data.content.decode('GB2312'))\n      f.write(data.content.decode('GB2312').encode('utf-8'))\ngetHistoryTradeInfo(str(600197))"#;
    const CODE_ORIGIN: &str = r#"def getHistoryTradeInfo(stockCode):
    download_url = "http://quotes.money.163.com/service/chddata.html?code=0" + stockCode + "&start=" + getStockStartDate(stockCode) + "&end=" + time.strftime("%Y%m%d") + "&fields=TCLOSE;HIGH;LOW;TOPEN;LCLOSE;CHG;PCHG;TURNOVER;VOTURNOVER;VATURNOVER;TCAP;MCAP"
    data = requests.get(download_url)
    #print(data.apparent_encoding)
    print(type(data.content.decode('GB2312')))
    with open( stockCode + '.csv', 'wb') as f:
    #f.write(data.content.decode('GB2312'))
      f.write(data.content.decode('GB2312').encode('utf-8'))
getHistoryTradeInfo(str(600197))"#;
    assert_eq!(
        escape_slash_from_frontend(CODE_FROM_FRONTEND.to_string()),
        CODE_ORIGIN
    );

    assert_eq!(
        escape_slash_from_frontend(r"print('1\n')".to_string()),
        r"print('1\n')"
    );
}
