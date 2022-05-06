# SchillerLib

The schiller library software.
A small, simple and intuitive library program for school libraries.

It is based on the [godot](https://godotengine.org/) engine.

### Download

The latest builds can be downloaded from the [releases page](https://github.com/wrenger/schiller-lib/releases/latest).


## Architecture

This application follows the 3-tier principle.

* **UI Layer:** This is implemented using Godot's GUI library and GDScript for the basic functionality.
* **Application Layer:** This is implemented in Rust using
[GDNative](https://docs.godotengine.org/en/stable/tutorials/plugins/gdnative/index.html) as interface to the UI.
It contains the business logic and most of the computation is done in this layer.
* **Database Layer:** The SQLite database that stores the persistent data specific to a project.

### UI Layer

The [UI](ui) is developed in Godot with GDScript for interactions and
interfacing with the GDNative application layer.

This layer is also responsible for internationalization
([translations.csv](translations/translations.csv)).
Currently, there are only two languages supported (English and German).
Contributions for new languages or improved translations are very welcome.

### Application Layer

This layer is implemented in Rust ([src](src)) and defines the GDNative
interface ([src/api](src/api)) that is used by the UI.

It is responsible for the consistency checks and business logic.
This layer also manages the database connection to store and fetch the project data.

Besides that, it also provides some functionality that is missing in Godot,
like the `Date` class.

### Database Layer

The [SQLite](https://sqlite.org/index.html) database has the following schema:

<img src="images/sbv_db.svg" alt="Database Schema" width=400 />

It contains any project-specific data and settings and can be distributed as such.


## Building the GDNative Library

The application layer is written in [rust](https://www.rust-lang.org/), which
has to be installed first.

Also, the `libclang-dev` library is needed for `bindgen` to generate C bindings
(see https://rust-lang.github.io/rust-bindgen/requirements.html).

After the dependencies are installed, the library can be built with cargo:
```bash
cargo build --release
```

> For debug builds, the paths in [schiller_lib.gdnlib](lib/schiller_lib.gdnlib) have to be updated accordingly.

### Cross-Compilation: Linux to Windows

Because of the `bindgen` dependencies, there is a custom docker container that
is also used for the CI ([Dockerfile](docker/Dockerfile)).

This container can be used for the cross-compilation as shown below:

```bash
# Build the docker image (only once)
docker build -t ghcr.io/wrenger/schiller-lib/cross docker
# Start the image and compile
docker run --rm -it --user "$(id -u)":"$(id -g)" --volume=$(pwd):/home/docker/project -w /home/docker/project \
    ghcr.io/wrenger/schiller-lib/cross \
    cargo build --target=x86_64-pc-windows-gnu --release
```

> Alternatively, the project can also been built without docker.
> This, however, requires the installation of `rust` including the
> `x86_64-pc-windows-gnu` toolchain and `gcc-mingw-w64` for your distribution.
>
> ```bash
> cargo build --target=x86_64-pc-windows-gnu --release
> ```


## Package & Distribute

After building the GDNative library, the project can be exported within the Godot editor.

> See https://docs.godotengine.org/en/stable/getting_started/workflow/export/index.html

Alternatively, this can be done from the command line:
```bash
# linux
mkdir export/x11
path/to/godot --export x11 export/x11/schiller-lib.x86_64
# windows
mkdir export/win
path/to/godot --export win export/win/schiller-lib.exe
```
