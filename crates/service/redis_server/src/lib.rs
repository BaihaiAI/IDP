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

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufReader;
use std::sync::Mutex;

use err::ErrorTrace;
use resp::Decoder;
use resp::Value;
use tracing::error;
use tracing::info;

struct AppCtx {
    hashes: HashMap<String, HashMap<String, String>>,
    lists: HashMap<String, VecDeque<String>>,
}

type Ctx = Mutex<AppCtx>;

/**
https://redis.io/docs/reference/protocol-spec/#resp-simple-strings

## redis line prefix

- `*3`: array length is 3
- `+`: simple string(ascii string), only used in server simple response?
- `$5`: bulk string length is 5
- `-`: sever error response prefix
- `:`: integer prefix

## ping req

```text
*1
$4
PING

```
*/
pub fn main() {
    logger::init_logger();
    let ctx = Mutex::new(AppCtx {
        hashes: HashMap::new(),
        lists: HashMap::new(),
    });
    let redis_port = business::idp_redis_port();
    if redis_port == 6379 {
        // use system's redis
        return;
    }
    let redis_addr = std::net::SocketAddr::from(([0, 0, 0, 0], redis_port));
    let listener = std::net::TcpListener::bind(redis_addr).unwrap();
    std::thread::scope(|s| {
        for stream_res in listener.incoming() {
            let stream = match stream_res {
                Ok(stream) => stream,
                Err(err) => {
                    error!("stream_res {err}");
                    continue;
                }
            };
            s.spawn(|| {
                info!("before handle_connection");
                if let Err(err) = handle_connection(stream, &ctx) {
                    error!("handle_connection {err}");
                }
                info!("after  handle_connection");
            });
        }
    });
}

// empty bulk string
const REDIS_NIL_RESP: &str = "$-1\r\n";
// const REDIS_EMPTY_ARR_RESP: &str = "$-1\r\n";

#[test]
fn test_parse_redis_req() {
    let input = b"*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n";
    let mut decoder = Decoder::new(BufReader::new(input.as_slice()));
    assert!(!decoder.decode().unwrap().is_error());
}

fn handle_connection(stream: std::net::TcpStream, ctx: &Ctx) -> Result<(), ErrorTrace> {
    let mut write_stream = stream.try_clone()?;
    let mut decoder = Decoder::new(BufReader::new(stream));
    loop {
        // if stream is non-blocking IO, should sleep to prevent CPU busy-wait
        std::thread::sleep(std::time::Duration::from_millis(50));
        let mut req_args = {
            // try reading from tcp stream
            let req = match decoder.decode() {
                Ok(req) => req,
                Err(err) => {
                    if matches!(err.kind(), std::io::ErrorKind::UnexpectedEof) {
                        continue;
                    }
                    return Err(err.into());
                }
            };

            // must be an array
            let req = match req {
                Value::Array(req) => Ok(req),
                _ => Err(ErrorTrace::new("invalid redis req")),
            }?;

            // must be an array of bulk string
            let mut req_str = Vec::with_capacity(req.len());
            for s in req {
                match s {
                    Value::Bulk(s) => req_str.push(s),
                    _ => return Err(ErrorTrace::new("invalid redis req")),
                }
            }

            // must not be empty
            if req_str.is_empty() {
                return Err(ErrorTrace::new("invalid redis req"));
            }
            req_str
        };
        let req_cmd = req_args.remove(0);
        let rsp = match req_cmd.to_lowercase().as_str() {
            "quit" => {
                break;
            }
            "ping" => "+PONG\r\n".to_string().into_bytes(),
            "set" => "+OK\r\n".to_string().into_bytes(),
            "expire" => {
                // current we doesn't support expire
                ":1\r\n".to_string().into_bytes()
            }
            "del" => {
                let mut ctx = ctx.lock().unwrap();
                let hvals_remove = ctx.hashes.remove(&req_args[0]);
                if hvals_remove.is_some() {
                    ":1\r\n".to_string().into_bytes()
                } else {
                    ":0\r\n".to_string().into_bytes()
                }
            }
            "exists" => {
                let mut req_args = req_args.into_iter();
                let redis_key = req_args.next().unwrap();
                let ctx = ctx.lock().unwrap();
                if ctx.hashes.contains_key(&redis_key) {
                    ":1\r\n".to_string().into_bytes()
                } else {
                    ":0\r\n".to_string().into_bytes()
                }
            }
            "evalsha" => {
                // eval redis script is unsupported
                ":1\r\n".to_string().into_bytes()
            }
            // command of hash
            "hvals" => {
                let ctx = ctx.lock().unwrap();
                let hash_values = if let Some(hash) = ctx.hashes.get(&req_args[0]) {
                    hash.values().cloned().collect()
                } else {
                    Vec::new()
                };
                encode_redis_arr(hash_values)
            }
            "hget" => {
                let mut req_args = req_args.into_iter();
                let redis_key = req_args.next().unwrap();
                let hash_key = req_args.next().unwrap();
                let hashes = &ctx.lock().unwrap().hashes;
                match hashes.get(&redis_key) {
                    Some(hash) => match hash.get(&hash_key) {
                        Some(val) => encode_bulk_str(val),
                        None => {
                            eprintln!("hget redis_key not found: {redis_key} {hash_key}");
                            REDIS_NIL_RESP.to_string().into_bytes()
                        }
                    },
                    None => {
                        eprintln!("hget redis_key not found: {redis_key}");
                        REDIS_NIL_RESP.to_string().into_bytes()
                    }
                }
            }
            "hdel" => {
                let mut ctx = ctx.lock().unwrap();
                let mut req_args = req_args.into_iter();
                let redis_key = req_args.next().unwrap();
                let hash_key = req_args.next().unwrap();
                let hash = ctx.hashes.entry(redis_key).or_default();
                let remove = hash.remove(&hash_key);
                if remove.is_some() {
                    ":1\r\n".to_string().into_bytes()
                } else {
                    ":0\r\n".to_string().into_bytes()
                }
            }
            "hset" => {
                let mut ctx = ctx.lock().unwrap();
                let mut req_args = req_args.into_iter();
                let redis_key = req_args.next().unwrap();
                let hash_key = req_args.next().unwrap();
                let hash_val = req_args.next().unwrap();
                let hash = ctx.hashes.entry(redis_key).or_default();
                // Integer reply: The number of fields that were added.
                hash.insert(hash_key, hash_val);
                format!(":{}\r\n", hash.len()).into_bytes()
            }
            "hmset" => {
                let mut ctx = ctx.lock().unwrap();
                let redis_key = req_args.remove(0);
                let hash = ctx.hashes.entry(redis_key).or_default();
                let mut req_args = req_args.into_iter();
                while let Some(hash_key) = req_args.next() {
                    let hash_val = req_args.next().unwrap();
                    hash.insert(hash_key.clone(), hash_val);
                }
                format!(":{}\r\n", hash.len()).into_bytes()
            }
            // command of list
            "lrange" => {
                let ctx = ctx.lock().unwrap();
                let list_values = if let Some(list) = ctx.lists.get(&req_args[0]) {
                    list.iter().cloned().collect()
                } else {
                    Vec::new()
                };
                encode_redis_arr(list_values)
            }
            "lpush" => {
                let mut ctx = ctx.lock().unwrap();
                let redis_key = req_args.remove(0);
                let list_value = req_args.remove(0);
                let list = ctx.lists.entry(redis_key).or_default();
                list.push_front(list_value);
                format!(":{}\r\n", list.len()).into_bytes()
            }
            "ltrim" => {
                // IDP only use ltrim(key, 0, 99)
                let mut ctx = ctx.lock().unwrap();
                let redis_key = req_args.remove(0);
                let list = ctx.lists.entry(redis_key).or_default();
                if list.len() > 100 {
                    list.pop_back();
                }
                "+OK\r\n".to_string().into_bytes()
            }
            cmd => format!("-unknown command {cmd}\r\n").into_bytes(),
        };
        std::io::Write::write_all(&mut write_stream, &rsp)?;
    }

    Ok(())
}

