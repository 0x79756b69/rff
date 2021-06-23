use web_view::{WebView, WVResult, Content};

use crate::structs::*;
use serde_json::json;

// When API Called
// Todo: d_ 系　共通処理　まとめられないかな？
pub fn d_insert(wv: &mut WebView<()>, cmd: String, db: String) -> WVResult {
    let st = cmd.into_dins();
    let db: sled::Db = sled::open(db).unwrap();
    // Todo: insert と delete dbのハンドリング系 expect
    db.insert(st.key.as_bytes(), st.value.as_bytes());
    let val = CmdSend{
        t: "dataInsert".to_string(),
        callback: None,
        param: None
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result

}

pub fn d_fetch(wv: &mut WebView<()>, cmd: String, db: String) -> WVResult {
    let st = cmd.into_dfet();
    let db: sled::Db = sled::open(db).unwrap();
    let re;
    match db.get(st.key) {
        Ok(Some(res)) => {re = String::from_utf8(res.to_vec()).unwrap();}
        _ => {re = String::from("");}
    }
    let v = json!({"v": re, "params": st.value});
    // todo: CmdSend builder
    let val = CmdSend{
        t: "dataFetch".to_string(),
        callback: Option::from(st.callback),
        param: Option::from(serde_json::to_string(&v).unwrap())
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}

pub fn d_delete(wv: &mut WebView<()>, cmd: String, db: String) -> WVResult {
    let st = cmd.into_ddel();
    let db: sled::Db = sled::open(db).unwrap();
    db.remove(st.key);
    let val = CmdSend{
        t: "dataDelete".to_string(),
        callback: None,
        param: None
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}
// wv.set_title("a");
// wv.set_visibility(true); で Windowをつくる（見かけ上）
// webview.eval(&format!("notify_success({})", serde_json::to_string(&("Switched to ".to_owned() + arg)).unwrap()));
