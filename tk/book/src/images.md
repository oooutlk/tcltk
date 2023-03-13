# Images

We've seen the basics of using images already, displaying them in labels or
buttons, for example. We create an image object, usually from a file on disk.

```rust,no_run
// cargo run --example images

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let label_image = root
        .add_ttk_label( "label_image" -text("Full name") )?
        .pack(())?;

    let img = tk.image_create_photo( -file("book/src/images/tcl.gif") )?;
    label_image.configure( -image(img) )?;

    root.winfo_rgb( TkColor::Name("red") )?;
    Ok( main_loop() )
}
```
