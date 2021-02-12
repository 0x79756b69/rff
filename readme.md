## Rust Friendly Framework (RFF)

### 概要
現状、Rustには[tauri](https://github.com/tauri-apps/tauri) などのGUIフレームワークが存在しますが、作者からするとあまり美しいインターフェイスとは思えません。

そこで、現状と比較して機能は少ない(今後アップデートする可能性もある)ですが、以下の特徴を備えたGUIアプリケーション作成のためのライブラリを制作しました。

- Mac, Linux, Windowsに対応したGUIのソフトウェアを爆速でつくれる。
- Rustの知識がほとんど不要でつくれる。
- Web-viewやtauriよりも美しいインターフェイスを提供します。
- tauriにはNode.js等、他の依存関係がありますが、このライブラリは以下のインポートのみで完結します。

`rff = {git = "https://github.com/0x79756b69/rff", branch = "main"}`

- [もっと見る](http://zenn.dev/0x/articles/ae4ce76f58ee65)

[![](http://img.youtube.com/vi/xgTBCUP3aq4/0.jpg)](http://www.youtube.com/watch?v=xgTBCUP3aq4 "")

### どのように書けるか
アプリケーション本体。
```rust
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
そのため、HTMLのビルド（ひとつのファイルにまとめる）時のヘルパーを用意しています。
```rust
use rff::html::{load_css_files, load_js_files, build_html};
use rff::data_controller::add_file;
use std::fs::read_to_string;

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

詳細は*examples*ディレクトリを参照。


## コード比較
このライブラリのシンプルさに関しては、以下で書かれているコードと比較してください。
### Web-view
Web-viewのExamplesディテクトリ下の[ToDoアプリ](https://github.com/Boscop/web-view/blob/master/examples/todo.rs) や有志によるQiitaでの[紹介記事](https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd)

### tauri
tauriの[examples](https://github.com/tauri-apps/examples/tree/dev/tauri/communication)


## 提供API
### data
KVストア。
- 追加
- 選択
- 削除

#### insert
```js
let d = new Cmd.data();
d.insert("KEY", "VALUE");
```

#### select
```js
// CALLBACK FUNCTION MUST HAVE ARGUMENT THAT RECEIVE A VALUE FROM DB
let callbackfn = function(data) {
    alert(data); // Alert a value from DB
}
d.select("KEY", callbackfn);
```

#### delete
```js
d.delete("KEY");
```


## Notice
このソフトウェアはまだ開発段階です。
- APIの変更が起こります。
- セキュリティの検証がまだです。

[comment]: <> (## WIP Memo)

[comment]: <> (1. APIを提供する)

[comment]: <> (2. APIについてドキュメント書く。)

[comment]: <> (3. htmlビルド時にデフォルトでJSとCSSの圧縮。)

[comment]: <> (4. APIがまだ同期処理しか対応してない。→非同期に対応する。&#40;かも&#41;)


[comment]: <> (#### window)

[comment]: <> (ウィンドウ関連のイベント)

[comment]: <> (- マルチウィンドウ（インスタンス）)

[comment]: <> (- ウィンドウ無効化)

[comment]: <> (- フルスクリーン変更)

[comment]: <> (- タイトル変更)

[comment]: <> (#### http)

[comment]: <> (JSからHTTP送信できるけど？ここらへんはセキュリティを含め要検討)

