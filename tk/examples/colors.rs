// cargo run --example colors

use tk::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    println!( "{:?}", root.winfo_rgb( TkColor::Name("red") )? );

    Ok( main_loop() )
}
