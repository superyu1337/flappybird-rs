#!/bin/zsh
rm flappybird-rs.zip
cargo build --release
mkdir flappybird-rs
cp ./target/release/flappybird-rs ./flappybird-rs/flappybird
cp -r ./assets ./flappybird-rs/assets
7z a ./flappybird-rs_linux.zip ./flappybird-rs
rm -r flappybird-rs