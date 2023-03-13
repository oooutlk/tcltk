use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let s = root.add_ttk_spinbox( "s" -from(1.0) -to(100.0) -textvariable("spinval") )?
        .grid(())?;

    s.set_state( TtkState::ReadOnly )?;

    Ok( main_loop() )
}
