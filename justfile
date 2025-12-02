set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
    @just run

run *args:
    cargo run {{ args }}

build *args:
    cargo build {{ args }}

bundle := if os() == "macos" {
    "just _bundle-macos"
} else if os() == "windows" {
    "just _bundle-windows"
} else {
    ""
}

plist := '''EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDisplayName</key>
    <string>bigfile</string>
    <key>CFBundleExecutable</key>
    <string>bigfile-gui</string>
    <key>CFBundleIdentifier</key>
    <string>rip.faint.bigfile</string>
    <key>CFBundleName</key>
    <string>bigfile</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleIconFile</key>
    <string>Icon</string>
</dict>
</plist>
EOF'''

bundle:
    @{{ bundle }}

_bundle-macos:
    cargo build --release

    @rm -rf build/bigfile.app

    @mkdir -p build/bigfile.app/Contents/MacOS
    @mkdir -p build/bigfile.app/Contents/Resources

    @cp target/release/bigfile-gui build/bigfile.app/Contents/MacOS/bigfile-gui
    @cp assets/Icon.icns build/bigfile.app/Contents/Resources/Icon.icns

    @cat > build/bigfile.app/Contents/Info.plist <<{{ plist }}

_bundle-windows:
    cargo build --release

    @mkdir build -Force > $null
    @copy target/release/bigfile-gui.exe build/bigfile.exe -Force
