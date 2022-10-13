# README

## 実行方法

```
> ./target/debug/ch9-clock2 set 2022-10-13T15:36:47.493990877+00:00
Unable to set the time : Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }
```

エラー内容からdockerの権限設定が不足している。
下記オプションを設定してコンテナを起動。

```
> docker run --privileged
```


実行し、設定はされたが、エラーが発生している


```
> ./target/debug/ch9-clock2 set 2022-10-13T15:36:47.493990877+00:00
Unable to set the time : Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

RaspberryPiにはRTCが存在しないため、/dev/rtcへアクセスできずエラーになっている。


---


