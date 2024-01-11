# Creating Items

When you create a new canvas widget, it is essentially a large rectangle with
nothing on it, truly a blank canvas, in other words. To do anything useful with
it, you'll need to add items to it. There are a wide variety of different types
of items you can add. Here, we'll add a simple line item to the canvas.

To create a line, you need to specify its starting and ending coordinates.
Coordinates are expressed as the number of pixels away from the top-left corner,
horizontally and vertically, i.e. (x,y). The pixel at the top-left corner, known
as the origin, has coordinates (0,0). The "x" value increases as you move to the
right, and the "y" value increases as you move down. A line is described by two
points, which we'd refer to as (x1,y1) and (x2,y2). This code creates a line
from (10,5) to (200,50):

```rust,no_run
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root.add_canvas(())?.pack(())?;
    canvas.create_line( &[ (10.0,5.0), (200.0,50.0) ], () )?;
    Ok( main_loop() )
}
```

The `create_line` command returns an item id (an integer) that uniquely refers
to this item. We'll see how it can be used shortly. Often, we don't need to
refer to the item later and can ignore the returned id.

## A Simple Sketchpad

Let's start our simple sketchpad example. For now, we'll implement freehand
drawing on the canvas with the mouse. We create a canvas widget and attach event
bindings to it to capture mouse clicks and drags. When the mouse is first
pressed, we'll remember that location as the "start" of our next line. As the
mouse is moved with the mouse button held down, we create a line item from this
"start" position to the current mouse location. This current location becomes
the "start" position for the next line item. Every mouse drag creates a new line
item.

```rust,no_run
// cargo run --example canvas_a_simple_sketchpad

use std::os::raw::c_double;
use tcl::*;
use tk::*;
use tk::cmd::*;

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
            canvas.create_line( &[ (last_x,last_y), (evt_x,evt_y) ], () )?;
            tk.set( "lastx", evt_x );
            tk.set( "lasty", evt_y );
            Ok(())
        }
    ))?;
    Ok( main_loop() )
}
```

## Item Attributes

When creating items, you can also specify one or more item attributes, affecting
how it appears. For example, we can specify that the line should be red and
three pixels wide.

```rust,no_run
canvas.create_line( &[ (10.0,10.0), (200.0,50.0) ], -fill("red")-width(3) )?;
```

The exact set of attributes will vary according to the type of item. Some
commonly used ones are:

|attribute|description|
|--------:|:----------|
|`fill`|color to draw the object|
|`width`|line width of the item (or its outline)                |
|`outline`|for filled shapes like rectangles, the color to draw the item's outline|
|`dash`|draw a dashed line instead of a solid one, e.g., 2 4 6 4 alternates short (2 pixels) and long (6 pixels) dashes with 4 pixels between|
|`stipple`|instead of a solid fill color, use a pattern, typically gray75, gray50, gray25, or gray12; stippling is currently not supported on macOS|
|`state`|assign a state of `normal`(default), `disabled`(item event bindings are ignored), or `hidden`(removed from display)|
|`disabledfill`, `disabledwidth`, ...| if the item's `state` is set to `disabled`, the item will display using these variants of the usual attributes|
|`activefill`, `activewidth`, ...| when the mouse pointer is over the item, it will display using these variants of the usual attributes|

> If you have canvas items that change state, creating the item with both the
regular and `disabled*` attribute variants can simplify your code. You simply
need to change the item's `state` rather than writing code to change multiple
display attributes. The same applies to the `active*` attribute variants. Both
encourage a more declarative style that can remove a lot of boilerplate code.

Just like with Tk widgets, you can change the attributes of canvas items after
they're created.

```rust,no_run
let id = canvas.create_line( &[ (0.0,0.0), (10.0,10.0) ], -fill("red") )?;
canvas.itemconfigure( id.into(), -fill("blue") -width(2) )?;
```

## Item Types

Canvas widgets support a wide variety of item types.

### Line

Our sketchpad created simple line items, each a single segment with a start
point and an end point. Lines items can also consist of multiple segments.

Lines have several interesting additional attributes, allowing for drawing
curves, arrows, and more.

