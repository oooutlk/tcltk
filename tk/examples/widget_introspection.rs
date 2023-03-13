// cargo run --example widget_introspection

use tk::*;
use tk::cmd::*;

fn print_hierarchy<TK:TkInstance>( w: &Widget<TK>, depth: usize ) -> TkResult<()> {
    println!( "{}{} w={} h={} x={} y={}"
        , str::repeat( " ", depth )
        , w.winfo_class()?
        , w.winfo_width()?
        , w.winfo_height()?
        , w.winfo_x()?
        , w.winfo_y()?
    );
    for child in w.winfo_children()? {
        print_hierarchy( &child, depth+1 )?;
    }

    Ok(())
}

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    tk.root().add_widgets(
        -pack( -label( -text("all in one") ))
        -pack( -frame( -pack( -button( "btn" -text("quit") -command("destroy .") ))))
    )?;

    print_hierarchy( &tk.root(), 0 )?;

    Ok(())
}
