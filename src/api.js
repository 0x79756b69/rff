

function request(cmd, params) {
    switch (cmd) {
        case "data":
            // data.insert data.find
            let a = JSON.stringify({name: cmd, param: params});
            external.invoke(JSON.stringify({cmd: "e", ctrl: a}));
            break
        case "window":
            break
        case "http":
            break
    }
    let a = JSON.stringify({name: cmd, param: params});
    external.invoke(JSON.stringify({cmd: "e", ctrl: a}));
}

// webview.set_fullscreen(true)
// webview.exit()
//
function dummy() {
    let params = {

        K : "Andymori",
        V : "GOOD MUSICIAN"
    }
    request("data.insert", params);
}

window.onload = function() {

};