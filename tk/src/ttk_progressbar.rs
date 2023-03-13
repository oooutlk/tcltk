use crate::{
    InterpResult,
    TkInstance,
    TtkCommonTraits,
    Widget,
};

use std::os::raw::{
    c_double,
    c_int,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TtkProgressbar<TK:TkInstance>( pub(crate) Widget<TK> );

pub struct TtkProgressbarInterval {
    milliseconds: c_int,
}

impl TtkProgressbarInterval {
    pub fn new( milliseconds: c_int ) -> Option<Self> {
        if milliseconds > 0 {
            Some( TtkProgressbarInterval{ milliseconds })
        } else {
            None
        }
    }
}

impl Default for TtkProgressbarInterval {
    fn default() -> Self {
        TtkProgressbarInterval{ milliseconds: 50 }
    }
}

impl From<TtkProgressbarInterval> for Obj {
    fn from( interval: TtkProgressbarInterval ) -> Obj {
        interval.milliseconds.into()
    }
}

impl<TK:TkInstance> TtkProgressbar<TK> {
    pub fn start( &self, interval: TtkProgressbarInterval ) -> InterpResult<()> {
        self.tk().run(( self.path, "start", interval ))
    }

    pub fn step( &self, amount: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "step", amount ))
    }

    pub fn stop( &self ) -> InterpResult<()> {
        self.tk().run(( self.path, "stop" ))
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkProgressbar<TK> {}
