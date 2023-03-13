use crate::{
    TkInstance,
    Widget,
};

#[derive( Copy, Clone )]
pub struct TkMenubutton<Inst:TkInstance>( pub(crate) Widget<Inst> );
