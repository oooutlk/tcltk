// cargo run --example configuration_options

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    // create a button, passing two options:
    let b = root.add_ttk_button( "b" -text("Hello") -command("button_pressed") )?.grid(())?;

    // check the current value of the text option:
    assert_eq!( b.cget( text )?.to_string(), "Hello" );

    // check the current value of the command option:
    assert_eq!( b.cget( command )?.to_string(), "button_pressed" );

    // change the value of the text option:
    b.configure( -text("Goodbye") )?;

    // check the current value of the text option:
    assert_eq!( b.cget( text )?.to_string(), "Goodbye" );

    Ok( main_loop() )
}
