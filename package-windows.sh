#!/bin/zsh
rm flappybird-rs.zip
cargo build --release --target x86_64-pc-windows-gnu
mkdir flappybird-rs
cp ./target/x86_64-pc-windows-gnu/release/flappybird-rs.exe ./flappybird-rs/flappybird.exe
cp -r ./assets ./flappybird-rs/assets
7z a ./flappybird-rs_windows.zip ./flappybird-rs
rm -r flappybird-rs