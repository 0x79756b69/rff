var Cmds = {
    DataInsert : "dataInsert",
    DataFetch : "dataFetch",
    DataDelete : "dataDelete",
    SqlQuery : "sqlQuery",
    FileSave : "fileSave",
    WindowHide : "windowHide",
    WindowShow : "windowShow",
    WindowExit : "windowExit",
    WindowNotify : "windowNotify",
    WindowFullscreen : "windowFullscreen"
};
// receiver
function receiver_from_rust(arg) {
    // arg.t arg.callback arg.param
    // arg.paramの構造は arg.tによって異なります。
    // alert(JSON.stringify(arg));
    let param = JSON.parse(arg.param);
    switch (arg.t) {
        case Cmds.DataInsert:
            // alert(arg.callback);
            // alert("DATA ins CALLED");
            break
        case Cmds.DataFetch:
            var wrap = s => "{ return " + param.cb + " };" //return the block having function expression
            var func = new Function( wrap(param.cb) );
            func.call(null).call(null, param.v, param.params ); //invoke the function using arguments
            break
        case Cmds.DataDelete:
            // alert(eval(arg.callback));
            // alert("DATA DELETE CALLED");
            break
        case Cmds.SqlQuery:
            var wrap = s => "{ return " + param.cb + " };" //return the block having function expression
            var func = new Function( wrap(param.cb) );
            func.call(null).call(null, param.v, param.params ); //invoke the function using arguments
            break
        case Cmds.FileSave:
            if (param === "Ok") {

            }else {

            }
            // var wrap = s => "{ return " + param.cb + " };" //return the block having function expression
            // var func = new Function( wrap(param.cb) );
            // func.call(null).call(null, param.v, param.params ); //invoke the function using arguments
            break

        case Cmds.WindowShow:
            break
        case Cmds.WindowHide:
            break
    }
    // arg -> {type, data}
    // type = ["data_insert","data_fetch","window_exit","window_appear","window_fullscreen","window_title"]
}

//sender
Cmd = (function () {
    /** KV DATA **/
        // 何か共通処理
    var data = function () {};
    // 各実装
    data.prototype = {
        insert: function (key, value) {
            this.type = Cmds.DataInsert;
            let query = JSON.stringify({key: key, value: value});
            request_to_rust(this.type, query);
        },
        select: function (key, callback, v) {
            this.type = Cmds.DataFetch;
            // Todo: storage_name いらない。
            let query = JSON.stringify({key: key, callback : callback.toString(), value: v});
            request_to_rust(this.type, query);
        },
        delete: function (key) {
            this.type = Cmds.DataDelete;
            let query = JSON.stringify({key: key});
            request_to_rust(this.type, query);
        }
    };
    /** MYSQL DATA **/
    var sql = function () {};
    sql.prototype = {
        // params is array
        query: function (db_url, stmt, params, callback, value) {
            this.type = Cmds.SqlQuery;
            let query = JSON.stringify({
                mysql_url: db_url,
                stmt: stmt,
                params: params,
                callback : callback.toString(),
                value : value
            });
            request_to_rust(this.type, query);
        }
    };

    /** FILE DATA **/
    var file = function () {};
    file.prototype = {
        // params is array
        save: function (file_path, file_data) {
            this.type = Cmds.FileSave;
            let query = JSON.stringify({ file_path, file_data });
            request_to_rust(this.type, query);
        }
    };


    /** WINDOW **/

    var window = function () {};
    window.prototype = {
        exit: function (callback) {
            this.type = Cmds.WindowExit;
        },
        // notify: function (html) {
        //     this.type = Cmds.WindowNotify;
        //     let query = JSON.stringify({html: html});
        //     request_to_rust(this.type, query);
        // },
        // show: function (name, html) {
        //     this.type = Cmds.WindowShow;
        //     let query = JSON.stringify({w_name: name, html: html});
        //     request_to_rust(this.type, query);
        // },
        // hide: function (name) {
        //     this.type = Cmds.WindowHide;
        //     let query = JSON.stringify({w_name: name});
        //     request_to_rust(this.type, query);
        // },
        set_fullscreen: function (bool) {
            this.type = Cmds.WindowFullscreen;
            let query = JSON.stringify({bool: bool});
            request_to_rust(this.type, query);
        },
        change_title: function (name) {
            this.type = "window_title";
        }
    };
    // Todo: 面倒くさいので今度実装します。
    return {
        data: data,
        sql: sql,
        file: file,
        window: window
    };
})();

function request_to_rust(ctype, query) {
    // let param = JSON.stringify({cb : callback.toString(), query})
    external.invoke(JSON.stringify({cmd: ctype, param: query}));
}

// 構造体定義

// function Callback(func, value) {
//     this.func = func;
//     this.value = value;
// }
// function Sql(db_url, query, params) {
//     this.url = db_url;
//     this.query = query;
//     this.params = params; // array
// }


// webview.set_fullscreen(true)
// webview.exit()
//
function dummy() {
    // let proc = function(data, value) {
    //     alert(data);
    //     alert(value);
    // }
    // let d = new Cmd.data();
    // d.insert("key", "vAALUE");
    // let v = "Hello!";
    // d.select("key", proc, v);
    // d.delete("key");
    //
    // let w = new Cmd.window();
    // w.show()
    // let fin = d.fetch("key")
    // w.change_title("aaa");
    // w.aa("aaa");
}