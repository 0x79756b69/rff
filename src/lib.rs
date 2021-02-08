mod structs;
mod helper;

pub use structs::{AppConfig, CmdReceive};
pub use helper::*;
pub use web_view::Color;

use web_view::*;
use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub fn launch(config: AppConfig, html : String) {
    println!("Launching GUI ...");
    let name = config.app_title.clone();
    let webview = make_gui(config, &html, &*name);
    webview.run().unwrap();
}

// When API Called
fn execute(wv: &mut WebView<HashMap<&str, &str>>, cmd: &str) -> WVResult {
    // change_title, exit, save_data, http_request,
    let a: CmdReceive = serde_json::from_str(cmd).unwrap();
    println!("{:?}", a);
    // wv.set_title("a");
    let result = wv.eval(&format!("acc_list_display({})", serde_json::to_string("aaaaa").unwrap()));
    result
}

// ## WebView API
// webview.eval(&format!("updateTicks({}, {})", counter, user_data))
// exit() : Window exit

fn make_gui<'a>(cfg: AppConfig, html: &'a str, name: &'a str) -> WebView<'a, HashMap<&'a str, &'a str>>{
    // -> WebView<HashMap<&str, &str>>
    // cfg.handler;
    let mut webview = web_view::builder()
        .title(name)
        .content(Content::Html(html))
        .size(cfg.window_width, cfg.window_height)
        .frameless(cfg.window_frameless)
        .resizable(cfg.window_resizable)
        .debug(cfg.app_debug)
        .user_data(HashMap::new())
        .invoke_handler(
            |webview, arg| {
                use Cmd::*;
                let result = match serde_json::from_str(arg).unwrap() {
                    // API CALL
                    E {ctrl} => execute(webview, &ctrl)
                    // Other INTERNAL CALL
                };
                // println!("{:?}", arg);
                result
            }
        )
        .build()
        .unwrap();
    webview.set_color(cfg.window_rgba);
    webview
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    E {
        ctrl : String
    },
}


