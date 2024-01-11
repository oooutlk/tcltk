use tcl::*;
use tk::*;
use tk::cmd::*;

use regex::Regex;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    // ## Entry Contents
    {
        let name = root.add_ttk_entry( "name" -textvariable("username") )?.grid(())?;
        tk.run(( "set", "username", "anonymous" ))?;

        println!( "current value is {}", name.get()? );
        name.delete_range( 0.. )?; // delete between two indices, 0-based
        name.insert( 0, "your name" )?; // insert new text at a given index
    }

    // ## Watching for Changes
    {
        #[proc] fn it_has_been_written() -> TkResult<()> { Ok(()) }
        tk.trace_add_variable_write( "username", "it_has_been_written" )?;
    }

    // ## Passwords
    {
        root.add_ttk_entry( "passwd" -textvariable("password") -show("*") )?.grid(())?;
    }

    // ## Validation
    {
        let validate_cmd = tclfn!( &tk,
            fn check_num( vldt_new: String ) -> TclResult<bool> {
                Ok( vldt_new.len() <= 5 &&
                    vldt_new.chars().filter( |&ch| ch >= '0' && ch <= '9' ).count() <= 5 )
            }
        );

        root.add_ttk_entry( "e" -textvariable("num") -validate("key") -validatecommand(validate_cmd) )?
            .grid( -column(0) -row(2) -sticky("we") )?;
    }

    // ## Validation, zip
    {
        const FORMATMSG: &'static str = "Zip should be ##### or #####-####";

        let f = root.add_ttk_frame( "f" )?
            .grid( -column(0) -row(3) )?;

        f.add_ttk_label( "l1" -text("Name:") )?
            .grid( -column(0) -row(4) -padx(5) -pady(5) )?;

        let _e1 = f.add_ttk_entry( "e1" )?
            .grid( -column(1) -row(4) -padx(5) -pady(5) )?;

        f.add_ttk_label( "l" -text("Zip:") )?
            .grid( -column(0) -row(5) -padx(5) -pady(5) )?;

        let f_btn = f.add_ttk_button( "btn" -text("Process") )?
            .grid( -column(2) -row(5) -padx(5) -pady(5) )?;

        f_btn.set_state( TtkState::Disabled )?;

        let check_zip_cmd = tclosure!( tk, cmd: "check_zip",
            |vldt_new, vldt_op| -> TkResult<bool> {
                let interp = tcl_interp!();
                interp.set( "errmsg", "" );

                let re = r#"^[0-9]{5}(\-[0-9]{4})?$"#;
                let regex = Regex::new( re ).unwrap();
                let valid = regex.is_match( &vldt_new );
                f_btn.set_state( if valid{ !TtkState::Disabled } else{ TtkState::Disabled })?;
                use event::TkValidationOp::*;
                match vldt_op {
                    Key => {
                        let regex = Regex::new( r#"^[0-9\-]*$"# ).unwrap();
                        let ok_so_far = regex.is_match( &vldt_new ) && vldt_new.len() <= 10;
                        if !ok_so_far {
                            interp.set( "errmsg", FORMATMSG );
                        }
                        return Ok( true );
                    }
                    FocusOut => {
                        if !valid {
                            interp.set( "errmsg", FORMATMSG );
                        }
                    }
                    _ => (),
                }
                Ok( valid )
            }
        );

        f.add_ttk_entry( "e" -textvariable("zip") -validate("all") -validatecommand(check_zip_cmd) )?
            .grid( -column(1) -row(5) -padx(5) -pady(5) )?;

        f.add_ttk_label( "msg" -font("TkSmallCaptionFont") -foreground("red") -textvariable("errmsg") )?
            .grid( -column(1) -row(2) -padx(5) -pady(5) -sticky("w") )?;
    }

    Ok( main_loop() )
}
