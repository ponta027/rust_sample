# README

## 目的

rust-bindgenのサンプル作成し、理解を深める


## 手順

```
cargo install bindgen
```

## bind-genについて

### サポート機能
* 継承
* Method
* コンストラクタとデストラクタ（暗黙的なものではない)
* オーバーロード
* Specializationなしのテンプレート

### 未サポート機能

* レイアウト、サイズ、アラインメント
* inline function
* template function method , class ,struct
* specializationのtype
* Cross 言語の継承
* 自動的なcopy,moveコンストラクタ、デストラクタ
    * Rustはmoveセマンティクスがサポートできない
* 例外


### 複数ヘッダファイルを設定する場合

```
    let bindings = bindgen::Builder::default()
        .header("../sample/inc/myclass.h")
        .header("../sample/inc/myclass_ns.h")   // 最後だけ有効になる
        // Enable C++ namespace support
        .enable_cxx_namespaces()
        // Add extra clang args for supporting `C++`
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
``` 


## reference
https://github.com/rust-lang/rust-bindgen
https://rust-lang.github.io/rust-bindgen/README
https://rust-lang.github.io/rust-bindgen/cpp.html
https://github.com/wisonye/rust-ffi-demo?tab=readme-ov-file#4-lets-call-c-function-in-rust
