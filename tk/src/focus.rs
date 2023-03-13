use crate::{
    InterpResult,
    NOT_SEND_SYNC,
    Tk,
    TkInstance,
    Widget,
};

use tcl::Obj;

impl<Inst:TkInstance> Tk<Inst> {
    pub fn focus( &self ) -> InterpResult<Widget<Inst>> {
        Ok( Widget{ path: Tk::<Inst>::make_or_get_path( &self.eval( "focus" )?.to_string() ), inst: self.inst, mark: NOT_SEND_SYNC })
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    pub fn focus( &self ) -> InterpResult<()> {
        self.tk().run(( "focus", self.path ))
    }

    pub fn focus_displayof( &self ) -> InterpResult<Obj> {
        self.tk().eval(( "focus", "-displayof", self.path ))
    }

    pub fn focus_force( &self ) -> InterpResult<()> {
        self.tk().run(( "focus", "-force", self.path ))
    }

    pub fn focus_lastfor( &self ) -> InterpResult<Obj> {
        self.tk().eval(( "focus", "-lastfor", self.path ))
    }
}