fn encode_redis_arr(arr: Vec<String>) -> Vec<u8> {
    resp::encode(&Value::Array(arr.into_iter().map(Value::Bulk).collect()))
}

fn encode_bulk_str(s: &str) -> Vec<u8> {
    resp::encode(&Value::Bulk(s.to_string()))
}

#[cfg(test)]
mod redis_server_test {
    use std::net::TcpStream;

    use super::*;

    struct TestClient {
        write: TcpStream,
        decode: Decoder<TcpStream>,
    }

    impl TestClient {
        fn new(stream: TcpStream) -> Self {
            Self {
                write: stream.try_clone().unwrap(),
                decode: Decoder::new(BufReader::new(stream)),
            }
        }

        fn send(&mut self, cmd: Vec<String>) -> std::io::Result<()> {
            let bytes = encode_redis_arr(cmd);
            std::io::Write::write_all(&mut self.write, &bytes)
        }

        fn receive(&mut self) -> Value {
            self.decode.decode().unwrap()
        }
    }

    fn get_test_connection() -> TcpStream {
        let redis_port = business::idp_redis_port();
        let redis_addr = std::net::SocketAddr::from(([0, 0, 0, 0], redis_port));
        TcpStream::connect(redis_addr).unwrap()
    }

    /// these tests cannot be executed in parallel as they do occupy the same port
    #[test]
    fn redis_server_test() {
        let redis_port = std::net::TcpListener::bind(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::LOCALHOST,
            0,
        ))
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
        std::env::set_var("IDP_REDIS_PORT", redis_port.to_string());
        std::thread::spawn(main);
        // make sure server bind success
        let mut retry = 0;
        loop {
            if retry == 100 {
                panic!("TCP bind timeout");
            }
            if std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, redis_port)).is_ok() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
            retry += 1;
        }
        let mut client = TestClient::new(get_test_connection());
        let test_data_1 = "LARGE_STRING-".repeat(10000);
        let test_data_2 = "large_string-".repeat(10000);
        client
            .send(vec![
                "lpush".to_string(),
                "test_data".to_string(),
                test_data_1.clone(),
            ])
            .unwrap();
        client
            .send(vec![
                "lpush".to_string(),
                "test_data".to_string(),
                test_data_2.clone(),
            ])
            .unwrap();
        assert_eq!(client.receive(), Value::Integer(1));
        assert_eq!(client.receive(), Value::Integer(2));
        client
            .send(vec!["lrange".to_string(), "test_data".to_string()])
            .unwrap();
        assert_eq!(
            client.receive(),
            Value::Array(vec![Value::Bulk(test_data_2), Value::Bulk(test_data_1),])
        );
        client.send(vec!["quit".to_string()]).unwrap();
    }
}
