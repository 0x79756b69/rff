mod structs;
mod helper;
mod handler;

pub use structs::{AppConfig};
pub use helper::*;
pub use web_view::Color;

use web_view::*;
use crate::structs::Cmd::*;
use crate::handler::window_handler::*;
use crate::handler::data_handler::*;
use crate::handler::sql_handler::*;
use crate::handler::file_handler::f_save;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;


// ## WebView API
// webview.eval(&format!("updateTicks({}, {})", counter, user_data))
// exit() : Window exit

pub fn make_gui<'a>(cfg: AppConfig, html: &'a str, title: &'a str) -> WebView<'a, ()>{
    let db_path = cfg.db_path.clone();
    let mut webview = web_view::builder()
        .title(title)
        .content(Content::Html(html))
        .size(cfg.window_width, cfg.window_height)
        .frameless(cfg.window_frameless)
        .resizable(cfg.window_resizable)
        .debug(cfg.app_debug)
        .user_data(())
        .invoke_handler(
            move |webview, arg| {
                let result = match serde_json::from_str(arg).unwrap() {
                    // API CALL
                    DataInsert {param} => d_insert(webview, param, db_path.clone()),
                    DataFetch {param} =>  d_fetch(webview, param, db_path.clone()),
                    DataDelete {param} =>  d_delete(webview, param, db_path.clone()),
                    SqlQuery {param} =>  sql_query(webview, param),
                    FileSave {param} => f_save(webview, param),
                    WindowFullscreen {param} => w_fullscreen(webview, param),
                    // WindowNotify {param} => Ok(w_notify(&mut notify, param)),
                    // WindowShow {param} =>  w_show(webview, param),
                    // WindowHide {param} =>  w_hide(webview, param),
                };
                result
            }
        )
        .build()
        .unwrap();
    webview.set_color(cfg.window_rgba);
    webview
}
