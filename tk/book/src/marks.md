# Marks

Marks indicate a particular place in the text. In that respect, they are like indices. However, as the text is modified, the mark will adjust to be in the same relative location. In that way, they resemble tags but refer to a single position rather than a range of text. Marks actually don't refer to a position occupied by a character in the text but specify a position between two characters.

Tk automatically maintains two different marks. The first, named `insert`, is the present location of the insertion cursor. As the cursor is moved (via mouse or keyboard), the mark moves with it. The second mark, named `current`, tracks the position of the character underneath the current mouse position.

To create your own marks, use the widget's `mark_set( name, index)` method, passing it the name of the mark and an index (the mark is positioned just before the character at the given index). This is also used to move an existing mark to a different position. Marks can be removed using the `mark_unset( name )` method, passing it the name of the mark. If you delete a range of text containing a mark, that also removes the mark.

The name of a mark can also be used as an index (in the same way
`Index::line_char(1,0)` or `Index::end().chars(-1)` are indices). You can find
the next mark (or previous one) from a given index in the text using the
`mark_next( index )` or `mark_previous( index )` methods. The `mark_names`
method will return a list of the names of all marks.

Marks also have a gravity, which can be modified with the
`set_mark_gravity( name, direction )` method, which affects what happens when
text is inserted at the mark. Suppose we have the text "ac" with a mark in
between that we'll symbolize with a pipe, i.e., "a|c." If the gravity of that
mark is `TkTextMarkGravity::Right` (the default), the mark attaches itself to
the "c." If the new text "b" is inserted at the mark, the mark will remain stuck
to the "c," and so the new text will be inserted before the mark, i.e., "ab|c."
If the gravity is instead `TkTextMarkGravity::Left`, the mark attaches itself to
the "a," and so new text will be inserted after the mark, i.e., "a|bc."
