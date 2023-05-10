# Rearranging Items

A node (and its descendants, if any) can be moved to a different location in the
tree. The only restriction is that a node cannot be moved underneath one of its
descendants for obvious reasons. As before, the target location is specified via
a parent node and a position within its list of children.

```rust,no_run
// move widgets under gallery
tree.move_item( "widgets", "gallery", Index::End )?;
```

Items can be detached from the tree. This removes the item and its descendants
from the hierarchy but does not destroy the items. This allows us to later
reinsert them with `move_item`.

Items can also be deleted, which does completely destroy the item and its
descendants.

```rust,no_run
tree.delete( &[ "widgets" ])?;
```

To traverse the hierarchy, there are methods to find the parent of an item
(`parent_item`), its next or previous sibling (`next_item` and `prev_item`), and
return the list of `children` of an item.

We can control whether or not the item is open and shows its children by
modifying the `open` item configuration option.

```rust,no_run
tree.set_item( "widgets", -open("true") )?;
let is_open = tree.item( "widgets", open )?;
```

## Run Example

`cargo run --example rearranging_items`
