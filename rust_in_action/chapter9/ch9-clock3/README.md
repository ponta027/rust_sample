# README

```
clock 0.1.3
Gets and sets the time

USAGE:
    ch9-clock3 [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --use-standard <std>     [default: rfc3339]  [possible values: rfc2822, rfc3339, timestamp]

ARGS:
    <action>      [possible values: get, set, check-ntp]
    <dateime>    When <action> is 'set' appley <datetime>> . Otherwise , ignore.

 Note : UNIX timestampsare parsed
 ```

> ch9-clock3 check-ntp 

> ch9-clock3 set 2022-10-13T15:36:47.493990877+00:00

