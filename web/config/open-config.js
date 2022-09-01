import cookie from 'react-cookies';

(function () {
    if (Boolean(process.env.NODE_OPEN)) {
        cookie.save('region', 'a');
        cookie.save('userId', '12345');
        cookie.save('teamId', '12345');
        cookie.save('projectId', '6789');
        localStorage.setItem("historyOpenProject", '6789');
    }
})()