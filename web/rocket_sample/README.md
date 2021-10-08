# Rocket sample

## [Rocket](https://rocket.rs/)

Rust で書かれたWeb Framework

* Simple to write fast.
* secure
が売り


0.5.0から [Future and Async]( https://rocket.rs/v0.5-rc/guide/overview/#futures-and-async)
が追加 そのためそのためAPIが変更されている。


https://rocket.rs/v0.5-rc/guide/requests/

## static files

web applicationでよくある static files 設定。
0.4.0ではrocket_contribで存在する。
0.5.rcでは、関数定義して登録する
NamedFileを使うとContent-Typeも自動設定

```
use rocket::fs::NamedFile;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
```


https://rocket.rs/v0.5-rc/guide/responses/


## Configuration

https://rocket.rs/v0.5-rc/guide/configuration/#profiles

設定値をデフォルト値から以下の方法で設定変更できる。

* Rocket.toml
* 環境変数
* Custom Provider

example Rocket.tomk

```
[default]
address="0.0.0.0"
[debug]
port = 9000
[nyc]
port = 9001
[release]
port = 9002
```

Profile,Config Pathは環境変数で設定可能


```
> ROCKET_PROFILE=nyc
> ROCKET_CONFIG=Rocket.toml
```





--
