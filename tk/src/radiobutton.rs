use crate::{
    InterpResult,
    TkInstance,
    Widget,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TkRadiobutton<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> TkRadiobutton<Inst> {
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
}
