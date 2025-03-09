# cut command


## spec 


* 使用法: cut OPTION... [FILE]...
* 各ファイルの行から選択した部分だけ切り出す
* ファイルの指定がない場合や-を指定した場合、標準入力から読み込み


### オプション

  -b, --bytes=LIST        バイトで数えた LIST を選択する
  -c, --characters=LIST   文字で数えた LIST を選択する
  -d, --delimiter=DELIM   フィールドの区切り文字として TAB の代わりに DELIM
                            を使用する 
  -f, --fields=LIST       LIST のフィールドのみを選択する。-s オプションが
                            指定されない限り、区切り文字を含まない行も表示
                            する
  -n                      (無視される)
      --complement        選択されたバイト数、文字数またはフィールド数の
                            組を補足する
  -s, --only-delimited    区切り文字を含まない行を出力しない
      --output-delimiter=STRING  出力の区切り文字として STRING を使用
                            デフォルトでは入力の区切り文字を使用
  -z, --zero-terminated     行の区切りを改行でなく NUL にする
      --help        display this help and exit
      --version     output version information and exit

-b, -c, -f はこのうち 1 つだけを、かつ必ず使用してください。
各 LIST は、コンマで区切られた 1 つまたは複数の範囲で構成されます。
選択された入力は読み込まれた順番で一度だけ出力されます。
範囲指定は以下のいずれかです。


-------------------------------------------


## cutコマンドの使い方

> cut -d , -f 2  tests/inputs/sample.csv 
> cut -c 5-10 tests/inputs/sample.csv 
> cut -c 5- tests/inputs/sample.csv 
> cut -b 5-10 tests/inputs/sample.csv 


