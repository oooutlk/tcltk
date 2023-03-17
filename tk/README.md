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

# Why I gave up writing most documents

I planned to write doc comments for every public types, traits, macros and
functions in this crate. Isn't it very bold? All APIs, 100% well documented in
doc comments. My earlier crates' APIs are nearly 100% documented. I thought a
well documented crate would make all users excited. It is estimated to take only
20-25 days to fill all APIs with doc comments. But finally I gave up. Why? I
read through the
[official Tk command references](https://www.tcl.tk/man/tcl/TkCmd), and could
not make my mind to translate it in Rust. Do you know what the biggest problem
is? Certainly it's an engineering problem. Firstly, in a brief, the biggest
problem is development efficiency. No time for documenting, for an experimental
crate in its version 0.1.0. Fundamental APIs may evolve in later versions 0.2,
0.3...etc, just like they evolved in unpublished 0.0.x versions. Once changed,
it may be necessary to change a lot of doc comments, taking another time budget
of 20-25 days. Too slow for rapid developing. Secondly, the probability of
wasting the effort to providing document in Rust is more than 50%. The users of
this crate may be familiar with Tcl/Tk programming, and they can master
immediately 95% usage of this crate after reading a few demonstration code. Even
users who are not familiar with Tk, can translate between Tcl and Rust
themselves when reading the Tk command reference, with the help of *naming
conventions* described above. I have learned that 100% documenting this
experimental crate will do harm to development efficiency, and lose a chance to
take advantage of the existing high quality documentation of Tcl/Tk library, to
which this crate provides bindings. Which is more important, 100% document
coverage, or rapid development? It is a pity that `cargo doc` produces little
content here, but lucky for me to be sane.

# The tutorial book

Luckily, I've got enough time to translate in Rust the
[Tk tutorial](https://oooutlk.github.io/tk/).

# License

Under Apache License 2.0 or MIT License, at your will.
