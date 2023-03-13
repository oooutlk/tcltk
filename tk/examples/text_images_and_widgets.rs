// cargo run --example text_images_and_widgets

use tk::*;
use tk::text::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let txt = root.add_text( "t" -width(40) -height(10) )?
        .grid(())?;

    //TODO: text doesn't contain any characters tagged with "sel"
    //let img = tk.image_create_photo( -file("book/src/images/tcl.gif") )?;
    //txt.image_create( Index::tag_first("sel"), -image(img) )?;

    let b = txt.add_ttk_button( -text("Push Me") )?;
    txt.window_create( Index::line_char(1,0), -window(b) )?;

    Ok( main_loop() )
}

