GODOT=./bin/headless/Godot_v3.2.3-stable_linux_headless.64
cargo build --release
$GODOT --no-window --export "Linux/X11 - Tiny" export/x11/sbv-gd.x86_64
cross build --target=x86_64-pc-windows-gnu --release
$GODOT --no-window --export "Windows - Tiny" export/windows/sbv-gd.exe
