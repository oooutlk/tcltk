use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let _txt = root.add_text( "t" -width(40) -height(10) )?
        .grid(())?;

    Ok( main_loop() )
}
