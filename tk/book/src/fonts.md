# Fonts

Tk's label widget allows you to change the font used to display text via the
`font` configuration option. The canvas and text widgets, covered in the
following chapters, also allow you to specify fonts. Other themed widgets that
display text may not have a `font` configuration option, but their fonts can be
changed using styles.

> We'll cover styles in detail later. In essence, they replace the old way of
tweaking multiple configuration options on individual widgets. Instead, fonts,
colors, and other settings that control appearance can be bundled together in a
style. That style can then be applied to multiple widgets. It's akin to the
difference between hardcoding display-oriented markup inside HTML pages vs.
using CSS to keep display-specific information separate.

As with many things in Tk, the default fonts are usually a good choice. If you
need to make changes, this section shows you the best way to do so, using named
fonts. Tk includes named fonts suitable for use in all different components of
your user interface. They take into account the conventions of the specific
platform you're running on. You can also specify your own fonts when you need
additional flexibility.

The font command reference provides full details on specifying fonts, as well as
other font-related operations.

> Many older Tk programs hardcoded fonts, using either the "family size style"
format we'll see below, X11 font names, or the older and more arcane X11 font
specification strings. These applications looked increasingly dated as platforms
evolved. Worse, fonts were often specified on a per-widget basis, leaving font
decisions spread throughout the code. Named fonts, particularly the standard
fonts that Tk provides, are a far better solution. Reviewing and updating font
decisions is an easy and important change to make in any existing applications.

## Standard Fonts

Each platform defines specific fonts that should be used for standard user
interface elements. Tk encapsulates many of these decisions into a standard set
of named fonts. They are available on all platforms, though the exact font used
will vary. This helps abstract away platform differences. Of course, the
standard widgets use these named fonts. The predefined fonts are:

`TkDefaultFont`     : Default for items not otherwise specified.

`TkTextFont`        : Used for entry widgets, listboxes, etc.

`TkFixedFont`       : A standard fixed-width font.

`TkMenuFont`        : The font used for menu items.

`TkHeadingFont`     : Font for column headings in lists and tables.

`TkCaptionFont`     : A font for window and dialog caption bars.

`TkSmallCaptionFont`: A smaller caption font for tool dialogs.

`TkIconFont`        : A font for icon captions.

`TkTooltipFont`     : A font for tooltips. 

## Platform-Specific Fonts

Tk provides additional named fonts to help you comply with less common
situations on specific platforms. Individual platform guidelines detail how and
where these fonts should be used. These fonts are only defined on specific
platforms. You'll need to take that into account if your application is portable
across platforms.

Tk on X11 recognizes any valid X11 font name (see, e.g., the `xlsfonts`
command). However, these can vary with the operating system, installed software,
and the configuration of the individual machine. There is no guarantee that a
font available on your X11 system has been installed on any other X11 system.

On Windows, Tk provides named fonts for all the fonts that can be set in the
"Display" Control Panel. It recognizes the following font names: `system`,
`ansi`, `device`, `systemfixed`, `ansifixed`, and `oemfixed`.

On macOS, the Apple Human Interface Guidelines (HIG) specifies a number of
additional fonts. Tk recognizes the following names: `systemSystemFont`,
`systemSmallSystemFont`, `systemApplicationFont`, `systemViewsFont`,
`systemMenuItemFont`, `systemMenuItemCmdKeyFont`, `systemPushButtonFont`,
`systemAlertHeaderFont`, `systemMiniSystemFont`,
`systemDetailEmphasizedSystemFont`, `systemEmphasizedSystemFont`,
`systemSmallEmphasizedSystemFont`, `systemLabelFont`, `systemMenuTitleFont`,
`systemMenuItemMarkFont`, `systemWindowTitleFont`,
`systemUtilityWindowTitleFont`, `systemToolbarFont`, and
`systemDetailSystemFont`.

## Working with Named Fonts

Tk provides several operations that help you work with named fonts. You can
start by getting a list of all the currently defined named fonts.

```rust,no_run
println!( "{:#?}", tk.font_names()? );
```

You can find out the actual system font represented by an abstract named font.
This consists of the `family` (e.g., `Times` or `Helvetica`), the `size` (in
points if positive, in pixels if negative), the `weight` (`normal` or `bold`),
the `slant` (`roman` or `italic`), and boolean attributes for `underline` and
`overstrike`. You can find out the font's `metrics` (how tall characters in the
font can be and whether it is monospaced), and even `measure` how many pixels
wide a piece of text rendered in the font would be.

```rust,no_run
println!( "{:#?}", tk.font_actual_get_all( Font::<()>::Name( "TkTextFont" ))? );
// e.g. -family .AppleSystemUIFont -size 13 -weight normal -slant roman -underline 0 -overstrike 0

println!( "{:#?}", tk.font_metrics_get_all( Font::<()>::Name( "TkTextFont" ))? );
// e.g. -ascent 13 -descent 3 -linespace 16 -fixed 0

println!( "{:#?}", tk.font_measure( Font::<()>::Name( "TkTextFont" ), "The quick brown fox" )? );
// e.g. 124
}
```

You can also create your own fonts, which can then be used exactly like the
predefined ones. To do so, choose a name for the font and specify its font
attributes as above.

```rust,no_run
tk.font_create( "AppHighlightFont", -family("Helvetica") -size(12) -weight("bold") )?;
root.add_ttk_label( "l" -text("Attention!") -font("AppHighlightFont") )?
    .grid(())?;
}
```

The `family` attribute specifies the font family. Tk ensures the names
`Courier`, `Times`, and `Helvetica` are available, though they may be mapped to
an appropriate monospaced, serif, or sans-serif font). Other fonts installed on
your system can be used, but the usual caveats about portability apply. You can
get the names of all available families with: 

```rust,no_run
println!( "{:#?}", tk.font_families()? );
```

## Run Example

`cargo run --example fonts`
