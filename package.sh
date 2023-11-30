#!/bin/zsh
rm -r dst

mkdir dst
./package-linux.sh
mv flappybird-rs_linux.zip dst/flappybird-rs_linux.zip

./package-windows.sh
mv flappybird-rs_windows.zip dst/flappybird-rs_windows.zip
