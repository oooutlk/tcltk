# Installing Tk

Before using the [tk crate](https://crates.io/crates/tk), you have to install
the native Tk distribution on your machine. Check your OS and pick the
correspoding chapter to go on.

## The Obligatory First Program

To make sure that everything actually did work, let's try to run a "Hello World"
program in Tk.

```rust
// cargo run --example the_obligatory_first_program

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    root.add_label( -text("hello,world!") )?.pack(())?;
    Ok( main_loop() )
}
```
