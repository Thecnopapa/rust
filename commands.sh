rust-install(){
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}

run (){
    cargo run "$@"
}

build (){
    maturin develop
}

test (){
    python test.py "$@"
}

benchmark (){
    python benchmark.py "$@"
}