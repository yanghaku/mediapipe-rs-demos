#!/bin/bash

# Wrapper the WasmEdge to run wasm using cargo.

WASMEDGE_DEFAULT_PATH=~/.wasmedge
if [[ -z "${WASMEDGE_PATH}" ]]; then
  export WASMEDGE_PATH=${WASMEDGE_DEFAULT_PATH}
fi

export WASMEDGE_BIN_PATH=${WASMEDGE_PATH}/bin
export WASMEDGE_LIB_PATH=${WASMEDGE_PATH}/lib

# need these environment variables to run
export WASMEDGE_PLUGIN_PATH=${WASMEDGE_LIB_PATH}/wasmedge
export PATH=${WASMEDGE_BIN_PATH}:${PATH}
export LD_LIBRARY_PATH=${WASMEDGE_LIB_PATH}:${LD_LIBRARY_PATH}

wasmedge --dir .:. "$@"
