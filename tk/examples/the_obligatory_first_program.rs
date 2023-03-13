// cargo run --example the_obligatory_first_program

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    root.add_label( -text("hello,world!") )?.pack(())?;
    Ok( main_loop() )
}
