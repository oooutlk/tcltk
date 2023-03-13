use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    TkCoord,
    TkInstance,
    TtkCommonTraits,
    Widget,
};

use std::os::raw::{
    c_double,
    c_int,
};

use tcl::{
    error::{
        DeError,
        InterpError,
    },
    from_obj,
};

#[derive( Copy, Clone )]
pub struct TtkScale<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkScale<TK> {
    #[cex]
    pub fn coords( &self ) -> Result!( TkCoord throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "coords" ))?;
        ret!( from_obj::<TkCoord>( obj ));
    }

    #[cex]
    pub fn coords_of( &self, value: c_double ) -> Result!( TkCoord throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "coords", value ))?;
        ret!( from_obj::<TkCoord>( obj ));
    }

    pub fn get( &self ) -> InterpResult<c_double> {
        let obj = self.tk().eval(( self.path, "get" ))?;
        self.tk().double( obj )
    }

    pub fn get_at( &self, scale_coord: c_int ) -> InterpResult<c_double> {
        let obj = self.tk().eval(( self.path, "get", scale_coord ))?;
        self.tk().double( obj )
    }

    pub fn set( &self, value: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "set", value ))
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkScale<TK> {}
