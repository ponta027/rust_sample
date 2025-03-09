# uniq command



## 参考

オリジナルのhelp

```
使用法: uniq [OPTION]... [INPUT [OUTPUT]]
INPUT (または標準入力) から行を読み込み、
連続する同じ行を取り除いて、OUTPUT (または標準出力) に出力します。

オプションが指定されない場合、連続する同じ行は最初に見つけた行にまとめられます。


長いオプションで必須となっている引数は短いオプションでも必須です。
  -c, --count           行の前に出現回数を出力する
  -d, --repeated        重複した行のみ出力する。出力はグループ毎に 1 回行われる
  -D                    重複する行を全て出力する
      --all-repeated[=METHOD]  -D と同様ですが、重複行のグループの区切り方法を指定できる
                                 METHOD={none(デフォルト),prepend,separate}
  -f, --skip-fields=N   最初の N 個のフィールドを比較しない
      --group[=METHOD]  すべての行を表示し、グループ間に区切りとして空行が置かれる
                          METHOD={separate(デフォルト),prepend,append,both}
  -i, --ignore-case     比較時に大文字と小文字の違いを無視する
  -s, --skip-chars=N    最初の N 文字を比較しない
  -u, --unique          重複していない行のみ出力する
  -z, --zero-terminated     行の区切りとして改行文字ではなく NUL を使用する
  -w, --check-chars=N   行の比較を最初の N 文字で行う
      --help        display this help and exit
      --version     output version information and exit

フィールドとは空白 (通常はスペース、タブ、その両方) が一つ以上連続し、
その後に空白以外の文字が続いているものです。
文字の前のフィールドはスキップされます。

備考: 'uniq' は隣接してない限り重複した行を検出しません。
最初に入力をソートしてください。または 'uniq' を使用せず
'sort -u' を使用することもできます。
```

## コマンドのヘルプ

```
Usage: uniq [OPTIONS] [FILE] [OUT_FILE]

Arguments:
  [FILE]      Input  file [default: -]
  [OUT_FILE]  Output file

Options:
  -c, --count  Show counts
  -h, --help   Print help
```


## test

ユニットテストではなく、結合テストで実施する。


```
cargo test --target-dir tests
```

-------------------------------------------




