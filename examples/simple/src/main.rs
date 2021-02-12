use rff::{AppConfig, Color};

static HTML: &'static str = include_str!("index.html");

fn main() {
    let config = AppConfig {
        app_title: String::from("Write your application name here"),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        app_debug: false,
        window_rgba: Color {
            r: 123,
            g: 213,
            b: 213,
            a: 225
        },
        window_frameless: false,
        db_path: "rff_db".to_string()
    };
    rff::launch(config, String::from(HTML));
}