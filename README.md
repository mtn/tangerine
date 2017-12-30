# tangerine
[![Build Status](https://travis-ci.org/mtn/tangerine.png)](https://travis-ci.org/mtn/tangerine)

This is a port of [fuchsia](https://github.com/mtn/fuchsia) to Rust, made so I can try out Rust as I learn it.

## Usage

Like fuchsia, it can be run in batch mode or with a repl. For example,

    $ cargo run
    >> (\x. x) y
    y
    >>

    $ cargo run examples/tests
    y
    (y y)
    y
    (y z)
    a
    (y w)
    (w w)

Tests can be run with `cargo test`
