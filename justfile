# run the program in debug mode
run:
    cargo run

fmt:
    rustfmt src/lib.rs
    rustfmt src/bin/main.rs

# test everything on default
test:
    bats test/test.bats
