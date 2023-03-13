use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    tk.option_add( "*tearOff", 0 )?;

    let win = root.add_toplevel(())?;
    let menubar = win.add_menu(())?;
    win.configure( -menu(menubar) )?;

    let menu_file = menubar.add_menu(())?;
    let menu_edit = menubar.add_menu(())?;
    menubar.add_cascade( -menu(menu_file) -label("File") )?;
    menubar.add_cascade( -menu(menu_edit) -label("Edit") )?;

    menu_file.add_command( -label("New")     -command("newFile")   )?;
    menu_file.add_command( -label("Open...") -command("openFile")  )?;
    menu_file.add_command( -label("Close")   -command("closeFile") )?;

    let menu_recent = menu_file.add_menu(())?;
    menu_file.add_cascade( -menu(menu_recent) -label("Open Recent") )?;
    let recent_files : [&str;0] = [];
    for f in recent_files {
        menu_recent.add_command(
            -label( format!("file tail {}",f) )
            -command( format!("openFile {}",f) )
        )?;
    }

    menu_file.add_separator(())?;

    menu_file.add_checkbutton( -label("Check") -variable("check") -onvalue(1) -offvalue(0) )?;
    menu_file.add_radiobutton( -label("One")   -variable("radio") -value(1) )?;
    menu_file.add_radiobutton( -label("Two")   -variable("radio") -value(2) )?;

    menu_recent.delete_range( 0.. )?;

    println!( "{}", menu_file.entrycget( 0, label )? ); // get label of top entry in menu
    println!( "{}", menu_file.entryconfigure_options(0)? ); // show all options for an item

    menu_file.entryconfigure( menu::Index::pattern("Close"), -state("disabled") )?;

    let bookmarks = menubar.add_menu(())?;
    bookmarks.entryconfigure( 3, -label("Hide Bookmarks") )?;

    let edit = menubar.add_menu(())?;
    edit.entryconfigure( menu::Index::pattern("Paste"), -accelerator("Command+V") ).ok();

    menubar.add_command( -label("Path Browser") -underline(5) )?; // underline "B"

    // Here's a minimal example showing how we'd add two items to an "Edit" menu, the
    // standard "Paste" item, and an application-specific "Find..." item that will open
    // a dialog to find or search for something. We'll include an entry widget so that
    // we can check that "Paste" works.

    let _e = root.add_ttk_entry(())?;
    let m = root.add_menu(())?;
    let m_edit = m.add_menu( "edit" )?;
    m.add_cascade( -menu(m_edit) -label("Edit") )?;
    m_edit.add_command( -label("Paste")
        -command( tclosure!( tk, move || -> TkResult<()> {
            Ok( tk.focus()?.event_generate( event::virtual_event("Paste"), () )? )})))?;
    m_edit.add_command( -label("Find...")
        -command( tclosure!( tk, move || -> TkResult<()> {
            Ok( root.event_generate( event::virtual_event("OpenFindDialog"), () )? )})))?;
    root.configure( -menu(m) )?;
 
    root.bind( event::virtual_event("OpenFindDialog"), tclosure!( tk, move || -> TkResult<String> {
        Ok( tk.message_box( -message("I hope you find what you're looking for!") )? )
    }))?;

    Ok( main_loop() )
}
