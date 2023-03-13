# The Basics

If you simply need a multi-line text field for a form, there are only a few
things to worry about: create and size the widget (check), provide an initial
value, and retrieve the text after a user has submitted the form.

## Providing Initial Content

Text widgets start with nothing in them, so we'll need to add any initial
content ourselves. Because text widgets can hold a lot more than plain text, a
simple mechanism (like the entry widget's `textvariable` configuration option)
isn't sufficient.

Instead, we'll use the widget's `insert` method:

```rust,no_run
txt.insert( Index::line_char(1,0), "here is my\ntext to insert" )?;
```

The `Index::line_char(1,0)` here is the position where to insert the text, and
can be read as "line 1, character 0". This refers to the first character of the
first line. Historically, especially on Unix, programmers tend to think about
line numbers as 1-based and character positions as 0-based.

The text to insert is just a string. Because the widget can hold multi-line
text, the string we supply can be multi-line as well. To do this, simply embed
`\n` (newline) characters in the string at the appropriate locations.

## Retrieving the Text

After users have made any changes and submitted the form (for example), your
program can retrieve the contents of the widget via the `get` method:

```rust,no_run
let the_text = txt.get_range( Index::line_char(1,0).. )?;
```

`Index::line_char(1,0)..` indicates the start position is "line 1, character 0"
and the end position is "the end"; You can provide different start and end
positions if you want to obtain only part of the text.  You'll see more on
positions shortly.

## Customizing Appearance

We previously saw the `width` and `height` configuration options for text
widgets. Several other options control its appearance. The most useful are:

|attribute|description|
|--------:|:----------|
|foreground|color to draw the text in|
|background|background color of the widget|
|padx, pady|extra padding along the inside border of the widget|
|borderwidth|width of the border around widget|
|relief|border style: `flat`, `raised`, `sunken`, `solid`, `ridge`, `groove`|

## Wrapping and Scrolling

What if some lines of text in the widget are very long, longer than the width of
the widget? By default, the text wraps around to the next line. This behavior
can be changed with the `wrap` configuration option. It defaults to `char`,
meaning wrap lines at any character. Other options are `word` to wrap lines only
at word breaks (e.g., spaces), and `none` meaning to not wrap lines at all. In
the latter case, some text of longer lines won't be visible unless we attach a
horizontal scrollbar to the widget. (Users can also scroll through the text
using arrow keys, even if scrollbars aren't present).

Both horizontal and vertical scrollbars can be attached to the text widget in
the same way as with other widgets, e.g., canvas, listbox.

```rust,no_run
// cargo run --example text_wrapping_and_scrolling

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let txt = root.add_text( "t"
        -width(40) -height(5) -wrap("none")
        -yscrollcommand(".ys set") -xscrollcommand(".xs set") )?
        .grid( -column(0) -row(0) -sticky("nwes") )?;
    let _xs = root
        .add_ttk_scrollbar( "xs" -orient("horizontal") -command(".t xview") )?
        .grid( -column(0) -row(1) -sticky("we") )?;
    let _ys = root
        .add_ttk_scrollbar( "ys" -orient("vertical") -command(".t yview") )?
        .grid( -column(1) -row(0) -sticky("ns") )?;

    txt.insert( text::Index::end(), "Lorem ipsum...\n...\n... " )?;
    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure( 0, -weight(1) )?;

    Ok( main_loop() )
}
```

We can also ask the widget to ensure that a certain part of the text is visible.
For example, let's say we've added more text to the widget than will fit
onscreen (so it will scroll). However, we want to ensure that the top of the
text rather than the bottom is visible. We can use the `see` method.

```rust,no_run
txt.see( Index::line_char(1,0) )?;
```

## Disabling the Widget

Some forms will temporarily disable editing in particular widgets unless certain
conditions are met (e.g., some other options are set to a certain value). To
prevent users from changing a text widget, set the `state` configuration option
to `disabled`. Re-enable editing by setting this option back to `normal`.

```rust,no_run
txt.configure( -state("disabled") )?;
```

> As text widgets are part of the classic widgets, the usual `state` and
`instate` methods are not available.

## Run Example

* `cargo run --example text_the_basics`
* `cargo run --example text_wrapping_and_scrolling`
