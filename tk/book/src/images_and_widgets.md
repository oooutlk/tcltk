# Images and Widgets

Like canvas widgets, text widgets can contain images and any other Tk widgets
(including frames containing many other widgets). In a sense, this allows the
text widget to work as a geometry manager in its own right. The ability to add
images and widgets within the text opens up a world of possibilities for your
program.

Images are added to a text widget at a particular index, with the image
specified as an existing Tk image. Other options that allow you to fine-tune
padding, etc.

```rust,no_run
let img = tk.image_create_photo( -file("book/src/images/tcl.gif") )?;
txt.image_create( Index::tag_first("sel"), -image(img) )?;
```

Other widgets are added to a text widget in much the same way as images. The
widget being added must be a descendant of the text widget in the widget
hierarchy.

```rust,no_run
let b = txt.add_ttk_button( -text("Push Me") )?;
txt.window_create( Index::line_char(1,0), -window(b) )?;
```

## Run Example

`cargo run --example text_images_and_widgets`
