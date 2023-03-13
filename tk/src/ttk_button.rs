use crate::{
    InterpResult,
    TkInstance,
    TtkCommonTraits,
    Widget,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TtkButton<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> TtkButton<Inst> {
    pub fn invoke( &self ) -> InterpResult<Obj> {
        Ok( self.0.tk().eval(( self.0.path, "invoke" ))? )
    }
}

impl<Inst:TkInstance> TtkCommonTraits<Inst> for TtkButton<Inst> {}
