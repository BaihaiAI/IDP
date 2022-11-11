const workercode = () => {
  let times = null;
  let _idleCount = 0;
  const outTime = 0.5 * 3600; // 1h
  self.onmessage = function (e) {
    if (!e.data) {
      clearInterval(times);
      _idleCount = 0;
    };
    let flg = false;
    times = setInterval(() => {
      _idleCount = _idleCount + 5; // 0 5 10 15 20 25 30
      // console.log('@离开页面超过：', _idleCount);
      if (_idleCount >= outTime) {
        flg = true;
        clearInterval(times);
        self.postMessage(flg);
      }
    }, 5000);
  }
};
// 把脚本代码转为string
let code = workercode.toString();
code = code.substring(code.indexOf("{") + 1, code.lastIndexOf("}"));

const blob = new Blob([code], { type: "application/javascript" });
const worker_script = URL.createObjectURL(blob);

module.exports = worker_script;
