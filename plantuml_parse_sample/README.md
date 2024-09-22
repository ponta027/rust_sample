# plantuml parser

## usage

```
plantuml_parse_sample 
description for command

USAGE:
    plantuml_parse_sample --file <FILE>

OPTIONS:
    -f, --file <FILE>    description for this argument
    -h, --help           Print help information

```


## description

plantuml のシーケンス図のメッセージをfrom,to,message,message_type
でパースして、リストを返す。


| item         | desc |
|--------------|------|
| from         | メッセージ元のActor     |
| to           | メッセージ先のActor     |
| message      | メッセージ    |
| message_type | 矢印の方向と色設定など     |


-------------------------------------------


