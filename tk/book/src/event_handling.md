# Event Handling

Tk, as with most other user interface toolkits, runs an event loop that receives
events from the operating system. These are things like button presses,
keystrokes, mouse movement, window resizing, and so on.

Generally, Tk takes care of managing this event loop for you. It will figure out
what widget the event applies to (did a user click on this button? if a key was
pressed, which textbox had the focus?), and dispatch it accordingly. Individual
widgets know how to respond to events; for example, a button might change color
when the mouse moves over it, and revert back when the mouse leaves.

> It's critical in event-driven applications that the event loop not be blocked.
The event loop should run continuously, normally executing dozens of steps per
second. At every step, it processes an event. If your program is performing a
long operation, it can potentially block the event loop. In that case, no events
would be processed, no drawing would be done, and it would appear as if your
application is frozen. There are many ways to avoid this happening, mostly
related to the structure of your application. We'll discuss this in more detail
in a later chapter.

## Command Callbacks

You often want your program to handle some event in a particular way, e.g., do
something when a button is pushed. For those events that are most frequently
customized (what good is a button without something happening when you press
it?), the widget will allow you to specify a callback as a widget configuration
option. We saw this in the example with the `command` option of the button.

```rust,no_run
#[proc] fn calculate() { /* omitted */ }

content.add_ttk_button( ".c.calc" -text("Calculate") -command("calculate") )?;
``` 

## Binding to Events

For events that don't have a widget-specific command callback associated with
them, you can use Tk's bind to capture any event, and then (like with callbacks)
execute an arbitrary piece of code.

Here's a (silly) example showing a label responding to different events. When an
event occurs, a description of the event is displayed in the label.

```rust,no_run
// cargo run --example binding_to_events

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;

    let l = tk.root().add_ttk_label( "l" -text("Starting...") )?.grid(())?;

    l.bind( event::enter(), tkbind!( tk, || l.configure( -text("Moved mouse inside") )))?;

    l.bind( event::leave(), tkbind!( tk, || l.configure( -text("Moved mouse outside") )))?;

    l.bind( event::button_press_1(), tkbind!( tk, || l.configure( -text("Clicked left mouse button") )))?;

    l.bind( event::button_press_3(), tkbind!( tk, || l.configure( -text("Clicked right mouse button") )))?;

    l.bind( event::double().button_press_1(), tkbind!( tk, || l.configure( -text("Double clicked") )))?;

    l.bind( event::button_3().motion(), tkbind!( tk, |evt_rootx, evt_rooty| -> TkResult<()> {
        Ok( l.configure( -text( format!( "right button drag to {evt_rootx} {evt_rooty}" )))? )
    }))?;

    Ok( main_loop() )
}
```

The first two bindings are pretty straightforward, just watching for simple
events. An `event::enter()` event means the mouse has moved over top the widget,
while the `event::Leave()` event is generated when the mouse moves outside the
widget to a different one.

The next binding looks for a mouse click, specifically a `event::button_press_1`
event. Here, the `button_press` is the actual event, but the `_1` is an event
detail specifying the left (main) mouse button on the mouse. The binding will
only trigger when a `button_press` event is generated involving the main mouse
button. If another mouse button was clicked, this binding would ignore it.

This next binding looks for a `event::button_press_3` event. It will respond to
events generated when the right mouse button is clicked. The next binding,
`event::double().button_press_1()` adds another modifier, Double, and so will
respond to the left mouse button being double clicked.

The last binding also uses a modifier: capture mouse movement (Motion), but only
when the right mouse button `button_3` is held down. This binding also shows an
example of how to use event parameters. Many events, such as mouse clicks or
movement carry additional information like the current position of the mouse. Tk
provides access to these parameters in Tcl callback scripts through the use of
percent substitutions. These percent substitutions let you capture them so they
can be used in your script.

## Multiple Bindings for an Event

