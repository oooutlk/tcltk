use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let check = root
        .add_ttk_checkbutton( "check" -text("Use Metric") -command( "metricChanged" )
            -variable("measuresystem") -onvalue("metric") -offvalue("imperial") )?
        .pack(())?;

    assert!( check.instate( TtkState::Alternate )? );

    Ok( main_loop() )
}
