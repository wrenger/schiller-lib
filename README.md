# SBV

The schiller library software.

It is based on the [godot](https://godotengine.org/) engine.

## Architecture

This application follows the 3-tier principle.

* **UI Layer:** This is implemented using godot's gui library and GDScript for the basic functionality.
* **Application Layer:** This is implemented in Rust using [GDNative](https://docs.godotengine.org/en/stable/tutorials/plugins/gdnative/index.html) as interface to the UI.
It contains the business logic and most of the computation is done in this layer.
* **Database Layer:** The sqlite database that stores the persistand data specific to a project.

## Building the GDNative Library

The application layer is written in [rust](https://www.rust-lang.org/), which has to be installed first.

Also the `libclang` library is needed for `bindgen` to generate the C bindings (see https://rust-lang.github.io/rust-bindgen/requirements.html).

After all dependencies are installed the library can be build with cargo:
```bash
cargo build
```

> The paths in [sbv.gdnlib](lib/sbv.gdnlib) have to be updated accordingly.

### Cross-Compilation: Linux to Windows

For cross-compilation the [cross](https://github.com/rust-embedded/cross) tool is used, which is a fancy wrapper around docker.
Because this project uses `bindgen`, which depends on `libclang` there is a special docker image ([Dockerfile](docker/cross-win/Dockerfile)) that specifies those dependencies.
This docker image has to be build only on the first time.
```bash
docker build -t custom/sbv-win docker/cross-win
```

Then the build process can be started by executing:
```bash
cross build --target=x86_64-pc-windows-gnu
```

## Distribute

After the GDNative library has been build the project can be exported within the godot editor.

> See https://docs.godotengine.org/en/stable/getting_started/workflow/export/index.html
