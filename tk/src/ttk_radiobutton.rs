use crate::{
    InterpResult,
    TkInstance,
    TtkCommonTraits,
    Widget,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TtkRadiobutton<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkRadiobutton<TK> {
    pub fn invoke( &self ) -> InterpResult<Obj> {
        Ok( self.0.tk().eval(( self.0.path, "invoke" ))? )
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkRadiobutton<TK> {}
