use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    root.add_ttk_progressbar( -orient("horizontal") -length(200) -mode("determinate") )?
        .grid(())?;

    Ok( main_loop() )
}
