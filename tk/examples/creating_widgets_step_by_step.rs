// cargo run --example creating_widgets_step_by_step

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let content = root
        .add_frame(())? // auto-generated name
        .pack(())?; // make visible
    let _label = content
        .add_label( "lbl" -text("step by step") )? // named "lbl"
        .pack(())?; // make visible
    let _button = content
        .add_button( "btn" -text("quit") -command("destroy .") )? // named "btn"
        .pack(())?; // make visible
    Ok( main_loop() )
}
