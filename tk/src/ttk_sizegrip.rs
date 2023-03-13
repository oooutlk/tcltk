use crate::{
    TkInstance,
    TtkCommonTraits,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TtkSizegrip<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkSizegrip<TK> {}
