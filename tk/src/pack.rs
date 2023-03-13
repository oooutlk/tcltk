use crate::{
    InterpResult,
    TkInstance,
    Widget,
};

impl<Inst:TkInstance> Widget<Inst> {
    pub fn pack_propagate( &self, do_propagate: bool ) -> InterpResult<()> {
        self.tk().run(( "pack", "propagate", self.path, do_propagate ))
    }

    pub fn pack_propagated( &self ) -> InterpResult<bool> {
        let obj = self.tk().eval(( "pack", "propagate", self.path ))?;
        self.tk().boolean( obj )
    }
}
