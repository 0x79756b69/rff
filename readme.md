## Rust Friendly Framework (RFF)

現状、Rustには[tauri](https://github.com/tauri-apps/tauri) などのGUIフレームワークが存在しますが、作者からするとあまり美しいインターフェイスとは思えません。

そこで、現状と比較して機能は少ない(今後アップデートする可能性もある)ですが、以下の特徴を備えたGUIフレームワークを制作しました。

このフレームワークは主に開発者の仕事を減らすために、以下の特徴を備えます。
- Mac, Linux, Windowsに対応したGUIのソフトウェアを爆速でつくれる。
- 直感的な理解がしやすいインターフェイスによるハイレベルな保守性。
- Rustの知識がほとんど不要でつくれる。

## デザイン
責任は基本的にJavascriptが負いますが、このライブラリが提供するAPI実行の責任はRustが負います。
**このライブラリは、JavascriptとRust、そしてデータベースの結合をスムーズにします。**

## 主にインターフェイスの比較
このライブラリの内部では[Web-view](https://github.com/Boscop/web-view) が使用されています。よって見方によってはWeb-viewのラッパーとも言えるでしょう。
- Web-viewやtauriよりも使いやすいインターフェイスを提供します。
- tauriにはNode.js等、他の依存関係が必要ですが、RFFは一つのCrateのみです。

### Web-view
Web-viewのExamplesディテクトリ下の[ToDoアプリ](https://github.com/Boscop/web-view/blob/master/examples/todo.rs) や有志によるQiitaでの[紹介記事](https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd) をご覧ください。

### tauri

```rust:main.rs
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use serde::Serialize;

#[derive(Serialize)]
struct Reply {
  data: String,
}

fn main() {
  tauri::AppBuilder::new()
    .setup(|webview, _source| {
      let mut webview = webview.as_mut();
      let mut webview_clone = webview.clone();
      tauri::event::listen(String::from("js-event"), move |msg| {
        println!("got js-event with message '{:?}'", msg);
        let reply = Reply {
          data: "something else".to_string(),
        };

        tauri::event::emit(
          &mut webview,
          String::from("rust-event"),
          Some(serde_json::to_string(&reply).unwrap()),
        )
        .expect("failed to emit");
      });

      webview_clone
        .dispatch(move |w| {
          w.eval("window.onTauriInit()");
        })
        .expect("failed to dispatch");
    })
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            LogOperation { event, payload } => {
              println!("{} {:?}", event, payload);
            }
            PerformRequest {
              endpoint,
              body,
              callback,
              error,
            } => {
              // tauri::execute_promise is a helper for APIs that uses the tauri.promisified JS function
              // so you can easily communicate between JS and Rust with promises
              tauri::execute_promise(
                _webview,
                move || {
                  println!("{} {:?}", endpoint, body);
                  // perform an async operation here
                  // if the returned value is Ok, the promise will be resolved with its value
                  // if the returned value is Err, the promise will be rejected with its value
                  // the value is a string that will be eval'd
                  Ok("{ key: 'response', value: [{ id: 3 }] }".to_string())
                },
                callback,
                error,
              )
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
```

tauriの[examples](https://github.com/tauri-apps/examples/tree/dev/tauri/communication) ディレクトリを参考にした
### RFF

```rust:main.rs
extern crate RFF;
use RFF::rff::*;
use RFF::rff::helper::html::{load_css_files, build_html, load_js_files};
use web_view::Color;
use std::fs::read_to_string;


fn main() {
    let html = read_to_string("./src/view/index.html").unwrap();
    let css = load_css_files(["./src/view/lib/css", "./src/view/my_lib/css"].to_vec());
    let js = load_js_files(["./src/view/lib/js", "./src/view/my_lib/js"].to_vec());
    let contents = build_html(html, css, js);

    let config = structs::AppConfig {
        app_title: String::from("Application Name"),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        app_debug: false,
        window_rgba: Color {
            r: 123,
            g: 213,
            b: 213,
            a: 225
        }
    };
    rff::launch(config, contents);
}

```
データやウィンドウの操作については、JS側からAPIを叩く。

上で読み込んでいるindex.html内には、`{LOAD_JS}`と`{LOAD_CSS}`があり、そこにjsとcssが挿入される。

## WIP Memo

APIを提供する

APIについてドキュメント書く。

まだ同期処理しか対応してない。→非同期に対応する。(かも)


