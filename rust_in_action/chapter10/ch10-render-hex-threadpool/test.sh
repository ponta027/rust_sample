#!/bin/bash

cargo run -- $( echo 'Rust in Action' | sha1sum | cut -f1 -d ' ')
