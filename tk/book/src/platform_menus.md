# Platform Menus

Each platform has a few menus in every menubar that are handled specially by Tk.

## macOS

You've probably noticed that Tk on macOS supplies its own default menubar. It
includes a menu named after the program being run (in this case, your
programming language's shell, e.g., "Wish", "Python", etc.), a File menu, and
standard Edit, Windows, and Help menus, all stocked with various menu items.

You can override this menubar in your own program, but to get the results you
want, you'll need to follow some particular steps (in some cases, in a
particular order).

> Starting at Tk 8.5.13, the handling of special menus on macOS changed due to
the underlying Tk code migrating from the obsolete Carbon API to Cocoa. If
you're seeing duplicate menu names, missing items, things you didn't put there,
etc., review this section carefully.

The first thing to know is that if you don't specify a menubar for a window (or
its parent window, e.g., the root window), you'll end up with the default
menubar Tk supplies, which unless you're just mucking around on your own, is
almost certainly not what you want.

### The Application Menu

Every menubar starts with the system-wide apple icon menu. To the right of that
is a menu for the frontmost application. It is always named after the binary
being run. If you do supply a menubar, at the time the menubar is attached to
the window, if there is not a specially named `.apple` menu (see below), Tk will
provide its default application menu. It will contain an "About Tcl & Tk" item,
followed by the standard menu items: preferences, the services submenu,
hide/show items, and quit. Again, you don't want this.

If you supply your own `.apple` menu, when the menubar is attached to the
window, Tk will add the standard items (preferences and onward) onto the end of
any items you have added. Perfect! Items you add after the menubar is attached
to the window will appear after the quit item, which, again, you don't want.

> The application menu, which we're dealing with here, is distinct from the
apple menu (the one with the apple icon, just to the left of the application
menu). Despite that, we really mean the application menu, even though Tk still
refer to it as the "apple" menu. This is a holdover from pre-OS X days when
these sorts of items did go in the actual apple menu, and there was no separate
application menu.

So, in other words, in your program, make sure you:

  1. Create a menubar for each window or the root window. Do not attach the
  menubar to the window yet!

  2. Add a menu to the menubar named .apple. It will be used as the application
  menu.

  3. The menu will automatically be named the same as the application binary; if
  you want to change this, rename (or make a copy of) the binary used to run your script.

  4. Add the items you want to appear at the top of the application menu, i.e.,
  an "About yourapp" item, followed by a separator.

  5. After you have done all this, you can then attach the menubar to your
  window via the window's menu configuration option.

```rust,no_run
let win = root.add_toplevel("win")?;
let menubar = win.add_menu("menubar")?;
let apple = menubar.add_menu("apple")?;
menubar.add_cascade( -menu(apple) )?;
apple.add_command( -label("About My Application") )?;
apple.add_separator(())?;
win.configure( -menu(menubar) )?;
```

> The pathname of the application menu must be .apple.

### Handling the Preferences Menu Item

As you've noticed, the application menu always includes a "Preferences..." menu
item. If your application has a preferences dialog, this menu item should open
it. If your application has no preferences dialog, this menu item should be
disabled, which it is by default.

To hook up your preferences dialog, you'll need to define a Tcl procedure named
`::tk::mac::ShowPreferences`. This will be called when the Preferences menu item
is chosen; if the procedure is not defined, the menu item will be disabled.

```rust,no_run
tclosure!( tk, cmd: "tk::mac::ShowPreferences", move || -> TkResult<()> {
    // show preferences
    Ok(())
});
```

### Providing a Help Menu

Like the application menu, any help menu you add to your own menubar is treated
specially on macOS. As with the application menu that needed a special name
(`.apple`), the help menu must be given the name `.help`. Also, like the
application menu, the help menu should also be added before the menubar is
attached to the window.

The help menu will include the standard macOS search box to search help, as well
as an item named "yourapp Help." As with the name of the application menu, this
comes from your program's executable and cannot be changed. Similar to how
preferences dialogs are handled, to respond to this help item, you need to
define a Tcl procedure named `::tk::mac::ShowHelp`. If this procedure is not
defined, it will not disable the menu item. Instead, it will generate an error
when the help item is chosen.

> If you don't want to include help, don't add a help menu to the menubar, and
none will be shown.

> Unlike on X11 and earlier versions of Tk on macOS, the Help menu will not
automatically be put at the end of the menubar, so ensure it is the last menu
added.

You can also add other items to the help menu. These will appear after the
application help item.

```rust,no_run
let menu_help = menubar.add_menu( "help" )?;
menubar.add_cascade( -menu(menu_help) -label("Help") )?;
tclosure!( tk, cmd: "::tk::mac::ShowHelp", move || -> TkResult<()> {
    // show help
    Ok(())
});
```

### Other Menu Handlers

You saw previously how handling certain standard menu items required you to
define Tcl callback procedures, e.g., `tk::mac::ShowPreferences` and
`tk::mac::ShowHelp`.

There are several other callbacks that you can define. For example, you might
intercept the Quit menu item, prompting users to save their changes before
quitting. Here is the complete list:

- `tk::mac::ShowPreferences`:

  Called when the "Preferences..." menu item is selected.

- `tk::mac::ShowHelp`:

  Called to display main online help for the application.

- `tk::mac::Quit`:

  Called when the Quit menu item is selected, when a user is trying to shut down the system etc.

- `tk::mac::OnHide`:

  Called when your application has been hidden.

- `tk::mac::OnShow`:

  Called when your application is shown after being hidden.

- `tk::mac::OpenApplication`:

  Called when your application is first opened.

- `tk::mac::ReopenApplication`:

  Called when a user "reopens" your already-running application (e.g. clicks on it in the Dock)

- `tk::mac::OpenDocument`:

  Called when the Finder wants the application to open one or more documents
  (e.g. that were dropped on it). The procedure is passed a list of pathnames of
  files to be opened.

- `tk::mac::PrintDocument`:

  As with OpenDocument, but the documents should be printed rather than opened.

For additional information, see the `tk_mac` command reference.

## Windows

On Windows, each window has a "System" menu at the top left of the window frame,
with a small icon for your application. It contains items like "Close",
"Minimize", etc. In Tk, if you create a system menu, you can add new items that
will appear below the standard items.

```rust,no_run
menubar.add_cascade( -menu( menubar.add_menu( "system" )? ))?;
```

## X11

On X11, if you create a help menu, Tk will ensure that it is always the last
menu in the menubar.

```rust,no_run
menubar.add_cascade( -label("Help") -menu( menubar.add_menu("help")? ))?;
```

## Run Example

`cargo run --example platform_menus`
