# SBV

The schiller library software.

It is based on the [godot](https://godotengine.org/) engine.

> Note: This project is under heavy development and some features may not work
> as expected or are completely missing at the moment.

### Download latest builds

* [Windows](https://gitlab.com/wrenger/sbv-gd/-/jobs/artifacts/master/download?job=deploy%3Awindows)
* [Linux](https://gitlab.com/wrenger/sbv-gd/-/jobs/artifacts/master/download?job=deploy%3Alinux)


## Architecture

This application follows the 3-tier principle.

* **UI Layer:** This is implemented using godot's gui library and GDScript for the basic functionality.
* **Application Layer:** This is implemented in Rust using
[GDNative](https://docs.godotengine.org/en/stable/tutorials/plugins/gdnative/index.html) as interface to the UI.
It contains the business logic and most of the computation is done in this layer.
* **Database Layer:** The SQLite database that stores the persistand data specific to a project.

### UI Layer

The [UI](ui) is developed in Godot with GDScript for interactions and
interfacing with the GDNative application layer.

This layer is also responsible for internationalization
([translations.csv](translations/translations.csv)).
Currently there are only two languages supported (English and German).
Contributions for new languages or improved translations are very welcome.

### Application Layer

This layer is implemented in Rust ([src](src)) and defines the gdnative
interface ([src/api](src/api)) that is used by the UI.

It is responsible for the consistency checks and business logic.
Also it connects to the database to store and fetch the project data.

Besides that it also provides some functionality that is missing in Godot,
like the `Date` class.

### Database Layer

The [SQLite](https://sqlite.org/index.html) database has the following schema:

<img src="images/sbv_db.svg" alt="Database Schema" width=600 />

It contains any project specific information and can be distributed as such.


## Building the GDNative Library

The application layer is written in [rust](https://www.rust-lang.org/), which
has to be installed first.

Also the `libclang-dev` library is needed for `bindgen` to generate the C bindings
(see https://rust-lang.github.io/rust-bindgen/requirements.html).

After the dependencies are installed the library can be build with cargo:
```bash
cargo build --release
```

> The paths in [sbv.gdnlib](lib/sbv.gdnlib) have to be updated accordingly.

### Cross-Compilation: Linux to Windows

Because of the `bindgen` dependencies there is a custom docker container that
is also been used for the CI ([Dockerfile](docker/cross-win/Dockerfile)).

This container can be used for the cross-compilation as shown below:

```bash
# Build the docker image (only once)
docker build -t registry.gitlab.com/wrenger/sbv-gd docker
# Start the image and compile
docker run --rm -it --volume=$(pwd):/home/docker/project -w /home/docker/project \
    registry.gitlab.com/wrenger/sbv-gd \
    cargo build --target=x86_64-pc-windows-gnu --release
```

> Alternatively the project can also been build without docker.
> This, however, requires the installation of `rust` including the `x86_64-pc-windows-gnu` toolchaing,
> `gcc-mingw-w64` and `libclang-dev` for your distribution.
>
> ```bash
> cargo build --target=x86_64-pc-windows-gnu --release
> ```


## Distribute

After the GDNative library has been build, the project can be exported within the godot editor.

> See https://docs.godotengine.org/en/stable/getting_started/workflow/export/index.html

Alternatively this can also be done from the command line:
```bash
# linux
mkdir export/x11
path/to/godot --export x11 export/x11/sbv-gd.x86_64
# windows
mkdir export/win
path/to/godot --export win export/win/sbv-gd.exe
```
