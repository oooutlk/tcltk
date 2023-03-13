// cargo run --example notebook

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let n = root.add_ttk_notebook(())?.pack(())?;
    let f1 = n.add_ttk_frame(())?; // first page, which would get widgets gridded into it 
    let f2 = n.add_ttk_frame(())?; // second page
    n.add( &f1, -text("One") )?;
    n.add( &f2, -text("Two") )?;

    Ok( main_loop() )
}
