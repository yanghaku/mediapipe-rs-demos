# [MediaPipe-rs] Demos

## Build from source

For the normal applications, just use ```cargo``` to build:

```shell
cargo build --release
```

For the ffmpeg applications (`audio-classification-ffmpeg` and `image-classification-ffmpeg`),
need download ffmpeg wasm32-wasi library and wasi-sysroot.

1. Download the FFMpeg dependencies
   ```shell
   ./scripts/download-ffmpeg-deps.sh
   ```
2. Set environment variables for ```cargo```
   ```shell
   source ./scripts/ffmpeg-cargo-env.sh
   ```
3. Build using ```cargo```
   ```shell
   pushd crates/audio/audio-classification-ffmpeg
   cargo build --release
   popd
   ```

   ```shell
   pushd crates/vision/image-classification-ffmpeg
   cargo build --release
   popd
   ```

## Run the demos

Use the [WasmEdge] with TfLite backend.

references: [Build WasmEdge with WASI-NN TfLite backend]


# License

This project is licensed under the Apache 2.0 license. See [LICENSE] for more details.

[LICENSE]: LICENSE

[MediaPipe-rs]: https://github.com/WasmEdge/mediapipe-rs

[Build WasmEdge with WASI-NN TfLite backend]: https://wasmedge.org/book/en/contribute/build_from_src/plugin_wasi_nn.html#build-wasmedge-with-wasi-nn-tensorflow-lite-backend

[WasmEdge]: https://github.com/WasmEdge/WasmEdge

[MediaPipe]: https://github.com/google/mediapipe

[MediaPipe Solutions]: https://developers.google.com/mediapipe/solutions/

[TF Hub]: https://tfhub.dev/
