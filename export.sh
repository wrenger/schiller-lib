cargo build --release
godot --no-window --export "Linux/X11 - Tiny" export/x11/sbv-gd.x86_64
cross build --target=x86_64-pc-windows-gnu --release
godot --no-window --export "Windows - Tiny" export/windows/sbv-gd.exe
