use crate::{
    TkInstance,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TkLabel<Inst:TkInstance>( pub(crate) Widget<Inst> );
