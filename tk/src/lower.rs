use crate::{
    InterpResult,
    TkInstance,
    Widget,
};

impl<Inst:TkInstance> Widget<Inst> {
    pub fn lower( &self ) -> InterpResult<()> {
        self.tk().run(( "lower", self.path ))
    }

    pub fn lower_below( &self, below_this: &Self ) -> InterpResult<()> {
        self.tk().run(( "lower", self.path, below_this.path ))
    }
}
