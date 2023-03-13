use crate::{
    TkInstance,
    Widget,
    WmManage,
};

#[derive( Copy, Clone )]
pub struct TkToplevel<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> WmManage<Inst> for TkToplevel<Inst> {}
