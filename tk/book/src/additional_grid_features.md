# Additional Grid Features

If you look at the [documentation](https://www.tcl.tk/man/tcl/TkCmd/grid.html)
for `grid`, you'll see many other things you can do with grid. Here are a few of
the more useful ones.

## Querying and Changing Grid Options

Like widgets themselves, it's easy to introspect the various grid options or
change them. Setting options when you first grid the widget is certainly
convenient, but you can change them anytime you'd like.

The `slaves` method will tell you all the widgets that have been gridded inside
a master, or optionally those within just a certain column or row. The `info`
method will return a list of all the grid options for a widget and their values.
Finally, the `configure` method lets you change one or more grid options on a
widget.

These are illustrated in this interactive session:

```rust,no_run
let got = c.grid_slaves(())?;
let got = got
    .iter()
    .map( |widget| widget.path() )
    .collect::<Vec<_>>();
let expected = vec![ ".c.cancel", ".c.ok", ".c.three", ".c.two", ".c.one", ".c.name", ".c.namelbl", ".c.f" ]
    .into_iter()
    .collect::<Vec<_>>();
assert_eq!( got, expected );

let got = c.grid_slaves( -row(3) )?;
let got = got
    .iter()
    .map( |widget| widget.path() )
    .collect::<Vec<_>>();
let expected = vec![ ".c.cancel", ".c.ok", ".c.three", ".c.two", ".c.one" ]
    .into_iter()
    .collect::<Vec<_>>();
assert_eq!( got, expected );

let got = c.grid_slaves( -column(0) )?;
let got = got
    .iter()
    .map( |widget| widget.path() )
    .collect::<Vec<_>>();
let expected = vec![ ".c.one", ".c.f" ]
    .into_iter()
    .collect::<Vec<_>>();
assert_eq!( got, expected );

let got = c_namelbl.grid_info()?
    .into_iter()
    .map( |(key, val)| (key, val.get_string() ))
    .collect::<HashMap<_,_>>();

let mut expected = HashMap::new();

expected.insert( "-in"        .to_owned(), ".c".to_owned() );
expected.insert( "-column"    .to_owned(), "3" .to_owned() );
expected.insert( "-row"       .to_owned(), "0" .to_owned() );
expected.insert( "-columnspan".to_owned(), "2" .to_owned() );
expected.insert( "-rowspan"   .to_owned(), "1" .to_owned() );
expected.insert( "-ipadx"     .to_owned(), "0" .to_owned() );
expected.insert( "-ipady"     .to_owned(), "0" .to_owned() );
expected.insert( "-padx"      .to_owned(), "5" .to_owned() );
expected.insert( "-pady"      .to_owned(), "0" .to_owned() );
expected.insert( "-sticky"    .to_owned(), "nw".to_owned() );

assert_eq!( got, expected );

c_namelbl.grid_configure( -sticky("ew") )?;

let got = c_namelbl.grid_info()?
    .into_iter()
    .map( |(key, val)| (key, val.get_string() ))
    .collect::<HashMap<_,_>>();

let mut expected = HashMap::new();

expected.insert( "-in"        .to_owned(), ".c".to_owned() );
expected.insert( "-column"    .to_owned(), "3" .to_owned() );
expected.insert( "-row"       .to_owned(), "0" .to_owned() );
expected.insert( "-columnspan".to_owned(), "2" .to_owned() );
expected.insert( "-rowspan"   .to_owned(), "1" .to_owned() );
expected.insert( "-ipadx"     .to_owned(), "0" .to_owned() );
expected.insert( "-ipady"     .to_owned(), "0" .to_owned() );
expected.insert( "-padx"      .to_owned(), "5" .to_owned() );
expected.insert( "-pady"      .to_owned(), "0" .to_owned() );
expected.insert( "-sticky"    .to_owned(), "ew".to_owned() );

assert_eq!( got, expected );
```

## Internal Padding

You saw how the `padx` and `pady` grid options added extra space around the
outside of a widget. There's also a less used type of padding called "internal
padding" controlled by the grid options `ipadx` and `ipady`.

The difference can be subtle. Let's say you have a frame that's 20x20, and
specify normal (external) padding of 5 pixels on each side. The frame will
request a 20x20 rectangle (its natural size) from the geometry manager.
Normally, that's what it will be granted, so it'll get a 20x20 rectangle for the
frame, surrounded by a 5-pixel border.

With internal padding, the geometry manager will effectively add the extra
padding to the widget when figuring out its natural size, as if the widget has
requested a 30x30 rectangle. If the frame is centered, or attached to a single
side or corner (using `sticky`), we'll end up with a 20x20 frame with extra
space around it. If, however, the frame is set to stretch (i.e., a `sticky`
value of `we`, `ns`, or `nwes`), it will fill the extra space, resulting in a
30x30 frame, with no border.

## Forget and Remove

The `forget` method of grid removes slaves from the grid they're currently part
of. It takes a list of one or more slave widgets as arguments. This does not
destroy the widget altogether but takes it off the screen as if it had not been
gridded in the first place. You can grid it again later, though any grid options
you'd originally assigned will have been lost.

The `remove` method of grid works the same, except that the grid options will be
remembered if you `grid` it again later.

## Run Example

`cargo run --example querying_and_changing_grid_options`
