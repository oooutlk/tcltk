// cargo run --example text_the_basics

use tk::*;
use tk::text::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let txt = root.add_text( "t" -width(40) -height(10) )?
        .grid(())?;

    txt.insert( Index::line_char(1,0), "here is my\ntext to insert" )?;

    let the_text = txt.get_range( Index::line_char(1,0).. )?;
    assert_eq!( the_text, "here is my\ntext to insert\n" );

    txt.see( Index::line_char(1,0) )?;

    txt.configure( -state("disabled") )?;

    Ok( main_loop() )
}
