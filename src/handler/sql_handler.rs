use web_view::{WebView, WVResult, Content};
use crate::structs::{CmdParser, CmdSend};

pub fn sql_query(wv: &mut WebView<()>, cmd: String) -> WVResult {

    let st = cmd.into_sql_query();
    let database_url = st.mysql_url; // like "mysql://root:4141@localhost:3306/apack_asobi";
    let val = CmdSend{
        t: "sqlQuery".to_string(),
        callback: None,
        param: None
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result

}