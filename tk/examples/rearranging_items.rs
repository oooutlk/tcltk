// cargo run --example rearranging_items

use tk::*;
use tk::ttk_treeview::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let tree = root.add_ttk_treeview(())?.pack(())?;

    // Inserted at the root, program chooses id:
    tree.insert( "", Index::End, "widgets" -text("Widget Tour") )?;

    // Same thing, but inserted as first child:
    tree.insert( "", 0, "gallery" -text("Applications") )?;

    // Treeview chooses the id:
    let id = tree
        .insert( "", Index::End, -text("Tutorial") )?
        .unwrap();

    // Inserted underneath an existing node:
    tree.insert( "widgets", Index::End, -text("Canvas") )?;
    tree.insert( &id, Index::End, -text("Tree") )?;

    // move widgets under gallery
    //tree.move_item( "widgets", "gallery", Index::End )?;

    //tree.delete( &[ "widgets" ])?;

    tree.set_item( "widgets", -open("true") )?;
    let is_open = tree.item( "widgets", open )?;
    println!( "is_open:{}", is_open );

    Ok( main_loop() )
}

