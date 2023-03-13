// cargo run --example text_wrapping_and_scrolling

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let txt = root.add_text( "t"
        -width(40) -height(5) -wrap("none")
        -yscrollcommand(".ys set") -xscrollcommand(".xs set") )?
        .grid( -column(0) -row(0) -sticky("nwes") )?;
    let _xs = root
        .add_ttk_scrollbar( "xs" -orient("horizontal") -command(".t xview") )?
        .grid( -column(0) -row(1) -sticky("we") )?;
    let _ys = root
        .add_ttk_scrollbar( "ys" -orient("vertical") -command(".t yview") )?
        .grid( -column(1) -row(0) -sticky("ns") )?;

    txt.insert( text::Index::end(), "Lorem ipsum...\n...\n... " )?;
    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure( 0, -weight(1) )?;

    Ok( main_loop() )
}
