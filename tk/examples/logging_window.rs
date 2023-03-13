// cargo run --example logging_window

use tk::*;
use tk::cmd::*;
use tk::text::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let log = root
        .add_text( -width(80) -height(24) -wrap("none") )?
        .grid(())?;

    let write_to_log = | msg: &str| -> TkResult<()> {
        let index = log.index( Index::end().lines(-1) )?;
        log.configure( -state("normal") )?;
        log.insert( Index::end(), msg )?;
        if log.index( Index::end().chars(-1) )? != Index::line_char(1,0) {
            log.insert( Index::end(), "\n" )?;
        }
        if let Index::LineChar( num_line, _, _ ) = index {
            if num_line > 24 {
                log.delete_ranges( vec![ Index::line_char(1,0)..Index::line_char(num_line-23,0) ])?;
            }
        }
        log.configure( -state("disabled") )?;
        Ok(())
    };

    for c in 'a'..='z' {
        write_to_log( &format!( "{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}", c ))?;
    }

    Ok( main_loop() )
}
