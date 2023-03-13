// `cargo run --example contextual_menus`

use std::ffi::c_int;
use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let menu = root.add_menu(())?;
    for i in ["One", "Two", "Three"] {
        menu.add_command( -label(i) )?;
    }

    tclosure!( tk, cmd: "handler", args: "%X %Y",
            move |x: c_int, y: c_int| -> TkResult<()> {
                Ok( tk.popup( menu, x, y, None )? )
            }
    );

    use event::*;
    if tk.windowing_system()? == TkWindowingSystem::Aqua {
        root.bind( button_press_2(), "handler" )?;
        root.bind( control().button_press_1(), "handler" )?;
    } else {
        root.bind( button_press_3(), "handler" )?;
    }

    Ok( main_loop() )
}