|attribute|description|
|--------:|:----------|
|`arrow`|place an arrowhead at the start(`first`), end(`last`), or both ends(`both`); default is `none`|
|`arrowshape`|allows changing the appearance of any arrowheads|
|`capstyle`|for wide lines without arrowheads, this controls how the end of lines are drawn; one of `butt`(default), `projecting`, or `round`|
|`joinstyle`|for wide lines with multiple segments, this controls drawings of each vertex; one of `round` (default), `bevel`, or `miter`|
|`smooth`|if specified as `true` (or `bezier`), draws a smooth curve (via quadratic splines) between multiple segments rather than using straight lines; `raw` specifies a different type of curve (cubic splines)|
|`splinesteps`|controls the smoothness of curved lines, i.e., those with the `smooth` option set|

### Rectangle

Rectangles are specified by the coordinates of opposing corners, e.g., top-left
and bottom-right. They can be filled in (via `fill`) with one color, and the
`outline` given a different color.

```rust,no_run
canvas.create_rectangle( 10.0, 10.0, 200.0, 50.0, -fill("red") -outline("blue") )?;
```

### Oval

Ovals items work exactly the same as rectangles.

```rust,no_run
canvas.create_oval( 10.0, 10.0, 200.0, 50.0, -fill("red") -outline("blue") )?;
```

### Polygon

Polygon items allow you to create arbitrary shapes as defined by a series of
points. The coordinates are given in the same way as multipoint lines. Tk
ensures the polygon is "closed," attaching the last point to the first if
needed. Like ovals and rectangles, they can have separate `fill` and `outline`
colors. They also support the `joinstyle`, `smooth`, and `splinesteps`
attributes of line items.

```rust,no_run
canvas.create_polygon(
    &[ (10.0,10.0), (200.0,50.0), (90.0,150.0), (50.0,80.0), (120.0,55.0) ],
    -fill("red") -outline("blue") )?;
```

### Arc

Arc items draw a portion of an oval; think of one piece of a pie chart. Its
display is controlled by three attributes:

* `start`: how far along the oval the arc should start, in degrees (0 is the
  3-o'clock position)
* The `extent`: how many degrees "wide" the arc should be, positive for
  counter-clockwise from the start, negative for clockwise
* `style`: one of `pieslice`(the default), `arc`(draws just the outer
  perimeter), or `chord`(draws the area between a line connecting the start and
  end points of the arc and the outer perimeter).

```rust,no_run
canvas.create_arc( 10.0, 10.0, 200.0, 50.0,
    -fill("yellow") -outline("black") -start(45) -extent(135) -width(5) )?;
```

### Widget

One of the coolest things you can do with the canvas widget is embed other
widgets inside it. This can be a lowly button, an entry (think in-place editing
of text items), a listbox, a frame itself containing a complex set of widgets...
anything! Remember when we said way back when that a canvas widget could act as
a geometry manager? This is what we meant.

Canvas items that display other widgets are known as window items (Tk's
longstanding terminology for widgets). They are positioned like text and image
items. You can give them explicit `width` and `height` attributes; they default
to the widget's preferred size. Finally, it's important that the widget you're
placing on the canvas (via the `window`) attribute be a child widget of the
canvas.

```rust,no_run
let button = root.add_ttk_button( -text("Implode!") )?;
canvas.create_window( 10.0, 10.0, -anchor("nw") -window(button) )?;
```

### Modifying Items

We've seen how you can modify the configuration options on an item — its color,
width, etc. There are several other things you can do with items.

To delete items, use the `delete` method.

To change an item's size and position, you can use the `coords` method. You
supply new coordinates for the item, specified the same way as when you first
created it. Calling this method without a new set of coordinates will return the
current coordinates of the item. You can use the `move_` method to offset one or
more items horizontally or vertically from their current position.

All items are ordered from top to bottom in what's called the stacking order. If
an item later in the stacking order overlaps an item below it, the first item
will be drawn on top of the second. The `raise` (`lift` in Tkinter) and `lower`
methods allow you to adjust an item's position in the stacking order.

There are several more operations detailed in the reference manual to modify
items and retrieve information about them.

## Run Example

* `cargo run --example canvas_creating_items`
* `cargo run --example canvas_a_simple_sketchpad`
* `cargo run --example canvas_item_types`
