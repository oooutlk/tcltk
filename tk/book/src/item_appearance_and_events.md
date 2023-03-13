# Item Appearance and Events

Like the text and canvas widgets, the treeview widget uses tags to modify the
appearance of items in the tree. We can assign a list of tags to each item using
the `tags` item configuration option (again, when creating the item or later
on).

Configuration options can then be specified on the tag, applied to all items
having that tag. Valid tag options include `foreground` (text color),
`background`, `font`, and `image` (not used if the item specifies its own
image).

We can also create event bindings on tags to capture mouse clicks, keyboard
events, etc.

```rust,no_run
tree.insert( "", Index::End, -text("button") -tags("ttk simple") )?;
tree.tag_configure( "ttk", -background("yellow") )?;
tree.tag_bind( "ttk", event::button_1(), item_clicked )?;
// the item clicked can be found via tree.focus()
```

The treeview will generate virtual events `TreeviewSelect`, `TreeviewOpen`, and
`TreeviewClose`, which allow us to monitor changes to the widget made by users.
We can use the `selection` method to determine the current selection (the
selection can also be changed from your program).

## Run Example

`cargo run --example item_appearance_and_events`
