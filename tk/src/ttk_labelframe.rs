use crate::{
    TkInstance,
    TtkCommonTraits,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TtkLabelframe<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkLabelframe<TK> {}
