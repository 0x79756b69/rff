use web_view::{WebView, WVResult, Content};
use crate::structs::{CmdParser, CmdSend};
use serde_json::json;

pub fn sql_query(wv: &mut WebView<()>, cmd: String) -> WVResult {
    // Todo: DBへの接続と、クエリの実行を行う
    let st = cmd.into_sql_query();
    // let database_url = st.mysql_url; // like "mysql://root:4141@localhost:3306/apack_asobi";
    let exec = mys_wrapper::exec(
        st.mysql_url,
        st.stmt,
        st.params
    );
    // println!("{:?}", hmap);
    let sql_result = match exec {
        Ok(map) => {
            let json = json!(map);
            serde_json::to_string(&json).unwrap()
        }
        Err(e) => {
            e
        }
    };
    let v = json!({"v": sql_result, "cb": st.callback, "params": st.value });
    let val = CmdSend{
        t: "sqlQuery".to_string(),
        param: Option::from(serde_json::to_string(&v).unwrap())
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    result

}