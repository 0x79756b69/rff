

function request(cmd, params) {
    switch (cmd) {
        case "save_data":
            let a = JSON.stringify({name: cmd, param: params});
            external.invoke(JSON.stringify({cmd: "e", ctrl: a}));
            break
        case "change_title":
            break
        case "req":
            break
    }
    let a = JSON.stringify({name: cmd, param: params});
    external.invoke(JSON.stringify({cmd: "e", ctrl: a}));
}

// webview.set_fullscreen(true)
function dummy() {

}

window.onload = function() {

};