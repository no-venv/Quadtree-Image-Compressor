@echo off

cargo "build" "--release"
cd "parser"
cargo "build" "--release"
cd ".."
move "target\release\qtcompress" "qtcompress"
move "parser\target\release\qtcompress_parse" "qtcompress_parse"