We've just seen how event bindings can be set up for an individual widget. When
a matching event is received by that widget, the binding will trigger. But
that's not all you can do.

Your binding can capture not just a single event, but a short sequence of
events. The `event::double().button_press_1()` binding triggers when two mouse
clicks occur in a short time. You can do the same thing to capture two keys
pressed in a row, e.g., `key_press( TkKey::A ).key_press( TkKey::B )`.

You can also set up an event binding on a toplevel window. When a matching event
occurs anywhere in that window, the binding will be triggered. In our example,
we set up a binding for the Return key on the main application toplevel window.
If the Return key was pressed when any widget in the toplevel window had the
focus, that binding would fire.

Less commonly, you can create event bindings that are triggered when a matching
event occurs anywhere in the application, or even for events received by any
widget of a given class, e.g., all buttons.

> More than one binding can fire for an event. This keeps event handlers concise
and limited in scope, meaning more modular code. For example, the behavior of
each widget class in Tk is itself defined with script-level event bindings.
These stay separate from event bindings in your application. Event bindings can
also be changed or deleted. They can be modified to alter event handling for
widgets of a certain class or parts of your application. You can reorder,
extend, or change the sequence of event bindings that will be triggered for each
widget; see the bindtags command reference if you're curious.

## Available Events

The most commonly used events are described below, along with the circumstances
when they are generated. Some are generated on some platforms and not others.
For a complete description of all the different event names, modifiers, and the
different event parameters that are available with each, the best place to look
is the bind command reference.

| event name       | description                            |
| :--------------- | :------------------------------------- |
| `activate`       | Window has become active.              |
| `deactivate`     | Window has been deactivated.           |
| `mouse_wheel`    | Scroll wheel on mouse has been moved.  |
| `key_press`      | Key on keyboard has been pressed down. |
| `key_release`    | Key has been released.                 |
| `button_press`   | A mouse button has been pressed.       |
| `button_release` | A mouse button has been released.      |
| `motion`         | Mouse has been moved.                  |
| `configure`      | Widget has changed size or position.   |
| `destroy`        | Widget is being destroyed.             |
| `focus_in`       | Widget has been given keyboard focus.  |
| `focus_out`      | Widget has lost keyboard focus.        |
| `enter`          | Mouse pointer enters widget.           |
| `leave`          | Mouse pointer leaves widget.           |

Event detail for mouse events are the button that was pressed, e.g. `1`, `2`, or
`3`. For keyboard events, it's the specific key, e.g. `A`, `9`, `space`, `plus`,
`comma`, `equal`. A complete list can be found in the keysyms command reference.

Event modifiers for include, e.g. `button_1` to signify the main mouse button
being held down, `double` or `triple` for sequences of the same event. Key
modifiers for when keys on the keyboard are held down inline `control`, `shift`,
`alt`, `option`, and `command`.

## Virtual Events

The events we've seen so far are low-level operating system events like mouse
clicks and window resizes. Many widgets also generate higher level or semantic
events called virtual events. These are indicated by `event::virtual_event()`,
e.g., `event::virtual_event( "foo" )`.

For example, a listbox widget will generate a `event::listbox_select()`
virtual event whenever its selection changes. The same virtual event is
generated whether a user clicked on an item, moved to it using the arrow keys,
or some other way. Virtual events avoid the problem of setting up multiple,
possibly platform-specific event bindings to capture common changes. The
available virtual events for a widget will be listed in the documentation for
the widget class.

Tk also defines virtual events for common operations that are triggered in
different ways for different platforms. These include `event::cut()`,
`event::copy()` and `event::paste()`.

You can define your own virtual events, which can be specific to your
application. This can be a useful way to keep platform-specific details isolated
in a single module, while you use the virtual event throughout your application.
Your own code can generate virtual events that work in exactly the same way that
virtual events generated by Tk do.

```rust,no_run
root.event_generate( event::virtual_event( "MyOwnEvent" ))?;
```
