rust-install(){
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}

run (){
    cargo run "$@"
}

build (){
    cargo build "$@"
}