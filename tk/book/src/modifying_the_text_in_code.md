# Modifying the Text in Code

While users can modify the text in the text widget interactively, your program
can also make changes. Adding text is done with the `insert` method, which we
used above to provide an initial value for the text widget.

## Text Positions and Indices

When we specified a position of `Index::line_char( 1, 0 )` (first
line, first character), this was an example of an index. It tells the `insert`
method where to put the new text (just before the first line, first character,
i.e., at the very start of the widget). Indices can be specified in a variety of
ways. We used another one with the `get` method: `Index::end()` means just
past the end of the text. (Why "just past?" Text is inserted right before the
given index, so inserting at `End` will add text to the end of the widget). Note
that Tk will always add a newline at the very end of the text widget.

Here are a few additional examples of indices and how to interpret them:

|attribute|description|
|--------:|:----------|
|`Index::line(3)..`|The newline at the end of line 3|
|`Index::line_char(1,0).chars(3)`|Three characters past the start of line 1|
|`Index::line(2)..Index::end().chars(-1)`|The last character before the new line in line 2|
|`Index::end().chars(-1)`|The newline that Tk always adds at the end of the text|
|`Index::end().chars(-2)`|The actual last character of the text|
|`Index::end().lines(-1)`|The start of the last actual line of text|
|`Index::line_char(2,2).lines(2)`|The third character (index 2) of the fourth line of text|
|`Index::line_char(2,5).linestart()`|The first character of line 2|
|`Index::line_char(2,5).lineend()`|The position of the newline at the end of line 2|
|`Index::line_char(2,5).wordstart()`|First char. of the word with the char. at index 2.5|
|`Index::line_char(2,5).wordend()`|First char. after the word with the char. at index 2.5|

Some additional things to keep in mind:

* An index past the end of the text (e.g., `Index::end().chars(100)` is
  interpreted as `Index::end()`.
* Indices wrap to subsequent lines as needed; e.g.,
  `Index::line_char(1,0).chars(10)` on a line with only five characters will
  refer to a position on the second line.
* Line numbers in indices are interpreted as logical lines, i.e., each line ends
  only at the "\n." With long lines and wrapping enabled, one logical line may
  represent multiple display lines. If you'd like to move up or down a single
  line on the display, you can specify this as, e.g.,
  `Index::line_char(1,0).display_lines(2)`.

To determine the canonical position of an index, use the
`index( &self, index: Index )` method. Pass it any index expression, and it
returns the corresponding index in the form "line.char". For example, to find
the position of the last character (ignoring the automatic newline at the end),
use:

```rust,no_run
txt.index( Index::end() )?;
```

You can compare two indices using the `compare` method, which lets you check for
equality, whether one index is later in the text than the other, etc.

```rust,no_run
if txt.compare( index1, TkCmp::Equal, index2 )? {
    // same position
}
```

## Deleting Text

While the `insert` method adds new text anywhere in the widget, the
`delete( &self, index: Index )` and
`delete_ranges( &self, ranges: Vec<Range<Index>> )` methods removes it. We can
delete either a single character (specified by index) or a range of characters
(specified by start and end indices). In the latter case, characters from (and
including) the start index until just before the end index are deleted (the
character at the end index is not deleted). So if we assume for each of these we
start off with `"abcd\nefgh"` in the text widget:

```rust,no_run
txt.delete( Index::line_char(1,2) )?; // "abd\nefgh"
txt.delete_ranges( vec![ Index::line_char(1,1), Index::line_char(1,2) ])?; // "acd\nefgh"
txt.delete_ranges( vec![ Index::line_char(1,0), Index::line_char(2,0) ])?; // "efgh"
txt.delete_ranges( vec![ Index::line_char(1,2), Index::line_char(2,1) ])?; // "abfgh"
```

There is also a `replace` method that performs a `delete` followed by an
`insert` at the same location.

## Example: Logging Window

Here's a short example using a text widget as an 80x24 logging window for an
application. Users don't edit the text widget at all. Instead, the program
writes log messages to it. We'd like to display more than 24 lines (so no
scrolling). If the log is full, old messages are removed from the top before new
ones are added at the end.

```rust,no_run
// cargo run --example logging_window

use tk::*;
use tk::cmd::*;
use tk::text::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let log = root
        .add_text( -width(80) -height(24) -wrap("none") )?
        .grid(())?;

    let write_to_log = | msg: &str| -> TkResult<()> {
        let index = log.index( Index::end().lines(-1) )?;
        log.configure( -state("normal") )?;
        log.insert( Index::end(), msg )?;
        if log.index( Index::end().chars(-1) )? != Index::line_char(1,0) {
            log.insert( Index::end(), "\n" )?;
        }
        if let Index::LineChar( num_line, _, _ ) = index {
            if num_line > 24 {
                log.delete_ranges( vec![ Index::line_char(1,0)..Index::line_char(num_line-23,0) ])?;
            }
        }
        log.configure( -state("disabled") )?;
        Ok(())
    };

    for c in 'a'..='z' {
        write_to_log( &format!( "{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}", c ))?;
    }

    Ok( main_loop() )
}
```

> Note that because the program placed the widget in a disabled state, we had to
re-enable it to make any changes, even from our program.
