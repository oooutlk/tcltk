# Even More

Text widgets can do many more things. Here, we'll briefly mention just a few
more of them. For details on any of these, see the reference manual.

## Search

The text widget includes a powerful `search` method to locate a piece of text
within the widget. This is useful for a "Find" dialog, as one obvious example.
You can search backward or forward from a particular position or within a given
range, specify the search term using exact text, case insensitive, or via
regular expressions, find one or all occurrences of the search term, etc.

## Modifications, Undo and Redo

The text widget keeps track of whether changes have been made (useful to know
whether it needs to be saved to a file, for example). We can query (or change)
using the `set_edit_modified( modified )` method. There is also a complete
multi-level undo/redo mechanism, managed automatically by the widget when we set
its `undo` configuration option to `true`. Calling `edit_undo` or `edit_redo`
modifies the current text using information stored on the undo/redo stack.

## Eliding Text

Text widgets can include text that is not displayed. This is known as "elided"
text, and is made available using the `elide` configuration option for tags. It
can be used to implement an outliner, a "folding" code editor, or even to bury
extra meta-data intermixed with displayed text. When specifying positions within
elided text, you have to be a bit more careful. Methods that work with positions
have extra options to either include or ignore the elided text.

## Introspection

Like most Tk widgets, the text widget goes out of its way to expose information
about its internal state. We've seen this in terms of the `get` method, widget
configuration options, `names` and `cget` for both tags and marks, etc. There is
even more information available that can be useful for a wide variety of tasks.
Check out the `debug`, `dlineinfo`, `bbox`, `count`, and `dump` methods in the
reference manual.

## Peering

The Tk text widget allows the same underlying text data structure (containing
all the text, marks, tags, images, etc.) to be shared between two or more
different text widgets. This is known as peering and is controlled via
`peer_create` and `peer_names` methods.
