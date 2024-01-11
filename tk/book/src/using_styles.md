# Using Styles

We'll now tackle the more complex issue of taking full advantage of styles and
themes within your application, not just reskinning it with an existing theme.

## Definitions

We first need to introduce a few essential concepts.

### Widget Class

A widget class identifies the type of a particular widget, whether it is a
button, a label, a canvas, etc. All themed widgets have a default class. Buttons
have the class `TButton`, labels `TLabel`, etc.

### Widget State

A *widget state* allows a single widget to have more than one appearance or
behavior, depending on things like mouse position, different state options set
by the application, and so on.

As you'll recall, all themed widgets maintain a set of binary state flags,
accessed by the `state` and `instate` methods. The flags are: `active`,
`disabled`, `focus`, `pressed`, `selected`, `background`, `readonly`,
`alternate`, `and` `invalid`. All widgets have the same set of state flags,
though they may ignore some of them (e.g., a label widget would likely ignore an
`invalid` state flag). See the
[`themed widget`(https://tcl.tk/man/tcl8.6/TkCmd/ttk_widget.htm) page in the
reference manual for the exact meaning of each state flag.

### Style

A style describes the appearance (or appearances) of a widget class. All themed
widgets having the same widget class will have the same appearance(s).

Styles are referred to by the name of the widget class they describe. For
example, the style `TButton` defines the appearance of all widgets with the
class `TButton`.

Styles know about different states, and one style can define different
appearances based on a widget's state. For example, a style can specify how a
widget's appearance should change if the `pressed` state flag is set.

### Theme

A theme is a collection of styles. While each style is widget-specific (one for
buttons, one for entries, etc.), a theme collects many styles together. All
styles in the same theme will be designed to visually "fit" together with each
other. (Tk doesn't technically restrict bad design or judgment, unfortunately!)

Using a particular theme in an application really means that, by default, the
appearance of each widget will be controlled by the style within that theme
responsible for that widget class.

## Style Names

Every style has a name. If you're going to modify a style, create a new one, or
use a style for a widget, you need to know its name.

How do you know what the names of the styles are? If you have a particular
widget, and you want to know what style it is currently using, you can first
check the value of its `style` configuration option. If that is empty, it means
the widget is using the default style for the widget. You can retrieve that via
the widget's class. For example:

```rust,no_run
let b = root.add_ttk_button(())?;
assert!( b.cget( style )?.is_empty() ); // empty string as a result
assert_eq!( b.winfo_class()?, "TButton" );
```

In this case, the style that is being used is `TButton`. The default styles for
other themed widgets are named similarly, e.g., `TEntry`, `TLabel`, etc.

> It's always wise to check the specifics. For example, the treeview widget's
class is `Treeview`, not `TTreeview`.

Beyond the default styles, though, styles can be named pretty much anything. You
might create your own style (or use a theme that has a style) named `FunButton`,
`NuclearReactorButton`, or even `GuessWhatIAm` (not a wise choice).

More often, you'll find names like `Fun.TButton` or `NuclearReactor.TButton`.
These suggest variations of a base style; as you'll see, this is something Tk
supports for creating and modifying styles.

> The ability to retrieve a list of all currently available styles is currently
not supported. This will likely appear in Tk 8.7 in the form of a new command,
`theme_styles()`, returning the list of styles implemented by a theme. It also
proposes adding a `style` method for all widgets, so you don't have to examine
both the widget's `style` configuration option and its class. See
[TIP #584](https://tip.tcl.tk/584).

## Applying a Style

To use a style means to apply that style to an individual widget. All you need
is the style's name and the widget to apply it to. Setting the style can be done
at creation time:

```rust,no_run
root.add_ttk_button( -text("Hello") -style("Fun.TButton") )?;
```

A widget's style can also be changed later with the `style` configuration
option:

```rust,no_run
b.configure( -style("Emergency.TButton") )?;
```

## Creating a Simple Style

So how do we create a new style like `Emergency.TButton`?

In situations like this, you're creating a new style only slightly different
from an existing one. This is the most common reason for creating new styles.

For example, you want most of the buttons in your application to keep their
usual appearance but have certain "emergency" buttons highlighted differently.
Creating a new style (e.g., `Emergency.TButton`), derived from the base style
(`TButton`), is appropriate.

Prepending another name (`Emergency`) followed by a dot onto an existing style
creates a new style derived from the existing one. The new style will have
exactly the same options as the existing one except for the indicated
differences:

```rust,no_run
some_style.configure( -font("helvetica 24") -foreground("red") -padding(10) )?;
```

As shown earlier, you can then apply that style to an individual button widget
via its style configuration option. Every other button widget would retain its
normal appearance.

How do you know what options are available to change for a given style? That
requires diving a little deeper inside styles.

> You may have existing code using the classic widgets that you'd like to move
to the themed widgets. Most appearance changes made to classic widgets through
configuration options can probably be dropped. For those that can't, you may
need to create a new style, as shown above.

> State-specific appearance changes can be treated similarly. In classic Tk,
several widgets supported a few state changes via configuration options. For
example, setting a button's `state` option to `disabled` would draw it with a
greyed-out label. Some allowed an additional state, active, which represented a
different appearance. You could change the widget's appearance in multiple
states via a set of configuration options, e.g., `foreground`,
`disabledforeground`, and `activeforeground`.

> State changes via configuration options should be changed to use the `state`
method on themed widgets. Configuration options to modify the widget's
appearance in a particular state should be dealt with in the style.

> Classic Tk widgets also supported a very primitive form of styles that you may
encounter. This used the *option database*, a now-obscure front end to X11-style
configuration files.

> In classic Tk, all buttons had the same class (`Button`), all labels had the
same class (`Label`), etc. You could use this widget class both for
introspection and for changing options globally through the option database. It
let you say, for example, that all buttons should have a red background.

> A few classic Tk widgets, including frame and toplevel widgets, let you change
the widget class of a particular widget when it was first created by providing a
`class` configuration option. For example, you could specify that one specific
frame widget had a class of `SpecialFrame`, while others would have the default
class `Frame`. You could use the option database to change the appearance of
just the `SpecialFrame` frames.

> Styles and themes take that simple idea and give it rocket boosters.
