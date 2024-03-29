#!/bin/bash

librime_search_paths=(
    librime
    ../librime
    ~/librime
)

for path in "${librime_search_paths[@]}"
do
    if [[ -d "${path}" ]]
    then
        librime_root="${path}"
        break
    fi
done

if [[ -n "${librime_root}" ]]
then
    echo "found librime at '${librime_root}'"
else
    echo >&2 "librime not found"
    exit 1
fi

if [[ "$1" = 'debug' ]]
then
    build_dir='debug'
else
    build_dir='build'
fi

librime_output="${librime_root}/${build_dir}/lib"

export C_INCLUDE_PATH="${librime_root}/src"
export LIBRARY_PATH="${librime_output}"
export DYLD_LIBRARY_PATH="${librime_output}"

cargo test
