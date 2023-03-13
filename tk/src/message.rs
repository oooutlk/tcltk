use crate::{
    TkInstance,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TkMessage<Inst:TkInstance>( pub(crate) Widget<Inst> );
