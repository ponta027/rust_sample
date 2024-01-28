# ITMeadia Rust

## HTTP Serverを実装
https://atmarkit.itmedia.co.jp/ait/articles/2310/20/news002.html


## Actix Web

https://atmarkit.itmedia.co.jp/ait/articles/2311/17/news004.html


## utoipa

### add Package
Cargo.toml に以下を追加

```
utoipa = { version = "4.1.0", features = ["actix_extras", "chrono", "serde_yaml"] }
utoipa-swagger-ui = { version = "5.0.0", features = ["actix-web"] }
```

### add OpenApi Macro

### add Path 

### add ToSchema

https://docs.rs/utoipa/latest/utoipa/derive.OpenApi.html
https://github.com/juhaku/utoipa/tree/master/examples/todo-actix


## html template

* [tera](https://keats.github.io/tera/)

-------------------------------------------


