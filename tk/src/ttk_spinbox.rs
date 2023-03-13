use crate::{
    InterpResult,
    TkBBoxTrait,
    TkEntryTraits,
    TkInstance,
    TkXView,
    TtkCommonTraits,
    Widget,
    traits::Delete,
    ttk_entry::Index,
};

#[derive( Copy, Clone )]
pub struct TtkSpinbox<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkSpinbox<TK> {
    pub fn get( &self ) -> InterpResult<String> {
        self.tk().eval(( self.0.path, "get" )).map( |obj| obj.to_string() )
    }

    pub fn set( &self, value: &str ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "set", value ))
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkSpinbox<TK> {}

impl<TK:TkInstance> TkBBoxTrait<TK> for TtkSpinbox<TK> {
    type Index = Index;
}

impl<Inst:TkInstance> Delete<Inst> for TtkSpinbox<Inst> {
    type Index = Index;
}

impl<TK:TkInstance> TkEntryTraits<TK> for TtkSpinbox<TK> {
    type Index = Index;
}

impl<TK:TkInstance> TkXView<TK> for TtkSpinbox<TK> {
    type Index = Index;
}
