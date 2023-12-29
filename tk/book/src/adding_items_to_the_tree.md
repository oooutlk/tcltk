# Adding Items to the Tree

To do anything useful with the treeview, we'll need to add one or more items to
it. Each item represents a single node in the tree, whether a leaf node or an
internal node containing other nodes. Items are referred to by a unique id. You
can assign this id when the item is first created, or the widget can
automatically generate one.

Items are created by inserting them into the tree, using the treeview's `insert`
method. To insert an item, we need to know where to insert it. That means
specifying the parent item and where within the list of the parent's existing
children the new item should be inserted.

The treeview widget automatically creates a root node (which is not displayed).
Its id is the empty string. It serves as the parent of the first level of items
that are added. Positions within the list of a node's children are specified by
index (0 being the first, and `Index::End` meaning insert after all existing
children).

Normally, you'll also specify the name of each item, which is the text displayed
in the tree. Other options allow you to add an image beside the name, specify
whether the node is open or closed, etc.

Inserting the item returns the id of the newly created item.

```rust,no_run
// cargo run --example adding_items_to_the_tree

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

    Ok( main_loop() )
}
```
