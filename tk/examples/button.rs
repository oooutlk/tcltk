use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    #[proc] fn my_action() -> TkResult<()> { Ok(()) }
    unsafe{ tk.def_proc( "myaction", my_action ); }

    let button = root
        .add_ttk_button( "action" -text("Action") -default_("active") -command("myaction") )?
        .pack(())?;

    root.bind(
        event::key_press( TkKey::Return ),
        tclosure!( tk, move || -> InterpResult<Obj> { button.invoke() })
    )?;

    let b = root
        .add_ttk_button( "b" -text("disabled?") -default_("active") -command("myaction") )?
        .pack(())?;

    b.set_state(    TtkState::Disabled )?; // set the disabled flag
    b.set_state(   !TtkState::Disabled )?; // clear the disabled flag
    b.instate(      TtkState::Disabled )?; // 1 if disabled, else 0
    b.instate(     !TtkState::Disabled )?; // 1 if not disabled, else 0
    b.instate_run( !TtkState::Disabled,
                            "myaction" )?; // execute 'myaction' if not disabled

    Ok( main_loop() )
}

