The `tcl_derive` crate provides the proc-macro implementation for the `tcl` crate.

1. The `#[proc]` attribute to get a Rust `fn` ready for registering as a Tcl command.

2. The `tclfn!{}` macro to define a Rust `fn` and register it as a Tcl command.

3. The `tclosure!{}` macro to define a Rust closure and register it as a Tcl command.

See [`tcl` crate's doc](../tcl/README.md) for more.

# License

Under Apache License 2.0 or MIT License, at your will.
