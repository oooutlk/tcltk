# Tags

We've seen that every canvas item can be referred to by a unique id number.
There is another handy and powerful way to refer to items on a canvas, using
tags.

A tag is just an identifier of your creation, something meaningful to your
program. You can attach tags to canvas items; each item can have any number of
tags. Unlike item id numbers, which are unique for each item, many items can
share the same tag.

What can you do with tags? We saw that you can use the item id to modify a
canvas item (and we'll see soon there are other things you can do to items, like
move them around, delete them, etc.). Any time you can use an item id, you can
use a tag. For example, you can change the color of all items having a specific
tag.

Tags are a good way to identify collections of items in your canvas (items in a
drawn line, items in a palette, etc.). You can use tags to correlate canvas
items to particular objects in your application (for example, tag all canvas
items that are part of the robot with id #X37 with the tag "robotX37"). With
tags, you don't have to keep track of the ids of canvas items to refer to groups
of items later; tags let Tk do that for you.

You can assign tags when creating an item using the `tags` item configuration
option. You can add tags later with the `addtag` method or remove them with the
`dtags` method. You can get the list of tags for an item with the `gettags`
method or return a list of item id numbers having the given tag with the `find`
command.

For example:

```rust,no_run
// cargo run --example canvas_tags

use tk::*;
use tk::canvas::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root.add_canvas(())?.pack(())?;
    let _tag1 = canvas.create_line( &[ (10.0,10.0), (20.0,20.0) ], -tags("firstline drawing") )?;
    let tag2 = canvas.create_rectangle( 30.0, 30.0, 40.0, 40.0, -tags("firstline drawing") )?;
    canvas.addtag( "rectangle", SearchSpec::WithTag( tag2.clone().into() ))?;
    canvas.addtag( "polygon", SearchSpec::WithTag( item_tag( "drawing" ).into() ))?;

    let tags = canvas.gettags( tag2.clone() )?;
    for name in &[ "drawing", "rectangle", "polygon" ] {
        assert!( tags.iter().find( |&tag| tag.0.as_str() == *name ).is_some() );
    }

    canvas.dtag( tag2.clone(), Some( ItemTag( "polygon".to_owned() )))?;

    let tags = canvas.gettags( tag2.clone() )?;
    for name in &[ "drawing", "rectangle" ] {
        assert!( tags.iter().find( |&tag| tag.0.as_str() == *name ).is_some() );
    }
    assert!( tags.iter().find( |&tag| tag.0.as_str() == "polygon" ).is_none() );

    let items = canvas.find( SearchSpec::WithTag( item_tag( "drawing" ).into() ))?;

    assert_eq!(
        items.get_elements()?.map( |item| item.get_string() ).collect::<Vec<_>>(),
        vec![ "1".to_owned(), "2".to_owned() ]);

    Ok( main_loop() )
}
```

As you can see, methods like `withtag` accept either an individual item or a
tag; in the latter case, they will apply to all items having that tag (which
could be none). The `addtag` and `find` methods have many other options,
allowing you to specify items near a point, overlapping a particular area, etc.

Let's use tags first to put a border around whichever item in our color palette
is currently selected.

```rust,no_run
// cargo run --example canvas_a_simple_sketchpad_border_around_selected

use std::os::raw::c_double;
use tcl::*;
use tk::*;
use tk::canvas::*;
use tk::cmd::*;

fn set_color<Inst:TkInstance>( tk: &Tk<Inst>, canvas: &TkCanvas<Inst>, color: &Obj ) -> TkResult<()> {
    tk.set( "color", color.clone() );
    canvas.dtag( item_tag( "all" ), Some( item_tag( "paletteSelected" )))?;
    canvas.itemconfigure( item_tag( "palette" ), -outline("white") )?;
    canvas.addtag( "paletteSelected", SearchSpec::WithTag( item_tag( &format!( "palette{}", color.clone() )).into() ))?;
    canvas.itemconfigure( item_tag( "paletteSelected" ), -outline("#999999") )?;
    Ok(())
}

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root
        .add_canvas(())?
        .grid( -sticky("nwes") -column(0i32) -row(0i32) )?;
    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure( 0, -weight(1) )?;
    Widget::bind( &canvas, event::button_press_1(), "set lastx %x; set lasty %y" )?;
    Widget::bind( &canvas, event::button_1().motion(), tclosure!( tk,
        |evt_x:c_double, evt_y:c_double| -> TkResult<()> {
            let last_x = tk.get_double("lastx")?;
            let last_y = tk.get_double("lasty")?;
            let color = tk.get("color")?;
            set_color( &tk, &canvas, &color )?;
            canvas.create_line( &[ (last_x,last_y), (evt_x,evt_y) ], -fill(color) )?;
            tk.set( "lastx", evt_x );
            tk.set( "lasty", evt_y );
            Ok(())
        }
    ))?;

    let id = canvas.create_rectangle( 10.0, 10.0, 30.0, 30.0,
        -fill("red") -tags("palette palettered") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, || { tk.set( "color", "red" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 35.0, 30.0, 55.0,
        -fill("blue") -tags("palette paletteblue") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, || { tk.set( "color", "blue" ); Ok(()) }))?;

    let id = canvas.create_rectangle( 10.0, 60.0, 30.0, 80.0,
        -fill("black") -tags("palette paletteblack paletteSelected") )?;
    canvas.bind( id,
        event::button_press_1(),
        tclosure!( tk, || { tk.set( "color", "black" ); Ok(()) }))?;

    set_color( &tk, &canvas, &Obj::from("black") )?;
    canvas.itemconfigure( item_tag( "palette" ), -width(5) )?;

    Ok( main_loop() )
}
```

The canvas `itemconfigure` method provides another way to change the properties
of a canvas item. The advantage over dealing with the canvas item object
directly is that we can specify a tag, so that the change we're making applies
to all items having that tag. Without this, we could use `gettags` to get all
the items, iterate through them, and set the option, but `itemconfigure` is more
convenient.

Let's also use tags to make the current stroke being drawn appear more
prominent. When the mouse button is released, we'll return the line to normal.

```rust,no_run
Widget::bind( &canvas, event::button_1().motion(), tclosure!( tk,
    |evt_x:c_double, evt_y:c_double| -> TkResult<()> {
        // ...
        canvas.create_line( &[ (last_x,last_y), (x,y) ],
            -fill(color) -width(5) -tags("currentline") )?;

        tk.set( "lastx", x );
        tk.set( "lasty", y );
        // ...
        Ok(())
    }
))?;

Widget::bind(
    &canvas,
    event::button_1().button_pelease(),
    tclosure!( tk, || ->TkResult<()> {
        Ok( canvas.itemconfigure( item_tag( "currentline" ), -width(1) )? )
    })
)?;
```

## Run Example

* `cargo run --example canvas_tags`
* `cargo run --example canvas_a_simple_sketchpad_border_around_selected`
* `cargo run --example canvas_a_simple_sketchpad_more_prominent`
