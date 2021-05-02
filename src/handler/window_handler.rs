use web_view::{WebView, WVResult};

use crate::structs::*;
use serde_json::json;

// When API Called
pub fn w_fullscreen(wv: &mut WebView<()>, cmd: String) -> WVResult {
    let st = cmd.into_wfullscreen();
    if st.bool {
        wv.set_fullscreen(true);
    }else {
        wv.set_fullscreen(false);
    }
    let val = CmdSend{
        t: "windowFullscreen".to_string(),
        callback: None,
        param: String::from("")
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}

// pub fn w_notify(wv: &mut WebView<()>, cmd: String) {
//     println!("{:?}", cmd);
//     let st = cmd.into_wnotify();
//     wv.set_html(&*st.html);
//     wv.set_visible(true);
// }

pub fn w_show(wv: &mut WebView<()>, cmd: String) -> WVResult {
    wv.set_visible(true);
    let val = CmdSend{
        t: "windowShow".to_string(),
        callback: None,
        param: String::from("")
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}

pub fn w_hide(wv: &mut WebView<()>, cmd: String) -> WVResult {
    wv.set_visible(false);
    let val = CmdSend{
        t: "windowHide".to_string(),
        callback: None,
        param: String::from("")
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result
}
// wv.set_title("a");
// wv.set_visibility(true); で Windowをつくる（見かけ上）
// webview.eval(&format!("notify_success({})", serde_json::to_string(&("Switched to ".to_owned() + arg)).unwrap()));
