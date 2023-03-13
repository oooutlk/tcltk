use crate::{
    InterpResult,
    TkInstance,
    Widget,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TkButton<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> TkButton<Inst> {
    pub fn flash( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "flash" ))
    }

    pub fn invoke( &self ) -> InterpResult<Obj> {
        self.0.tk().eval(( self.0.path, "invoke" ))
    }
}
