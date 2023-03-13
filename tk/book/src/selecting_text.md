# Selecting Text

We can identify the range of text selected by a user, if any. For example, an editor may have a toolbar button to bold the selected text. While you can tell when the selection has changed (e.g., to update whether or not the bold button is active) via the `Selection` virtual event, that doesn't tell you what has been selected.

The text widget automatically maintains a tag named `sel`, which refers to the selected text. Whenever the selection changes, the `sel` tag will be updated. So we can find the range of text selected using the `tag_ranges tag` method, passing it `sel` as the tag to report on.

Similarly, we can change the selection by using `tag_add` to set a new
selection, or `tag_remove` to remove the selection. The sel tag cannot be
deleted, however.

> Though the default widget bindings prevent this from happening, `sel` is like
any other tag in that it can support multiple ranges, i.e., disjoint selections.
To prevent this from happening, when changing the selection from your code, make
sure to remove any old selection before adding a new one.
