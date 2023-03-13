use crate::{
    TkInstance,
    TtkCommonTraits,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TtkSeparator<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkSeparator<TK> {}
