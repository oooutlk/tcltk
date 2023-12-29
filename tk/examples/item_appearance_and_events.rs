// cargo run --example item_appearance_and_events

use tcl::*;
use tk::*;
use tk::ttk_treeview::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let tree = root.add_ttk_treeview(())?.pack(())?;
    tree.insert( "", Index::End, -text("button") -tags("ttk simple") )?;
    tree.tag_configure( "ttk", -background("yellow") )?;
    let item_clicked = tclosure!( tk, || {
        println!( "item clicked!" );
        Ok(())
    });
    tree.tag_bind( "ttk", event::button_press_1(), item_clicked )?;

    Ok( main_loop() )
}



