FFMPEG_DEPS="$(realpath "$(dirname -- "$0")")/../ffmpeg-deps"

export WASI_SYSROOT="${FFMPEG_DEPS}/wasi-sysroot"
export FFMPEG_DIR="${FFMPEG_DEPS}/ffmpeg-lib"
export CLANG_RT="${FFMPEG_DEPS}/clang-rt"

export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=${WASI_SYSROOT} --target=wasm32-wasi -fvisibility=default"
