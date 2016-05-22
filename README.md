# Calling nom parsers from C code

here is an example parser wrapped in a library callable by C code.

This is heavily influenced by html5ever's solution.

The parser itself is in `embed/src/lib.rs`, while the C interface is stored in another subproject in `embed_capi/`.

The compilation of the `embed_capi` project indicates which library must be linked as well:

```
$ cargo build
   Compiling embed_capi v0.0.1 (file:///Users/geal/dev/rust/projects/parsers/embed/embed_capi)
note: link against the following native artifacts when linking against this static library
note: the order and any duplication can be significant on some platforms, and so may need to be preserved
note: library: c
note: library: m
note: library: System
note: library: pthread
note: library: c
note: library: m
```

Once the library is build, it is present as `embed_capi/target/debug/libembed_capi-<hash>.a` (or in `embed_capi/target/release` if built with `cargo build --release`).

The `src` directory contains an example C program that links to this library.
