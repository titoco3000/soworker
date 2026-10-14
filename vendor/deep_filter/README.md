# deep_filter (vendored)

The inference part of [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet)'s `libDF`, taken from commit
`d375b2d8309e0935d165700c91da9de862a99c31` with the DeepFilterNet 3 model (`models/DeepFilterNet3_onnx.tar.gz`).
SoWorker uses it to clean the microphone (`src/mic.rs`). Licensed MIT or Apache-2.0, see `LICENSE-MIT` / `LICENSE-APACHE`.

Changes from upstream:

- Only `lib.rs`, `tract.rs`, `transforms.rs` and `logging.rs` are kept; datasets, the C API, wasm and the binaries are gone.
- tract 0.21.18 and ndarray 0.16 instead of tract 0.21.4 / ndarray 0.15: tract up to 0.21.6 fails to optimize the model
  ("duplicate name /convt3/Conv.bias"). The one API change this needed: `symbol_table` is now `symbols`.
- `into_shape` replaced by `into_shape_with_order` (deprecated in ndarray 0.16), and elided lifetimes spelled out
  (`'_`) for newer rustc lints.
- The model path in `tract.rs` points at `models/` inside this directory.
