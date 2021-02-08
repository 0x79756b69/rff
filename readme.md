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
アプリケーションのビルド時。
```rust:main.rs
use rff::{AppConfig, Color};

// ビルド時に同梱するファイル
static HTML: &'static str = include_str!("index.html");

fn main() {
    let config = AppConfig {
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
        },
        window_frameless: false
    };
    // アプリケーションの開始
    rff::launch(config, String::from(HTML));
}
```
尚、一つのhtmlファイルにJS, CSS, 画像を詰める必要があります。

大丈夫です。 HTMLのビルド（ひとつのファイルにまとめる）時のヘルパーを用意しています。
```rust:main.rs
use rff::html::{load_css_files, load_js_files, build_html};
use std::fs::read_to_string;
use rff::data_controller::add_file;

fn main() {
    let html = read_to_string("./src/view/index.html").unwrap_or("".to_string());
    let css = load_css_files(["./src/view/lib/css", "./src/view/my_lib/css"].to_vec());
    let js = load_js_files(["./src/view/lib/js", "./src/view/my_lib/js"].to_vec());
    let contents = build_html(html, css, js);
    add_file("./src/index.html", contents);
}

```

## 使い方
データやウィンドウの操作については、JS側からAPIを叩く。

上で読み込んでいるindex.html内には、`{LOAD_JS}`と`{LOAD_CSS}`があり、そこにjsとcssが挿入される。

詳細はexamplesディレクトリを参照。


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


