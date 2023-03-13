// cargo run --example one_step_at_a_time

use std::os::raw::c_int;
use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let f = root.add_ttk_frame( "f" )?
        .grid(())?;
    let f_b = f.add_ttk_button( "b" -text("Start!") )?
        .grid( -column(1) -row(0) -padx(5) -pady(5) )?;
    let f_l = f.add_ttk_label( "l" -text("No Answer") )?
        .grid( -column(0) -row(0) -padx(5) -pady(5) )?;
    let f_p = f.add_ttk_progressbar( "p" -orient("horizontal") -mode("determinate") -maximum(20) )?
        .grid( -column(0) -row(1) -padx(5) -pady(5) )?;

    tclfn!( tk, fn stop() -> TkResult<()> {
        tcl_interp!().set( "interrupt", 1 );
        Ok(())
    });

    tclosure!( tk, cmd: "result", move |answer: String| -> TkResult<()> {
        f_p.configure( -value(0) )?;
        f_b.configure( -text("Start!") -command("start") )?;
        f_l.configure( -text({
            if answer.is_empty() {
                "No Answer".to_owned()
            } else {
                format!( "Answer: {}", answer )
            }
        }))?;
        Ok(())
    });

    tclosure!( tk, cmd: "step", move |count: c_int| ->TkResult<()> {
        let interp = tcl_interp!();

        f_p.configure( -value(count) )?;
        if interp.get_boolean("interrupt")? {
            interp.eval( "result {}" )?;
            return Ok(());
        }
        interp.after_ms( 100 )?; // next step in our operation; don't take too long!

        if count == 20 {
            interp.eval( "result 42" )?;
            return Ok(());  // done!
        }

        interp.after( 100, ( tclosure!( tk, move || -> TkResult<()> {
            tcl_interp!().eval(( "step", count+1 ))?;
            Ok(())
        }), ))?;

        Ok(())
    });

    f_b.configure( -command( tclosure!( tk, cmd:"start", move || -> TkResult<()> {
        f_b.configure( -text("Stop") -command("stop") )?;
        f_l.configure( -text("Working...") )?;
        let interp = tcl_interp!();
        interp.set( "interrupt", 0 );
        interp.set( "count", 1 );
        interp.after( 1, ( "step 0", ))?;
        Ok(())
    })))?;

    Ok( main_loop() )
}
