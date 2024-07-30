default:
    @just --list

test-v1:
    cargo test -p v1_recursive

test-v2:
    cargo test -p v2_iterative

bench:
    cargo bench