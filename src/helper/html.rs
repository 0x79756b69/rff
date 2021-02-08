
use crate::helper::data_controller::get_files_from_dir;
use std::fs::read_to_string;

pub fn build_html(mut html: String, css :String, mut js: String) -> String {
    let apilib = load_js_files(vec!["./src/js/"]);
    js += &apilib;
    html = html.replace("{LOAD_JS}", &js);
    html = html.replace("{LOAD_CSS}", &css);
    html
}


pub fn load_css_files(paths : Vec<&str>) -> String {
    let mut css = Vec::new();
    for path in paths {
        // Todo この二重ループもうちょっといい方法ないかなあ。
        for i in get_files_from_dir(true, "css", &path) {
            css.push(i);
        }
    }
    let mut tx_css = "".to_string();
    for i in css {
        let s = read_to_string(i).unwrap_or(String::from(""));
        tx_css += &format!(r#"<style type="text/css">{}</style>"#, s);
    }
    // println!("{:?}", css);
    // println!("{:?}", tx_css);
    tx_css
}

pub fn load_js_files(paths : Vec<&str>) -> String {
    let mut js = Vec::new();
    for path in paths {
        for i in get_files_from_dir(true, "js", &path) {
            js.push(i);
        }
    }
    let mut tx_js = "".to_string();
    for i in js {
        let s = read_to_string(i).unwrap_or(String::from(""));
        tx_js += &format!(r#"<script type="text/javascript">{}</script>"#, s);
    }
    // println!("{:?}", js);
    // println!("{:?}", tx_js);
    tx_js
}

