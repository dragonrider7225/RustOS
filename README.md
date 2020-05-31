# RustOS
An OS written in Rust based on reading [https://os.phil-opp.com/]

This project requires `cargo-xbuild` because `::core`, `::compiler_builtins`,
and `::alloc` need to be built for custom target triples.
To install `cargo-xbuild`, run `$ cargo install cargo-xbuild`.
