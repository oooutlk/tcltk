use crate::{
    InterpResult,
    TkInstance,
    TtkCommonTraits,
    Widget,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TtkCheckbutton<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkCheckbutton<TK> {
    pub fn deselect( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "deselect" ))
    }

    pub fn flash( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "flash" ))
    }

    pub fn invoke( &self ) -> InterpResult<Obj> {
        Ok( self.0.tk().eval(( self.0.path, "invoke" ))? )
    }

    pub fn select( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "select" ))
    }

    pub fn toggle( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "toggle" ))
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkCheckbutton<TK> {}
