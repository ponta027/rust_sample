# README



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
        // Enable C++ namespace support
        .enable_cxx_namespaces()
        // Add extra clang args for supporting `C++`
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
``` 

### std::functionを使いたい場合

引数にstd::functionを使うことはできないため、一旦ラップした型を定義する。       
opaque_typeを用いるとvoid*として扱える仕組みがあるが、呼び出し側の型の整合を考えると
ラップした型定義をしたほうがよい。

## reference

https://github.com/rust-lang/rust-bindgen
https://rust-lang.github.io/rust-bindgen/README
https://rust-lang.github.io/rust-bindgen/cpp.html
https://github.com/wisonye/rust-ffi-demo?tab=readme-ov-file#4-lets-call-c-function-in-rust


