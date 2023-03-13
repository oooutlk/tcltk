use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let label_text = root
        .add_ttk_label( "label_text" -text("Full name") )?
        .pack(())?;

    label_text.configure( -textvariable("resultContents") )?;
    tk.set( "resultContents", "New value to display" );

    label_text.configure( -font("TkDefaultFont") )?;

    let label_image = root
        .add_ttk_label( "label_image" -text("Full name") )?
        .pack(())?;

    let img = tk.image_create_photo( -file("book/src/images/tcl.gif") )?;
    label_image.configure( -image(img) )?;

    Ok( main_loop() )
}
