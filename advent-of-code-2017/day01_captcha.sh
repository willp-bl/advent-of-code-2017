#!/usr/bin/env sh
EXE=day01_captcha.exe
rustc src/day01/mod.rs -o $EXE && ./$EXE
