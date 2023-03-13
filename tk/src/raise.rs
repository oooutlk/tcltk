use crate::{
    InterpResult,
    TkInstance,
    Widget,
};

impl<Inst:TkInstance> Widget<Inst> {
    pub fn raise( &self ) -> InterpResult<()> {
        self.tk().run(( "raise", self.path ))
    }

    pub fn raise_above( &self, above_this: &Self ) -> InterpResult<()> {
        self.tk().run(( "raise", self.path, above_this.path ))
    }
}
