This project provides safe and easy to use API bindings to Tcl/Tk commands.

# Features

1. Friendly API for both Rustaceans and Tk programers.

# Quickstart

## A quick glance

```rust
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    root.add_label( -text("constructs widgets and layout step by step") )?
        .pack(())?;
    let f = root
        .add_frame(())?
        .pack(())?;
    let _btn = f
        .add_button( "btn" -text("quit") -command("destroy .") )?
        .pack(())?;
    Ok( main_loop() )
}
```

## Another glance

```rust
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    tk.root().add_widgets(
        -pack( -label( -text("constructs widgets and layout in one expression") ))
        -pack( -frame( -pack( -button( "btn" -text("quit") -command("destroy .") ))))
    )?;
    Ok( main_loop() )
}
```

## The naming conventions in translating Tk commands to Rust bindings

1. Prefix Tk widget constructors with `add_` and put parentheses around option values.

    The Tk command to add a widget looks like `Constructor path -options_and_values`, e.g.

    ```tcl
    label .lb -text "lorem ipsum" -width 50 -height 20
    ```

    The equivalent Rust statement is as follows.

    ```rust_no_run
    let lb = root.add_label( /*".lb"*/ -text("lorem ipsum") -width(50) -height(20) )?;
    ```

2. Converts Tcl's imperative style to Rust's object style

    The Tk command is in the form of "verb noun options", e.g.

    ```tcl
    pack .lb -fill both
    ```

    The equivalent Rust statement is in th form of "object method options", as follows.

    ```rust_no_run
    lb.pack( -fill("both") )?; // use pack(()) without any option.
    ```

3. Converts Tk's space-separated commands to Rust's underscore-separated function names.

    Tk commands are space-separated, e.g.

    ```tcl
    tk fontchooser show
    ```

    The equivalent Rust statement is as follows.

    ```rust_no_run
    tk.fontchooser_show()?;
    ```

    Users can look into the Tk command reference and find the "fontchooser" page then search "show".

4. Distinguish between set and get via the `set_` prefix.

    In Tk, it is common to distinguish set and get by providing or omitting the value argument, e.g.

    `wm title window "Lorem ipsum"` means to set the window's title to "Lorem ipsum",
    while `wm title window` means to get the windows' title.

    The equivalent Rust statements are as follows.

    ```rust_no_run
    window.set_wm_title( "Lorem ipsum" )?;
    window.wm_title()?;
    ```

# Documents

1. [Tk tutorial](https://oooutlk.github.io/tk/)

2. [Official Tk command references](https://www.tcl.tk/man/tcl/TkCmd/contents.html)

# License

Under Apache License 2.0 or MIT License, at your will.
