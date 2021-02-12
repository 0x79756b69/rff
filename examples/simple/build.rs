
use rff::html::{load_css_files, load_js_files, build_html};
use rff::data_controller::create_file;
use std::fs::read_to_string;

fn main() {
    let html = read_to_string("./src/view/index.html").unwrap_or("".to_string());
    let css = load_css_files(["./src/view/lib/css", "./src/view/my_lib/css"].to_vec());
    let js = load_js_files(["./src/view/lib/js/primary", "./src/view/lib/js/secondary", "./src/view/my_lib/js"].to_vec());
    let contents = build_html(html, css, js);
    create_file("./src/index.html", contents);
}