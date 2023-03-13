use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    TkCoord,
    TkInstance,
    Widget,
    error::TkScalePartParseError,
};

use std::os::raw::{
    c_double,
    c_int,
    c_longlong,
};

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
    },
    from_obj,
};

#[derive( Copy, Clone )]
pub struct TkScale<Inst:TkInstance>( pub(crate) Widget<Inst> );

pub enum TkScaleCoord {
    Horizontal( c_longlong ),
    Vertical( c_longlong ),
}

impl From<TkScaleCoord> for Obj {
    fn from( scale_coord: TkScaleCoord ) -> Obj {
        match scale_coord {
            TkScaleCoord::Horizontal( x ) => (x, 0).into(),
            TkScaleCoord::Vertical(   y ) => (0, y).into(),
        }
    }
}

pub enum TkScalePart {
    Slider, Trough1, Trough2,
}

impl<Inst:TkInstance> TkScale<Inst> {
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

    pub fn get_at( &self, scale_coord: TkScaleCoord ) -> InterpResult<c_double> {
        let obj = self.tk().eval(( self.path, "get", scale_coord ))?;
        self.tk().double( obj )
    }

    #[cex]
    pub fn identify( &self, x: c_int, y: c_int ) -> Result!( Option<TkScalePart> throws InterpError, TkScalePartParseError ) {
        let obj = self.tk().eval(( self.path, "identify", x, y ))?;
        let s = obj.to_string();
        ret!( match s.as_str() {
            "slider"  => Some( TkScalePart::Slider  ),
            "trough1" => Some( TkScalePart::Trough1 ),
            "trough2" => Some( TkScalePart::Trough2 ),
            "" => None,
            _ => throw!( TkScalePartParseError( s )),
        })
    }

    pub fn set( &self, value: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "set", value ))
    }
}
