# Manipulating Styles

In this section, we'll see how to change the style's appearance by modifying
style options. You can do this either by modifying an existing style, or more
typically, by creating a new style. We saw how to create a simple style that was
derived from another one earlier:

```rust,no_run
let emergency_tbutton_style = tk.new_ttk_style( "Emergency.TButton", None );
emergency_tbutton_style
    .configure( -font("helvetica 24") -foreground("red") -padding(10) )?;
```

## Modifying a Style Option

Modifying an option for an existing style is done similarly to modifying any
other configuration option, by specifying the style, name of the option, and new
value:

```rust,no_run
tbutton_style.configure( -font("helvetica 24") )?;
```

You'll learn more about what the valid options are shortly.

> If you modify an existing style, like we've done here with `TButton`, that
modification will apply to all widgets using that style (by default, all
buttons). That may well be what you want to do.

To retrieve the current value of an option, use the `lookup` method:

```rust,no_run
println!( "{}", tbutton_style.lookup_normal( font )? ); // "helvetica 24"
```

## State Specific Style Options

Besides the normal configuration options for the style, the widget author may
have specified different options to use when the widget is in a particular
widget state. For example, when a button is disabled, it may change the button's
label to grey.

> Remember that the state is composed of one or more state flags (or their
negation), as set by the widget's `state` method or queried via the `instate`
method.

You can specify state-specific variations for one or more of a style's
configuration options with a map. For each configuration option, you can specify
a list of widget states, along with the value that option should be assigned
when the widget is in that state.

The following example provides for the following variations from a button's
normal appearance:

* when the widget is in the disabled state, the background color should be set
  to `#d9d9d9`

* when the widget is in the active state (mouse over it), the background color
  should be set to `#ececec`

* when the widget is in the disabled state, the foreground color should be set
  to `#a3a3a3` (this is in addition to the background color change we already
  noted)

* when the widget is in the state where the button is pressed, and the widget is
  not disabled, the relief should be set to `sunken`

```rust,no_run
tbutton_style.map(
    -background([ "disabled", "#d9d9d9", "active", "#ececec" ].as_slice())
    -foreground([ "disabled", "#a3a3a3" ].as_slice())
    -relief([ "pressed !disabled", "sunken" ].as_slice()))?;
```

> Because widget states can contain multiple flags, more than one state may
match an option (e.g., `pressed` and `pressed` `!disabled` will both match if
the widget's `pressed` state flag is set). The list of states is evaluated in
the order you provide in the map command. The first state in the list that
matches is used.
