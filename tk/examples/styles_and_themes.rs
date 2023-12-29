use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    // Themes are identified by a name. You can obtain the names of all available themes:
    let names = tk.theme_names()?
        .iter()
        .fold( String::new(), |acc,name| format!( "{} {}", acc, name ));
    println!( "{}", names );

    // Only one theme can be active at a time. To obtain the name of the theme currently in use, use the following:
    let theme = tk.theme_in_use()?;
    println!( "{}", theme.name ); // aqua

    // Switching to a new theme can be done with:
    let new_theme = tk.theme_create(())?;
    new_theme.theme_use()?;

    // How do you know what the names of the styles are? If you have a particular
    // widget, and you want to know what style it is currently using, you can first
    // check the value of its `style` configuration option. If that is empty, it means
    // the widget is using the default style for the widget. You can retrieve that via
    // the widget's class. For example:
    let b = root.add_ttk_button(())?;
    assert!( b.cget( style )?.is_empty() ); // empty string as a result
    assert_eq!( b.winfo_class()?, "TButton" );

    // To use a style means to apply that style to an individual widget. All you need
    // is the style's name and the widget to apply it to. Setting the style can be done
    // at creation time:
    root.add_ttk_button( -text("Hello") -style("Fun.TButton") )?;

    // A widget's style can also be changed later with the `style` configuration
    // option:
    b.configure( -style("Emergency.TButton") )?;

    // Prepending another name (`Emergency`) followed by a dot onto an existing style
    // creates a new style derived from the existing one. The new style will have
    // exactly the same options as the existing one except for the indicated
    // differences:
    let some_style = tk.new_ttk_style( "SomeStyle", None );
    some_style.configure( -font("helvetica 24") -foreground("red") -padding(10) )?;

    // We can ask Tk for the layout of the `TButton` style:
    let tbutton_style = tk.new_ttk_style( "TButton", None );
    println!( "{}", tbutton_style.layout()? );


    // You can determine what options are available for each element? Here's an example
    // of checking what options are available for the label inside the button (which we
    // know from the `layout` method is identified as `Button.label`):
    let options = tk
        .element("Button.label")
        .element_options()?
        .iter()
        .fold( String::new(), |acc,opt| format!( "{} {}", acc, opt ));
    println!( "{}", options ); // " -compound -space -text -font -foreground -underline -width -anchor -justify -wraplength -embossed -image -stipple -background"

    // In this section, we'll see how to change the style's appearance by modifying
    // style options. You can do this either by modifying an existing style, or more
    // typically, by creating a new style. We saw how to create a simple style that was
    // derived from another one earlier:
    let emergency_tbutton_style = tk.new_ttk_style( "Emergency.TButton", None );
    emergency_tbutton_style
        .configure( -font("helvetica 24") -foreground("red") -padding(10) )?;

    // Modifying an option for an existing style is done similarly to modifying any
    // other configuration option, by specifying the style, name of the option, and new
    // value:
    tbutton_style.configure( -font("helvetica 24") )?;

    // To retrieve the current value of an option, use the `lookup` method:
    println!( "{}", tbutton_style.lookup_normal( font )? ); // "helvetica 24"

    // The following example provides for the following variations from a button's
    // normal appearance:
    //
    // * when the widget is in the disabled state, the background color should be set
    //   to `#d9d9d9`
    //
    // * when the widget is in the active state (mouse over it), the background color
    //   should be set to `#ececec`
    //
    // * when the widget is in the disabled state, the foreground color should be set
    //   to `#a3a3a3` (this is in addition to the background color change we already
    //   noted)
    //
    // * when the widget is in the state where the button is pressed, and the widget is
    //   not disabled, the relief should be set to `sunken`
    tbutton_style.map(
        -background([ "disabled", "#d9d9d9", "active", "#ececec" ].as_slice())
        -foreground([ "disabled", "#a3a3a3" ].as_slice())
        -relief([ "pressed !disabled", "sunken" ].as_slice()))?;

    Ok( main_loop() )
}
