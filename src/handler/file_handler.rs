use web_view::{WebView, WVResult};
use crate::structs::{CmdParser, CmdSend};

// use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub fn f_save(wv: &mut WebView<()>, cmd: String) -> WVResult {
    let st = cmd.into_fsave();
    // let path = FileDialog::new()
    //     .set_location("~/Desktop")
    //     .show_open_single_dir()
    //     .unwrap();
    // let path = match path {
    //     Some(path) => path,
    //     None => PathBuf::new(),
    // };
    // if path != PathBuf::new() {
    //     let edited_path = path.to_str().unwrap().replace("file://", "");
    //     // Todo: unwrap / currently not safe
    //     let new_path = edited_path.to_owned() + &st.file_name;
    //     // println!("{:?}", new_path);
    //     let mut file = File::create(new_path).unwrap();
    //     file.write_all(st.file_data.as_ref()).unwrap();
    //     file.flush().unwrap();
    // }
    // let new_path = "/Desktop/".to_owned() + &st.file_name;
    let mut file = File::create(st.file_path).unwrap();
    file.write_all(st.file_data.as_ref()).unwrap();
    file.flush().unwrap();
    let val = CmdSend{
        t: "fileSave".to_string(),
        param: None
    };
    let result = wv.eval(&format!("receiver_from_rust({})", serde_json::to_string(&val).unwrap()));
    return result
}