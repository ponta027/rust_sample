# WASM 環境構築

https://rustwasm.github.io/docs/book/game-of-life/setup.html

に書いてあることをコピーとちょっとしたエラー時の対応のログ。


## 必要なパッケージのインストール

### npm

```
npm install npm@latest -g
```

### wasm pack

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### cargo-generate

```
$ cargo --version
cargo 1.71.1 (7f1d04c00 2023-07-29)
```


```
$ cargo install cargo-generate
  Updating crates.io index
  Installing cargo-generate v0.19.0
  Updating crates.io index
error: failed to compile `cargo-generate v0.19.0`, intermediate artifacts can be found at `/tmp/cargo-installDG3XkQ`
Caused by:
  package `normpath v1.2.0` cannot be built because it requires rustc 1.74.0 or newer, while the currently active rustc v
ersion is 1.71.1
  Try re-running cargo install with `--locked`
```

エラーの指示の通りrustcのバージョンアップを
する。

```
$ rustup check
stable-aarch64-unknown-linux-gnu - Update available : 1.71.1 (eb26296b5 2023-08-03) -> 1.76.0 (07dca489a 2024-02-04)
rustup - Up to date : 1.26.0
$ rustup update stable
$ cargo install cargo-generate
```

無事インストール完了
 
## 動作確認

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
cd wasm-game-of-life
wasm-pack build
npm init wasm-app www
```

pkg/package.json を作成し、下記を追加

```
{
  "name": "wasm-game-of-life",
  "collaborators": [
    "Your Name <your.email@example.com>"
  ],
  "description": null,
  "version": "0.1.0",
  "license": null,
  "repository": null,
  "files": [
    "wasm_game_of_life_bg.wasm",
    "wasm_game_of_life.d.ts"
  ],
  "main": "wasm_game_of_life.js",
  "types": "wasm_game_of_life.d.ts"
}
```



```
vi www/package.json
```

以下ブロックを追加

```
  "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  },
```

```

export NODE_OPTIONS=--openssl-legacy-provider
npm install
npm run start
```



ブラウザでhttp://localhost:8080 を開くとアラートが表示される

## Exercises

lib.rsのgreeting関数に引数をして、jsから引数ありの呼び出しをしてみる。

lib.rs

```
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

index.js

```
wasm.greet("Your Name");
```

