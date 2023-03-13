use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    Obj,
    error::{
        InterpError,
        NotList,
    },
    interp::{
        Interp,
        Result,
    },
};

use std::os::raw::c_int;

use tuplex::*;

impl Interp {
    pub fn after_ms( &self, ms: c_int ) -> Result<()> {
        self.run(( "after", ms ))
    }

    /// In this form the command returns immediately, but it arranges for a Tcl command
    /// to be executed ms milliseconds later as an event handler. The command will be
    /// executed exactly once, at the given time. The delayed command is formed by
    /// concatenating all the script arguments in the same fashion as the concat
    /// command. The command will be executed at global level (outside the context of
    /// any Tcl procedure). If an error occurs while executing the delayed command then
    /// the background error will be reported by the command registered with interp
    /// bgerror. The after command returns an identifier that can be used to cancel the
    /// delayed command using after cancel.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tcl::*;
    /// use tuplex::*;
    ///
    /// let interp = Interpreter::new().unwrap();
    ///
    /// interp.eval( "set ::foo 1; set ::bar 2;" )?;
    /// tclfn!( interp, cmd: "set_foo", fn set_foo() -> TclResult<()> {
    ///     let interp = tcl_interp!();
    ///     interp.set( "foo", 3 );
    ///     Ok(())
    /// });
    ///
    /// tclfn!( interp, cmd: "set_bar", fn set_bar() -> TclResult<()> {
    ///     let interp = tcl_interp!();
    ///     interp.set( "bar", 4 );
    ///     Ok(())
    /// });
    ///
    /// interp.after( 1000, ( "set_foo", ))?;
    /// interp.after( 1000, ( "set_bar", ))?;
    ///
    /// unsafe{ clib::Tcl_DoOneEvent( 0 ); }
    ///
    /// assert_eq!( interp.get_int("foo")?, 3 );
    /// assert_eq!( interp.get_int("bar")?, 4 );
    ///
    /// # TclResult::<()>::Ok(())
    /// ```
    pub fn after<Scripts,Tag>( &self, ms: c_int, scripts: Scripts ) -> Result<Obj>
        where Scripts: IntoHomoTuple<Obj> + NonZeroLen<Tag>
            , <Scripts as IntoHomoTuple<Obj>>::Output : Into<Obj>
    {
        self.eval(( "eval", "after", ms, scripts.into_homo_tuple() ))
    }

    pub fn after_cancel_id( &self, id: impl Into<Obj> ) -> Result<()>
    {
        self.run(( "after", "cancel", id ))
    }

    pub fn after_cancel<Scripts,Tag>( &self, scripts: Scripts ) -> Result<()>
        where Scripts: IntoHomoTuple<Obj> + NonZeroLen<Tag>
            , <Scripts as IntoHomoTuple<Obj>>::Output : Into<Obj>
    {
        self.run(( "eval", "after", "cancel", scripts.into_homo_tuple() ))
    }

    pub fn after_idle<Scripts,Tag>( &self, scripts: Scripts ) -> Result<()>
        where Scripts: IntoHomoTuple<Obj> + NonZeroLen<Tag>
            , <Scripts as IntoHomoTuple<Obj>>::Output : Into<Obj>
    {
        self.run(( "eval", "after", "idle", scripts.into_homo_tuple() ))
    }

    pub fn after_info_id( &self, id: Obj ) -> Result<Obj> {
        self.eval(( "after", "info", id ))
    }

    #[cex]
    pub fn after_info( &self ) -> Result!( Vec<Obj> throws InterpError, NotList ) {
        Ok( self.eval(( "after", "info" ))?.get_elements()?.collect() )
    }
}
