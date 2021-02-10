

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_title: String,
    pub app_debug : bool,
    pub window_width : i32,
    pub window_height : i32,
    pub window_resizable : bool,
    pub window_frameless : bool,
    pub window_rgba : web_view::Color,
    pub db_path : String
}




// 内部のRust実行関係で使う。 Todo: コマンド関連もうちょっといい書き方ないかな？
#[derive(Debug, Serialize)]
pub struct CmdSend {
    pub t : String, // Todo : もうこれとか最悪。
    pub callback: String,
    pub param: String,
}
#[derive(Debug, Deserialize)]
pub struct CmdReceive {
    pub cb : String,
    pub query: String,
}
#[derive(Debug, Deserialize)]
pub struct DataInsertSt {
    pub key : String,
    pub value : String
}
#[derive(Debug, Deserialize)]
pub struct DataFetchSt {
    pub key : String
}
#[derive(Debug, Deserialize)]
pub struct DataDeleteSt {
    pub key : String
}



#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    DataInsert {
        param : String
    },
    DataFetch {
        param : String
    },
    DataDelete {
        param : String
    },
    // Window {
    //
    // },
}

// String to Command structure
pub trait CmdParser {
    fn into_cmd(&self) -> CmdReceive;
    fn into_dins(&self) -> DataInsertSt;
    fn into_dfet(&self) -> DataFetchSt;
    fn into_ddel(&self) -> DataDeleteSt;
}
impl CmdParser for String {
    fn into_cmd(&self) -> CmdReceive{
        let st: CmdReceive = serde_json::from_str(&self).unwrap();
        st
    }
    fn into_dins(&self) -> DataInsertSt{
        let st: DataInsertSt = serde_json::from_str(&self).unwrap();
        st
    }
    fn into_dfet(&self) -> DataFetchSt{
        let st: DataFetchSt = serde_json::from_str(&self).unwrap();
        st
    }
    fn into_ddel(&self) -> DataDeleteSt{
        let st: DataDeleteSt = serde_json::from_str(&self).unwrap();
        st
    }
}