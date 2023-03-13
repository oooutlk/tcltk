use crate::{
    InterpResult,
    Tk,
    TkInstance,
};

use std::path::Path;

use tcl::{
    Obj,
};

impl<Inst:TkInstance> Tk<Inst> {
    pub fn option_add( &self, pattern: impl Into<Obj>, value: impl Into<Obj> ) -> InterpResult<()> {
        self.run(( "option", "add", pattern, value ))
    }

    pub fn option_add_with_priority( &self, pattern: impl Into<Obj>, value: impl Into<Obj>, priority: impl Into<Obj> )
        -> InterpResult<()>
    {
        self.run(( "option", "add", pattern, value, priority ))
    }

    pub fn option_clear( &self ) -> InterpResult<()> {
        self.run(( "option", "clear" ))
    }

    pub fn option_get( &self, window: impl Into<Obj>, name: impl Into<Obj>, class: impl Into<Obj> ) -> InterpResult<Obj> {
        self.eval(( "option", "get", window, name, class ))
    }

    pub fn option_readfile( &self, filename: impl AsRef<Path> ) -> InterpResult<()> {
        self.run(( "option", "readfile", filename.as_ref().to_string_lossy() ))
    }

    pub fn option_readfile_with_priority( &self, filename: impl AsRef<Path>, priority: impl Into<Obj> )
        -> InterpResult<()>
    {
        self.run(( "option", "readfile", filename.as_ref().to_string_lossy(), priority ))
    }
}
