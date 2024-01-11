use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let country = root.add_ttk_combobox( "country" -textvariable("country") )?
        .pack(())?;

    let script = tclosure!( tk,
        || -> TkResult<()> { Ok( println!( "combobox {} item selected: {}", country.path(), country.get()? ))}
    );

    country.bind( event::virtual_event( "ComboboxSelected" ), script )?;

    country.configure( -values([ "USA","Canada","Australia" ].as_slice() ))?;

    country.set_state( TtkState::ReadOnly )?;

    Ok( main_loop() )
}
