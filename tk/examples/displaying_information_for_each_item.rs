// cargo run --example displaying_information_for_each_item

use tk::*;
use tk::ttk_treeview::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let tree = root.add_ttk_treeview( -columns("size modified") )?.pack(())?;
    tree.configure( -columns("size modified owner") )?;
    tree.insert( "", Index::End, "widgets" -text("Widget Tour") )?;

    tree.column_configure( "size", -width(100) -anchor("center") )?;
    tree.heading_configure( "size", -text("Size") )?;

    tree.set_item_at_column( "widgets", "size", "12KB" )?;
    let size = tree.item_at_column( "widgets", "size" )?;
    assert_eq!( size.to_string(), "12KB" );
    tree.insert( "", Index::End,
        -text("Listbox") -values(vec!["15KB","Yesterday","mark"]) )?;

    Ok( main_loop() )
}


