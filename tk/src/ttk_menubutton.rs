use crate::{
    TkInstance,
    TtkCommonTraits,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TtkMenubutton<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkMenubutton<TK> {}
