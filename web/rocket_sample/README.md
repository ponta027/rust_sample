# Rocket sample

## [Rocket](https://rocket.rs/)

Rust で書かれたWeb Framework

* Simple to write fast.
* secure
が売り


0.5.0から [Future and Async]( https://rocket.rs/v0.5-rc/guide/overview/#futures-and-async)
が追加 そのためそのためAPIが変更されている。


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
