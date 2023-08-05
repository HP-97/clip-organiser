
@test "can run our Rust program" {
    bats_require_minimum_version 1.5.0
    run -0 cargo run
}