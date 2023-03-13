# Formatting with Tags

So far, we've used text widgets when all the text is in a single font. Now it's
time to add formatting like bold, italic, strikethrough, background colors, font
sizes, and much more. Tk's text widget implements these using a feature called
tags.

Tags are objects associated with the text widget. Each tag is referred to via a
name chosen by the programmer. Each tag has several configuration options. These
are things like fonts and colors that control formatting. Though tags are
objects having state, they don't need to be explicitly created but are
automatically created the first time the tag name is used.

## Adding Tags to Text

Tags can be associated with one or more ranges of text in the widget. As before,
ranges are specified via indices. A single index represents a single character,
and a pair of indices represent a range from the start character to just before
the end character. Tags are added to a range of text using the `tag_add` method.

```rust,no_run
txt.tag_add( "highlightline", vec![ Index::line_char(5,0), Index::line_char(6,0) ] )?;
```

Tags can also be provided when first inserting text. The `insert_with_tags`
method supports an optional parameter containing a list of one or more tags to
add to the text being inserted.

```rust,no_run
log.insert_with_tags( Index::end(), &[
    ( "new material to insert", &["highlightline","recent","warning"] )
])?;
```

As the widget's contents are modified (whether by a user or your program), the
tags will adjust automatically. For example, if we tagged the text "the quick
brown fox" with the tag "nounphrase", and then replaced the word "quick" with
"speedy," the tag still applies to the entire phrase.

## Applying Formatting to Tags

Formatting is applied to tags via configuration options; these work similarly to
configuration options for the entire widget. As an example:

```rust,no_run
txt.tag_configure( "highlightline",
    -background("yellow") -font("TkFixedFont") -relief("raised") )?;
```

Tags support the following configuration options: `background`, `bgstipple`,
`borderwidth`, `elide`, `fgstipple`, `font`, `foreground`, `justify`,
`lmargin1`, `lmargin2`, `offset`, `overstrike`, `relief`, `rmargin`, `spacing1`,
`spacing2`, `spacing3`, `tabs`, `tabstyle`, `underline`, and `wrap`. Check the
reference manual for detailed descriptions of these. The
`tag_cget( tag, option )` method allows us to query the configuration options of
a tag.

Because multiple tags can apply to the same range of text, there is the
possibility of conflict (e.g., two tags specifying different fonts). A priority
order is used to resolve these; the most recently created tags have the highest
priority, but priorities can be rearranged using the
`tag_raise( tag, above_this )` and `tag_lower( tag, below_this )` methods.

## More Tag Manipulations

To delete one or more tags altogether, we can use the `tag_delete( tags)`
method. This also, of course, removes any references to the tag in the text. We
can also remove a tag from a range of text using the `tag_remove( tag, ranges )`
method. Even if that leaves no ranges of text with that tag, the tag object
itself still exists.

The `tag_ranges( tag )` method will return a list of ranges in the text that the
tag has been applied to. There are also `tag_nextrange( tag, range )` and
`tag_prevrange( tag, range )` methods to search forward or backward for the
first such range from a given position.

The `tag_names_all()` method will return a list of all tags currently defined in
the text widget (including those that may not be presently used). The
`tag_names( index )` method will return the list of tags applied to just the
character at the index.

Finally, we can use the first and last characters in the text having a given tag
as indices, the same way we can use `Index::end()` or `Index::line_char(2,5)`.
To do so, just specify `Index::TagFirst( name, _ )` or
`Index::TagLast( name, _ )`.

## Differences between Tags in Canvas and Text Widgets

Both canvas and text widgets support "tags" that can be applied to several
objects, style them, etc. However, canvas and text tags are not the same and
there are substantial differences to take note of.

In canvas widgets, only individual canvas items have configuration options that
control their appearance. When we refer to a tag in a canvas, the meaning of
that is identical to "all canvas items presently having that tag." The tag
itself doesn't exist as a separate object. So in the following snippet, the last
rectangle added will not be colored red. 

```rust,no_run
canvas.itemconfigure( item_tag("important"), -fill("red") )?;
canvas.create_rectangle( 10, 10, 40, 40, -tags("important") )?;
```

In contrast, with text widgets, it's not the individual characters that retain
the state information about appearance, but tags, which are objects in their own
right. So in this snippet, the newly added text will be colored red.

```rust,no_run
txt.insert_with_tags( Index::end(), &[ "first text", &[ "important" ]])?;
txt.tag_configure( "important" -foreground("red") )?;
txt.insert_with_tags( Index::end(), &[ "second text", &[ "important" ]])?;
```
