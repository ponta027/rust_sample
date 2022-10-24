# README

1. コマンドライン引数を解析して、DateTime<FixedOffset>の値を作る。
  FixedOffsetというタイムゾーンは「ユーザから提供されるかもしれないタイムゾーン」
  のプロキシとしてchronoが提供する。
  chrono自身はどのタイムゾーンが選択されるかをコンパイル時に知ることができない。
2. DateTime<FixedOffset>を DateTime<Local>  に変換すると、タイムゾーンを比較できる。
3.  システムコールの引数として使う、OSごとの構造体を実体化する
4. システムの時刻を　unsafeブロックの中で設定する。このブロックが必要なのは、責任をOSに委ねるからだ。
5. 更新された時刻を表示する。


## 実行方法

```
> ./target/debug/ch9-clock2 set 2022-10-13T15:36:47.493990877+00:00
Unable to set the time : Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }
```

dockerの権限設定でエラーになった。

```
> docker run --privileged
```

privileged で起動する。


起動して、実行した場合、



で実行する。


Unable to set the time : Os { code: 2, kind: NotFound, message: "No such file or directory" }


/dev/rtcがないため、エラーになっている。


