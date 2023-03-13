use crate::{
    TkInstance,
    Widget,
    WmManage,
};

#[derive( Copy, Clone )]
pub struct TkLabelframe<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> WmManage<Inst> for TkLabelframe<Inst> {}
