// cargo run --example listbox

use std::collections::HashMap;

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    // Initialize our country "databases":
    //  - the list of country codes (a subset anyway)
    //  - parallel list of country names, same order as the country codes
    //  - a hash table mapping country code to population
    tk.set( "countrycodes", vec![
        "ar", "au", "be", "br", "ca", "cn", "dk", "fi", "fr", "gr",
        "in", "it", "jp", "mx", "nl", "no", "es", "se", "ch" ]);

    tk.set( "countrynames", vec![
        "Argentina", "Australia", "Belgium", "Brazil", "Canada", "China",
        "Denmark", "Finland", "France", "Greece", "India", "Italy", "Japan",
        "Mexico", "Netherlands", "Norway", "Spain", "Sweden", "Switzerland" ]);

    let mut populations = HashMap::new();

    populations.insert( "ar",   41000000 );
    populations.insert( "au",   21179211 );
    populations.insert( "be",   10584534 );
    populations.insert( "br",  185971537 );
    populations.insert( "ca",   33148682 );
    populations.insert( "cn", 1323128240 );
    populations.insert( "dk",    5457415 );
    populations.insert( "fi",    5302000 );
    populations.insert( "fr",   64102140 );
    populations.insert( "gr",   11147000 );
    populations.insert( "in", 1131043000 );
    populations.insert( "it",   59206382 );
    populations.insert( "jp",  127718000 );
    populations.insert( "mx",  106535000 );
    populations.insert( "nl",   16402414 );
    populations.insert( "no",    4738085 );
    populations.insert( "es",   45116894 );
    populations.insert( "se",    9174082 );
    populations.insert( "ch",    7508700 );

    tk.set( "populations", populations );

    // Names of the gifts we can send
    tk.arr_set( "gifts", "card"     , "Greeting card" );
    tk.arr_set( "gifts", "flowers"  , "Flowers"       );
    tk.arr_set( "gifts", "nastygram", "Nastygram"     );

    // Create and grid the outer content frame
    let c = root.add_ttk_frame( "c" -padding(( 5, 5, 12, 0 )) )?
        .grid( -column(0) -row(0) -sticky("nwes") )?;
    root.grid_columnconfigure( 0, -weight(1) )?;
    root.grid_rowconfigure(    0, -weight(1) )?;

    // Create the different widgets; note the variables that many
    // of them are bound to, as well as the button callback.
    // The listbox is the only widget we'll need to refer to directly
    // later in our program, so for convenience we'll assign it to a variable.
    let lbox = c.add_listbox( "countries" -listvariable("countrynames") -height(5) )?;

    // Called when the user double clicks an item in the listbox, presses
    // the "Send Gift" button, or presses the Return key.  In case the selected
    // item is scrolled out of view, make sure it is visible.
    //
    // Figure out which country is selected, which gift is selected with the
    // radiobuttons, "send the gift", and provide feedback that it was sent.
    let send_gift = tclosure!( tk, || -> TkResult<()> {
        let interp = tcl_interp!();
        let idx = lbox.curselection()?;
        if idx.len() == 1 {
            let idx = idx[0];
            lbox.see( idx )?;
            let gift = interp.get("gift")?;
            let gift = interp.arr_get( "gifts", gift )?;
            let name = interp
                .get( "countrynames" )?
                .list_index( idx )?
                .map( |obj| obj.get_string() )
                .unwrap_or_default();
            // Gift sending left as an exercise to the reader
            interp.set( "sentmsg",
                format!( "Sent {} to leader of {}", gift, name ));
        }
        Ok(())
    });

    c.add_ttk_label( "lbl" -text("Send to country's leader:") )?;
    c.add_ttk_radiobutton( "g1" -text( tk.arr_get( "gifts", "card"      )? ) -variable("gift") -value("card") )?;
    c.add_ttk_radiobutton( "g2" -text( tk.arr_get( "gifts", "flowers"   )? ) -variable("gift") -value("flowers") )?;
    c.add_ttk_radiobutton( "g3" -text( tk.arr_get( "gifts", "nastygram" )? ) -variable("gift") -value("nastygram") )?;
    c.add_ttk_button( "send"    -text("Send Gift") -command(&*send_gift) -default_("active") )?;
    c.add_ttk_label(  "sentlbl" -textvariable("sentmsg")   -anchor("center") )?;
    c.add_ttk_label(  "status"  -textvariable("statusmsg") -anchor("w") )?;

    // Grid all the widgets
    tk.grid( ".c.countries" -column(0) -row(0) -rowspan(6) -sticky("nsew") )?;
    tk.grid( ".c.lbl"       -column(1) -row(0) -padx(10) -pady(5) )?;
    tk.grid( ".c.g1"        -column(1) -row(1) -sticky("w") -padx(20) )?;
    tk.grid( ".c.g2"        -column(1) -row(2) -sticky("w") -padx(20) )?;
    tk.grid( ".c.g3"        -column(1) -row(3) -sticky("w") -padx(20) )?;
    tk.grid( ".c.send"      -column(2) -row(4) -sticky("e") )?;
    tk.grid( ".c.sentlbl"   -column(1) -row(5) -columnspan(2) -sticky("n") -pady(5) -padx(5) )?;
    tk.grid( ".c.status"    -column(0) -row(6) -columnspan(2) -sticky("we") )?;
    c.grid_columnconfigure( 0, -weight(1) )?;
    c.grid_rowconfigure(    5, -weight(1) )?;

    // Called when the selection in the listbox changes; figure out
    // which country is currently selected, and then lookup its country
    // code, and from that, its population.  Update the status message
    // with the new population.  As well, clear the message about the
    // gift being sent, so it doesn't stick around after we start doing
    // other things.
    let show_population = tclosure!( tk, || -> TkResult<()> {
        let interp = tcl_interp!();
        let idx = lbox.curselection()?;
        if idx.len() == 1 {
            let idx = idx[0];
            let code = interp.get( "countrycodes" )?.list_index( idx )?
                .map( |obj| obj.get_string() ).unwrap_or_default();
            let name = interp.get( "countrynames" )?.list_index( idx )?
                .map( |obj| obj.get_string() ).unwrap_or_default();
            let popn = interp.get("populations")?.dict_get( code.clone() )?
                .map( |obj| obj.get_string() ).unwrap_or_default();
            interp.set( "statusmsg",
                format!( "The population of {}({}) is {}", name, code, popn ));
        }
        interp.set( "sentmsg", "" );
        Ok(())
    });

    // Set event bindings for when the selection in the listbox changes,
    // when the user double clicks the list, and when they hit the Return key
    lbox.bind( event::virtual_event( "ListboxSelect" ), &*show_population )?;
    lbox.bind( event::double().button_press_1(), &*send_gift )?;
    root.bind( event::key_press( TkKey::Return ), &*send_gift )?;

    // Colorize alternating lines of the listbox
    let len = tk.get( "countrynames" )?.list_length()?;
    (0..len).step_by(2).try_for_each( |i| -> InterpResult<()> {
        Ok( lbox.itemconfigure( i, -background("#f0f0ff") )? )
    })?;

    // Set the starting state of the interface, including selecting the
    // default gift to send, and clearing the messages.  Select the first
    // country in the list; because the <<ListboxSelect>> event is only
    // fired when users makes a change, we explicitly call showPopulation.
    tk.set( "gift", "card" );
    tk.set( "sentmsg", "" );
    tk.set( "statusmsg", "" );
    lbox.selection_set_range( 0.. )?;
    tk.run( &*show_population )?;

    //
    //lbox.bind( event::virtual_event( "ListboxSelect" ),
    //    tclosure!( tk, || -> TkResult<()> {
    //        Ok( update_details( lbox.curselection()? ))
    //    }
    //))?;
    //
    //lbox.bind( event::double().button_press_1(),
    //    tclosure!( tk, || -> TkResult<()> {
    //        Ok( invoke_action( lbox.curselection()? ))
    //    }
    //))?;

    Ok( main_loop() )
}
