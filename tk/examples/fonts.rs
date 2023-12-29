// cargo run --example fonts

use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    println!( "{:#?}", tk.font_names()? );

    println!( "{:#?}", tk.font_actual_get_all( Font::<()>::Name( "TkTextFont" ))? );
    // e.g. -family .AppleSystemUIFont -size 13 -weight normal -slant roman -underline 0 -overstrike 0

    println!( "{:#?}", tk.font_metrics_get_all( Font::<()>::Name( "TkTextFont" ))? );
    // e.g. -ascent 13 -descent 3 -linespace 16 -fixed 0

    println!( "{:#?}", tk.font_measure( Font::<()>::Name( "TkTextFont" ), "The quick brown fox" )? );
    // e.g. 124

    tk.font_create( "AppHighlightFont", -family("Helvetica") -size(12) -weight("bold") )?;
    root.add_ttk_label( "l" -text("Attention!") -font("AppHighlightFont") )?
        .grid(())?;

    println!( "{:#?}", tk.font_families()? );

    Ok(())
}
