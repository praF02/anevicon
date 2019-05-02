#!/bin/sh

# This script simply generates an .AppImage file inside the anevicon.AppDir
# directory. Usage: ./appimage.sh

cargo build --release
cp target/release/anevicon anevicon.AppDir/anevicon
cd anevicon.AppDir
./appimagetool-x86_64.AppImage .
