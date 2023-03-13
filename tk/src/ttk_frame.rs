use crate::{
    TkInstance,
    TtkCommonTraits,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TtkFrame<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkFrame<TK> {}
