## Rust Friendly Framework (RFF)

現状、Rustには[tauri](https://github.com/tauri-apps/tauri) などのGUIフレームワークが存在しますが、作者からするとあまり美しいインターフェイスとは思えません。

そこで、現状と比較して機能は少ない(今後アップデートする可能性もある)ですが、以下の特徴を備えたGUIアプリケーション作成のためのライブラリを制作しました。

- Mac, Linux, Windowsに対応したGUIのソフトウェアを爆速でつくれる。
- Rustの知識がほとんど不要でつくれる。

## デザイン
責任は基本的にJavascriptが負いますが、このライブラリが提供するAPI実行の責任はRustが負います。

## インターフェイス
このライブラリの内部では[Web-view](https://github.com/Boscop/web-view) が使用されています。
- Web-viewやtauriよりも使いやすいインターフェイスを提供します。
- tauriにはNode.js等、他の依存関係がありますが、RFFはこのライブラリのみで完結します。

### RFF

```rust:main.rs
use rff::{html::{load_css_files, load_js_files, build_html}, Color, AppConfig};
use std::fs::read_to_string;

fn main() {
    let html = read_to_string("./src/view/index.html").unwrap();
    let css = load_css_files(["./src/view/lib/primary/css", "./src/view/lib/secondary/css"].to_vec());
    let js = load_js_files(["./src/view/lib/primary/js", "./src/view/my_lib/secondary/js"].to_vec());
    let contents = build_html(html, css, js);

    let config = AppConfig {
        app_title: String::from("Example Application"),
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
        window_frameless: false
    };
    rff::launch(config, contents);
}

```
データやウィンドウの操作については、JS側からAPIを叩く。

上で読み込んでいるindex.html内には、`{LOAD_JS}`と`{LOAD_CSS}`があり、そこにjsとcssが挿入される。

詳細はexamplesを参照。

## 比較
このライブラリのシンプルさに関しては、以下で書かれているコードと比較してください。
### Web-view
Web-viewのExamplesディテクトリ下の[ToDoアプリ](https://github.com/Boscop/web-view/blob/master/examples/todo.rs) や有志によるQiitaでの[紹介記事](https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd)

### tauri
tauriの[examples](https://github.com/tauri-apps/examples/tree/dev/tauri/communication)


## WIP Memo

APIを提供する

APIについてドキュメント書く。

まだ同期処理しか対応してない。→非同期に対応する。(かも)


