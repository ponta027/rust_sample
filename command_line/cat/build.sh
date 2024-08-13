#!/bin/bash
cargo build
RET=$?
if [ "${RET}" = 0 ] ; then
cargo test --target-dir tests
fi
