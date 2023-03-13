# Displaying Information for each Item

The treeview can display one or more additional pieces of information about each
item. These are shown as columns to the right of the main tree display.

Each column is referenced by a symbolic name that we assign. We can specify the
list of columns using the `columns` configuration option of the treeview widget,
either when first creating the widget or later on.

```rust,no_run
root.add_ttk_treeview( -columns("size modified") )?;
tree.configure( -columns("size modified owner") )?;
```

We can specify the width of the column, how the display of item information in
the column is aligned, and more. We can also provide information about the
column's heading, such as the text to display, an optional image, alignment, and
a script to invoke when the item is clicked (e.g., to sort the tree).

```rust,no_run
tree.column_configure( "size", -width(100) -anchor("center") )?;
tree.heading_configure( "size", -text("Size") )?;
```

What to display in each column for each item can be specified individually by
using the `set_item_at_column` method. You can also provide a list describing
what to display in all the columns for the item. This is done using the `values`
item configuration option. It takes a list of values and can be provided when
first inserting the item or changed later. The order of the list must be the
same as the order in the `columns` widget configuration option.

```rust,no_run
tree.set_item_at_column( "widgets", "size", "12KB" )?;
let size = tree.item_at_column( "widgets", "size" )?;
assert_eq!( size.to_string(), "12KB" );
tree.insert( "", Index::End,
    -text("Listbox") -values(&["15KB","Yesterday","mark"]) )?;
```

## Run Example

`cargo run --example displaying_information_for_each_item`
