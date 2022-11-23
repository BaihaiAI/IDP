import cookie from 'react-cookies';

(function () {
    if (Boolean(process.env.NODE_OPEN)) {
        cookie.save('region', 'a');
        cookie.save('userId', '1');
        cookie.save('teamId', '1');
        cookie.save('projectId', '1');
        localStorage.setItem("historyOpenProject", '1');
    }
})()