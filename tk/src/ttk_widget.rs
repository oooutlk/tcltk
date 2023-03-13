use enumx::export::*;
use enumx::predefined::*;
use cex::*;

//use bitflags::bitflags;

use crate::{
    InterpResult,
    TkInstance,
    Widget,
    error::TtkStateParseError,
};

use std::{
    convert::TryFrom,
    fmt::{self, Display},
    ops::{Deref, Not},
    os::raw::c_int,
    str::FromStr,
};

use tcl::{
    Obj,
    error::{
        InterpError,
        NotList,
    },
};

pub type TtkStateSpec = Vec<TtkState>;

impl From<TtkState> for TtkStateSpec {
    fn from( state: TtkState ) -> TtkStateSpec {
        vec![ state ]
    }
}

#[derive( Copy, Clone, Debug, PartialEq, Eq )]
pub enum TtkState {
    Active        = 0b0000000001,
    Disabled      = 0b0000000010,
    Focus         = 0b0000000100,
    Pressed       = 0b0000001000,
    Selected      = 0b0000010000,
    Background    = 0b0000100000,
    ReadOnly      = 0b0001000000,
    Alternate     = 0b0010000000,
    Invalid       = 0b0100000000,
    Hover         = 0b1000000000,
    NotActive     = 0b1111111110,
    NotDisabled   = 0b1111111101,
    NotFocus      = 0b1111111011,
    NotPressed    = 0b1111110111,
    NotSelected   = 0b1111101111,
    NotBackground = 0b1111011111,
    NotReadonly   = 0b1110111111,
    NotAlternate  = 0b1101111111,
    NotInvalid    = 0b1011111111,
    NotHover      = 0b0111111111,
}

impl Not for TtkState {
    type Output = Self;

    fn not( self ) -> Self::Output {
        use TtkState::*;
        match self {
            Active        => NotActive    ,
            Disabled      => NotDisabled  ,
            Focus         => NotFocus     ,
            Pressed       => NotPressed   ,
            Selected      => NotSelected  ,
            Background    => NotBackground,
            ReadOnly      => NotReadonly  ,
            Alternate     => NotAlternate ,
            Invalid       => NotInvalid   ,
            Hover         => NotHover     ,
            NotActive     => Active       ,
            NotDisabled   => Disabled     ,
            NotFocus      => Focus        ,
            NotPressed    => Pressed      ,
            NotSelected   => Selected     ,
            NotBackground => Background   ,
            NotReadonly   => ReadOnly     ,
            NotAlternate  => Alternate    ,
            NotInvalid    => Invalid      ,
            NotHover      => Hover        ,
        }
    }
}

impl TtkState {
    fn to_str( &self ) -> &'static str {
        use TtkState::*;
        match self {
            Active        => "active"     ,
            Disabled      => "disabled"   ,
            Focus         => "focus"      ,
            Pressed       => "pressed"    ,
            Selected      => "selected"   ,
            Background    => "background" ,
            ReadOnly      => "readonly"   ,
            Alternate     => "alternate"  ,
            Invalid       => "invalid"    ,
            Hover         => "hover"      ,
            NotActive     => "!active"    ,
            NotDisabled   => "!disabled"  ,
            NotFocus      => "!focus"     ,
            NotPressed    => "!pressed"   ,
            NotSelected   => "!selected"  ,
            NotBackground => "!background",
            NotReadonly   => "!readonly"  ,
            NotAlternate  => "!alternate" ,
            NotInvalid    => "!invalid"   ,
            NotHover      => "!hover"     ,
        }
    }
}

impl Display for TtkState {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        formatter.write_str( self.to_str() )
    }
}

impl FromStr for TtkState {
    type Err = TtkStateParseError;

    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        use TtkState::*;
        match s {
            "active"      => Ok( Active        ),
            "disabled"    => Ok( Disabled      ),
            "focus"       => Ok( Focus         ),
            "pressed"     => Ok( Pressed       ),
            "selected"    => Ok( Selected      ),
            "background"  => Ok( Background    ),
            "readonly"    => Ok( ReadOnly      ),
            "alternate"   => Ok( Alternate     ),
            "invalid"     => Ok( Invalid       ),
            "hover"       => Ok( Hover         ),
            "!active"     => Ok( NotActive     ),
            "!disabled"   => Ok( NotDisabled   ),
            "!focus"      => Ok( NotFocus      ),
            "!pressed"    => Ok( NotPressed    ),
            "!selected"   => Ok( NotSelected   ),
            "!background" => Ok( NotBackground ),
            "!readonly"   => Ok( NotReadonly   ),
            "!alternate"  => Ok( NotAlternate  ),
            "!invalid"    => Ok( NotInvalid    ),
            "!hover"      => Ok( NotHover      ),
            _ => Err( TtkStateParseError( s.to_owned() )),
        }
    }
}

impl From<TtkState> for Obj {
    fn from( state: TtkState ) -> Obj {
        state.to_str().into()
    }
}

impl TryFrom<Obj> for TtkState {
    type Error = TtkStateParseError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        TtkState::from_str( &obj.get_string() )
    }
}

pub trait TtkCommonTraits<TK:TkInstance>
    where Self : Deref<Target=Widget<TK>>
{
    fn identify_element( &self, x: c_int, y: c_int ) -> InterpResult<Option<String>> {
        let widget = self.deref();
        let s = widget.tk().eval(( widget.path, "identify", "element", x, y ))?.to_string();

        if s.is_empty() {
            Ok( None )
        } else {
            Ok( Some( s ))
        }
    }

    fn instate( &self, state_spec: impl Into<TtkStateSpec> ) -> InterpResult<bool> {
        let widget = self.deref();
        let obj = widget.tk().eval(( widget.path, "instate", state_spec.into() ))?;
        widget.tk().boolean( obj )
    }

    fn instate_run( &self, state_spec: impl Into<TtkStateSpec>, script: impl Into<Obj> ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "instate", state_spec.into(), script ))
    }

    #[cex]
    fn set_state( &self, state_spec: impl Into<TtkStateSpec> ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "state", state_spec.into() ))
    }

    #[cex]
    fn state( &self ) -> Result!( TtkStateSpec throws InterpError, NotList, TtkStateParseError ) {
        let widget = self.deref();
        let obj = widget.tk().eval(( widget.path, "state" ))?;
        let mut state_spec = TtkStateSpec::new();
        for elem in obj.get_elements()? {
            state_spec.push( TtkState::try_from( elem )? );
        }

        Ok( state_spec )
    }
}
