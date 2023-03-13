use crate::{
    TkInstance,
    Widget,
    WmManage,
};

#[derive( Copy, Clone )]
pub struct TkFrame<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> WmManage<Inst> for TkFrame<Inst> {}
