var Cmds = {
    DataInsert : "dataInsert",
    DataFetch : "dataFetch",
    DataDelete : "dataDelete"
};
// receiver
function receiver_from_rust(arg) {
    // arg.t arg.callback arg.param
    // arg.paramの構造は arg.tによって異なります。
    // alert(JSON.stringify(arg));
    switch (arg.t) {
        case Cmds.DataInsert:
            alert(arg.callback);
            alert("DATA ins CALLED");
            break
        case Cmds.DataFetch:
            let param = JSON.parse(arg.param);
            var wrap = s => "{ return " + arg.callback + " };" //return the block having function expression
            var func = new Function( wrap(arg.callback) );
            func.call(null).call(null, param.v ); //invoke the function using arguments
            break
        case Cmds.DataDelete:
            alert(eval(arg.callback));
            alert("DATA DELETE CALLED");
            break
    }
    // arg -> {type, data}
    // type = ["data_insert","data_fetch","window_exit","window_appear","window_fullscreen","window_title"]
}

//sender
Cmd = (function () {
    /** DATA **/
        // 何か共通処理
    var data = function () {};
    // 各実装
    data.prototype = {
        insert: function (key, value, callback) {
            this.type = Cmds.DataInsert;
            let query = JSON.stringify({key: key, value: value});
            this.receive = request_to_rust(this.type, callback, query);
            return this.receive;
        },
        select: function (key, callback) {
            this.type = Cmds.DataFetch;
            // Todo: storage_name いらない。
            let query = JSON.stringify({key: key});
            request_to_rust(this.type, callback, query);
        },
        delete: function (key, callback) {
            this.type = Cmds.DataDelete;
            let query = JSON.stringify({key: key});
            request_to_rust(this.type, callback, query);
        }
    };
    /** WINDOW **/
    var window = function () {};
    window.prototype = {
        exit: function (callback) {
            this.type = "window_exit";
        },
        appear: function () {
            this.type = "window_appear";
        },
        set_fullscreen: function (bool) {
            this.type = "window_fullscreen";
        },
        change_title: function (name) {
            this.type = "window_title";
        }
    };
    // Todo: 面倒くさいので今度実装します。
    /** HTTP **/
    var http = function () {};
    http.prototype = {
        dummy: function () {

        }
    };
    return {
        data: data,
        window: window,
        http: http
    };
})();

function request_to_rust(ctype, callback, query) {
    let param = JSON.stringify({cb : callback.toString(), query})
    external.invoke(JSON.stringify({cmd: ctype, param: param}));
}

// webview.set_fullscreen(true)
// webview.exit()
//
// function dummy() {
//     let proc = function(data) {
//         alert(data);
//     }
//     let d = new Cmd.data();
//     d.insert("key", "vAALUE");
//     d.select("key", proc);
//     d.delete("key");
//     // let fin = d.fetch("key")
//     // w.change_title("aaa");
//     // w.aa("aaa");
// }