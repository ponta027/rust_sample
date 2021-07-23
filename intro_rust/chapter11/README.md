# Chapter 11 create package

## use library

* http://crates.io


### use rand crate

* add Cargo.toml

```
[dependencites]
rand="0.7"
```

### use binary

use ripgrep 

> cargo install ripgrep
> rg 
> rg Hello main.rs


## create package

> cargo new my_random --lib
> cargo build -release

create library target/release/libmy_random.rlib .
copy this library to other project, use library


