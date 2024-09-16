# binuid-cli
## Install or Reinstall  
cargo install --path .
## Create a new project  
binuid-cli.exe init --name <project_name>

## binuid commands
### binuid init --mode=lib --name=<project_name>

--mode = lib | bin | namescape

## how to run rustc:
rustc ./lib/binuid_std.rs --crate-type=lib --crate-name=binuid_std --edition=2021

## how to run rustdoc
rustdoc ./lib/binuid_std.rs --crate-name binuid_std

## how to run rustc with dependencies:
rustc ./app/pages/index.rs --crate-type=bin --edition=2021 --extern binuid_std=./deps/libbinuid_std.rlib


## spliter: split the entire bin project into each Component 
go into each file in your project and move each item that implements Component trait into its own file.