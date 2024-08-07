# Changelog

This document tracks changes of released library versions.

See also [Pulse](https://github.com/godot-rust/gdext/pulse/monthly) for recent activities.  
Cutting-edge API docs of the `master` branch are available [here](https://godot-rust.github.io/docs/gdext).

## [v0.1.2](https://docs.rs/godot/0.1.2)

_15 July 2024_

### 🌻 Features

- Add more `normalized` functions ([#761](https://github.com/godot-rust/gdext/pull/761))
- Add conversion from `Vec<$Element>` to `$PackedArray` types ([#785](https://github.com/godot-rust/gdext/pull/785))
- Add `snapped` to integer vectors ([#768](https://github.com/godot-rust/gdext/pull/768))
- Add determinant to `Transform2D` ([#770](https://github.com/godot-rust/gdext/pull/770))
- Support `#[export(range = (radians_as_degrees, suffix=XX))]` ([#783](https://github.com/godot-rust/gdext/pull/783))
- Add support for `nothreads` Wasm builds (Godot 4.3+) ([#794](https://github.com/godot-rust/gdext/pull/794))

### 🧹 Quality of life

- Reorder compile errors for `#[derive(GodotClass)]` ([#773](https://github.com/godot-rust/gdext/pull/773))
- Change `Global` to use `Once` ([#752](https://github.com/godot-rust/gdext/pull/752))
- Prevent global `CallError` tracker from growing indefinitely ([#798](https://github.com/godot-rust/gdext/pull/798))

### 🛠️ Bugfixes

- Change logic to disable `OpenXR` for iOS ([#781](https://github.com/godot-rust/gdext/pull/781))
- Pointer is already `*const u32` on aarch64 ([#788](https://github.com/godot-rust/gdext/pull/788))
- Handle panics in virtual interface methods ([#757](https://github.com/godot-rust/gdext/pull/757))

### 📚 Documentation

- Document why `Basis` columns are `a`, `b`, and `c` ([#776](https://github.com/godot-rust/gdext/pull/776))


## [v0.1.1](https://docs.rs/godot/0.1.1)

_24 June 2024_

Initial release on crates.io. See [devlog article](https://godot-rust.github.io/dev/june-2024-update).
