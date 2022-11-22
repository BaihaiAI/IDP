const express = require("express");
const expressWs = require("express-ws");
const pty = require("node-pty");
const os = require("os");
const execSync = require('child_process').execSync

const isWin32 = os.platform() === "win32";
const shell = isWin32 ? "powershell.exe" : "bash";
const login = isWin32 ? [] : ["--login"];
const homePath = isWin32 ? "$env:HOMEPATH" : "$HOME";
const app = express();
expressWs(app);
const termMap = new Map();

const args = process.argv.slice(2);

// 初始化terminal
function nodeEnvBind(req) {
  console.log(`${new Date()} ${JSON.stringify(req.query)}`);
  const cols = req.query.cols ? parseInt(req.query.cols) : 80;
  const rows = req.query.rows ? parseInt(req.query.rows) : 30;
  const teamId = req.query.teamId;
  const projectId = req.query.projectId;

  //绑定当前系统 node 环境
  let term = pty.spawn(shell, login, {
    name: "xterm-color",
    cols: cols,
    rows: rows,
    cwd: process.env.HOME,
    env: process.env,
  });
  term.onExit((code) => {
    console.log(`${new Date()} project: ${projectId} term exit: ${JSON.stringify(code)}`);
    term.kill();
    termMap.delete(term.pid);
    if (127 === code.exitCode) {
      term = nodeEnvBind(req);
      termMap.set(term.pid, term);
    }
  })
  termMap.set(term.pid, term); 
  const cmd = `cd ${homePath}/.idp/store/1/projects/1/notebooks\n clear\n`;
  term.write(cmd);
  console.log(term);
  return term;
}

// 校验终端是否初始化成功
function checkTerm(term, req) {
  try {
    const checkPid = execSync(`ps -ef | grep login | grep ${term.pid} | grep -v grep`).toString();
    console.log(`${new Date()} ${checkPid}`);
    return term;
  } catch (error) {
    console.log(`${new Date()} check term: ${error}`);
    term.kill();
    termMap.delete(term.pid);
    const newTerm = nodeEnvBind(req);
    termMap.set(term.pid, term);
    return newTerm
  }
}

//解决跨域问题
app.all("*", function (req, res, next) {
    res.header("Access-Control-Allow-Origin", "*");
    res.header("Access-Control-Allow-Headers", "Content-Type");
    res.header("Access-Control-Allow-Methods", "*");
    next();
});
//服务端初始化
app.get("/api/v1/terminal/pid", (req, res) => {
  try {
    let term = nodeEnvBind(req);
    term = checkTerm(term, req);
    res.send({
      code: 20000000,
      data: {
        pid: term.pid.toString()
      },
      message: "success"
    });
  } catch (error) {
    console.log(`${new Date()} get pid: ${error}`);
    res.send({
      code: 21200001,
      data: null,
      message: "Init terminal failed!"
    });
  }
  res.end();
});

app.ws("/api/v1/terminal/socket/:pid", (ws, req) => {
    const pid = parseInt(req.params.pid);
    const term = termMap.get(pid);
    term.on("data", function (data) {
      try {
        // console.log(data)
        ws.send(data);
      } catch(err) {
        console.log(`${new Date()} ws: ${err}`);
      }
    });

    ws.on("message", (data) => {
      try {
        // console.log(data);
        term.write(data);
      } catch(err) {
        console.log(`${new Date()} ws message: ${err}`);
        term.write(err.toString)
      }
    });
    ws.on("close", function () {
        term.kill();
        termMap.delete(pid);
    });
});

try {
  let port = 8089
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--port' && i + 1 < args.length) {
      port = Number(args[i + 1])
      i += 1
    }
  }
  app.listen(port, "0.0.0.0");
  console.log(`terminal start at: http://0.0.0.0:${port}/`)
} catch(err) {
  console.log(err)
}
