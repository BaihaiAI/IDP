//全屏
// 打开浏览器全屏模式
/**
 * screenChange <Function> 为窗口变化的 的回调函数  参数为当前是否为全屏状态
 */
 const init = (screenChange)=>{

    // 取值17是为了处理页面内容出现滚动条的情况
    let isFull = window.screen.height - window.document.documentElement.clientHeight <= 17;
    
    // 阻止F11键默认事件，用HTML5全屏API代替
    window.addEventListener('keydown', function (e) {
        e = e || window.event;
        if (e.keyCode===122 && !isFull) {
            e.preventDefault();
            enterFullScreen();
        }
    });
    //监听窗口变化
    window.onresize = function () {
        isFull = window.screen.height - window.document.documentElement.clientHeight <= 17;
        screenChange(isFull);
    }
};

//进入全屏
const enterFullScreen  = ()=>{
    let el = document.documentElement;
    let rfs = el.requestFullScreen || el.webkitRequestFullScreen || el.mozRequestFullScreen || el.msRequestFullscreen
    if (rfs) { // typeof rfs != "undefined" && rfs
        rfs.call(el)
    } else if (typeof window.ActiveXObject !== 'undefined') {
        // for IE，这里其实就是模拟了按下键盘的F11，使浏览器全屏
        let wscript = new ActiveXObject('WScript.Shell');  //eslint-disable-line
        if (wscript != null) {
            wscript.SendKeys('{F11}')
        }
    }
};


// 退出全屏
const exitFullScreen = ()=>{
    let el = document;
    let cfs = el.cancelFullScreen || el.mozCancelFullScreen || el.msExitFullscreen || el.webkitExitFullscreen || el.exitFullscreen;
    if (cfs) { // typeof cfs != "undefined" && cfs
        cfs.call(el);
    } else if (typeof window.ActiveXObject !== 'undefined') {
        // for IE，这里和fullScreen相同，模拟按下F11键退出全屏
        let wscript = new ActiveXObject('WScript.Shell'); //eslint-disable-line
        if (wscript != null) {
            wscript.SendKeys('{F11}')
        }
    }
};

export default {
    init,
    enterFullScreen,
    exitFullScreen
}