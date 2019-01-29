#!/bin/bash

LIBRIME_ROOT=~/code/rime/librime
# TODO: for macOS
LIBRIME_OUTPUT=${LIBRIME_ROOT}/xdebug/lib/Debug

export C_INCLUDE_PATH=${LIBRIME_ROOT}/src
export LIBRARY_PATH=${LIBRIME_OUTPUT}
export LD_LIBRARY_PATH=${LIBRIME_OUTPUT}

cargo test --features separate-gears-lib
