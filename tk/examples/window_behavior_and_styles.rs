// cargo run --example window_behavior_and_styles

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let window = root.add_toplevel(())?;
    let _old_title = window.wm_title()?;
    window.set_wm_title( "New title" )?;

    window.set_wm_geometry( TkGeometry{ w: 300, h: 200, x: -5, y: 40 })?;

    tk.update_idletasks()?;
    println!( "{}", window.wm_geometry()? );

    window.set_wm_resizable( false, false )?;

    window.set_wm_minsize( 200, 100 )?;
    window.set_wm_maxsize( 500, 500 )?;

    window.winfo_reqwidth()?; // or reqheight

    let callback = tkbind!( tk, || -> TkResult<()> { Ok(()) }); //dummy
    unsafe { window.set_wm_protocol( "WM_DELETE_WINDOW", callback )?; }

    window.set_wm_attributes( -alpha(0.5) )?;

    window.set_wm_attributes( -fullscreen(true) )?;

    let _the_state = window.wm_state()?;
    window.set_wm_state( TkState::Normal )?;
    window.wm_iconify()?;
    window.wm_deiconify()?;
    window.wm_withdraw()?;

    window.set_wm_attributes( -topmost(true) )?;

    window
        .wm_stackorder()?
        .iter()
        .for_each( |widget| println!( "stackorder: {}", widget.path() ));

    let other = root.add_toplevel(())?;
    if window.wm_stackorder_isabove( &other ).unwrap_or_default() {}
    if window.wm_stackorder_isbelow( &other ).unwrap_or_default() {}

    window.raise()?;
    window.raise_above( &other )?;
    window.lower()?;
    window.lower_below( &other )?;

    let little = root.add_ttk_label( "little" -text("Little") )?
        .grid( -column(0) -row(0) )?; 
    root.add_ttk_label( "bigger" -text("Much Bigger Label") )?
        .grid( -column(0) -row(0) )?;
    tk.after( 2000, (tkbind!( tk,
        || -> TkResult<()> { Ok( little.raise()? )}),))?;

    println!( "color depth={} ({})", root.winfo_screendepth()?, root.winfo_screenvisual()? );
    println!( "pixels per inch={}", root.winfo_pixels( TkDistance::Inches(1.0) )? );
    println!( "width={} height={}", root.winfo_screenwidth()?, root.winfo_screenheight()? );

    root.winfo_screen()?;

    root.wm_maxsize()?;

    Ok( main_loop() )
}

