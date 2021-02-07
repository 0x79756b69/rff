

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_title: String,
    pub window_width : i32,
    pub window_height : i32,
    pub window_resizable : bool,
    pub app_debug : bool,
    pub window_rgba : web_view::Color,
}

// 内部のRust実行関係で使う。
#[derive(Debug, Deserialize)]
pub struct CmdReceive {
    pub name : String,
    pub param: String,
}

