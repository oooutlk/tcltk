use tcl::Obj;

use crate::{
    error::{
        TkGeometryParseError,
        TkScreenNameParseError,
    },
};

use std::{
    fmt::{self, Display},
    os::raw::{
        c_double,
        c_int,
    },
    str::FromStr,
};

#[derive( Copy, Clone, Debug, PartialEq, Eq )]
pub enum TkColor<'a> {
    Name( &'a str ),
    RGB(  TkRGB   ),
}

impl<'a> From<TkColor<'a>> for Obj {
    fn from( color: TkColor<'a> ) -> Obj {
        match color {
            TkColor::Name( name ) => name.into(),
            TkColor::RGB( rgb ) => rgb.into(),
        }
    }
}

#[derive( Clone, Debug )]
pub struct TkHandler {
    pub name    : String,
    pub command : Obj,
}

#[derive( Copy, Clone, Debug, PartialEq )]
pub enum TkDistance {
    Pixels(        c_double ),
    Centimeters(   c_double ),
    Inches(        c_double ),
    Millimeters(   c_double ),
    PrinterPoints( c_double ),
}

impl Display for TkDistance {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            TkDistance::Pixels(        v ) => write!( f, "{}",  v ),
            TkDistance::Centimeters(   v ) => write!( f, "{}c", v ),
            TkDistance::Inches(        v ) => write!( f, "{}i", v ),
            TkDistance::Millimeters(   v ) => write!( f, "{}m", v ),
            TkDistance::PrinterPoints( v ) => write!( f, "{}p", v ),
        }
    }
}

impl From<TkDistance> for Obj {
    fn from( distance: TkDistance ) -> Obj {
        distance.to_string().into()
    }
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
pub struct TkBBox {
    pub x : c_int,
    pub y : c_int,
    pub w : c_int,
    pub h : c_int,
}

#[derive( Clone, Copy, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub struct TkCaret {
    #[serde( rename = "-x"      )] pub x      : c_int,
    #[serde( rename = "-y"      )] pub y      : c_int,
    #[serde( rename = "-height" )] pub height : c_int,
}

#[derive( Clone, Copy, Debug, Default, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub struct TkCoord {
    pub x: c_int,
    pub y: c_int,
}

impl TkCoord {
    pub(crate) fn wrap( self ) -> Option<Self> {
        if self.x == -1 && self.y == -1 {
            None
        } else {
            Some( self )
        }
    }
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub struct TkDLine {
    #[serde( flatten )]
    geometry : TkGeometry,
    baseline : c_int,
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub struct TkGeometry {
    pub w : c_int,
    pub h : c_int,
    pub x : c_int,
    pub y : c_int,
}

impl FromStr for TkGeometry {
    type Err = TkGeometryParseError;

    fn from_str( s: &str ) -> Result<Self,Self::Err> {
        fn find_sign( s: &str ) -> Option<usize> {
            s .chars()
              .enumerate()
              .find( |(_,c)| *c == '+' || *c == '-' )
              .map( |(n,_)| n )
        }

        // WIDTHxHEIGHT±X±Y
        if let Some( h_start ) = s.find('x') {
            let mut rest = s;
            if let Ok( w ) = (&rest[..h_start]).parse::<c_int>() {
                rest = &s[ h_start+1.. ];
                if let Some( x_start ) = find_sign( rest ) {
                    if let Ok( h ) = rest[..x_start].parse::<c_int>() {
                        rest = &rest[ x_start+1.. ];
                        if let Some( y_start ) = find_sign( rest ) {
                            if let Ok( x ) = rest[..y_start].parse::<c_int>() {
                                rest = &rest[ y_start+1.. ];
                                if let Ok( y ) = rest.parse::<c_int>() {
                                    return Ok( TkGeometry{ w, h, x, y });
                                }
                            }
                        }
                    }
                }
            }
        }
        Err( TkGeometryParseError( s.to_owned() ))
    }
}

impl Display for TkGeometry {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "{}x{}{:+}{:+}", self.w, self.h, self.x, self.y )
    }
}

impl From<TkGeometry> for Obj {
    fn from( geometry: TkGeometry ) -> Obj {
        geometry.to_string().into()
    }
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub struct TkRGB( pub u16, pub u16, pub u16 );

impl From<TkRGB> for Obj {
    fn from( rgb: TkRGB ) -> Obj {
        ( rgb.0, rgb.1, rgb.2 ).into()
    }
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
pub struct TkRectangle {
    pub left    : c_int,
    pub top     : c_int,
    pub right   : c_int,
    pub bottom  : c_int,
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
pub struct TkResizable {
    pub width  : bool,
    pub height : bool,
}

#[derive( Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize )]
pub enum TkRequester {
    #[serde( rename = "program" )] Program,
    #[serde( rename = "user"    )] User,
}

impl From<TkRequester> for Obj {
    fn from( who: TkRequester ) -> Obj {
        match who {
            TkRequester::Program => "program".into(),
            TkRequester::User    => "user"   .into(),
        }
    }
}

#[derive( Clone, Debug, PartialEq, Eq )]
pub struct TkScreenName {
    pub display_name: String,
    pub screen_index: u32,
}

impl FromStr for TkScreenName {
    type Err = TkScreenNameParseError;

    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        if let Some( dot ) = s.find('.') {
            let display_name = s[..dot].into();
            if let Ok( screen_index ) = s[dot+1..].parse::<u32>() {
                return Ok( TkScreenName{ display_name, screen_index });
            }
        }
        Err( TkScreenNameParseError( s.to_owned() ))
    }
}

#[derive( Copy, Clone, Default, Debug, PartialEq, Eq )]
pub struct TkSize {
    pub width  : c_int,
    pub height : c_int,
}

#[derive( Copy, Clone, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub enum TkState {
    #[serde( rename = "normal" )]    Normal,
    #[serde( rename = "iconic" )]    Iconic,
    #[serde( rename = "withdrawn" )] Withdrawn,
    #[serde( rename = "icon"   )]    Icon,
    #[cfg( any( target_os = "windows", target_os = "macos" ))]
    #[serde( rename = "zoomed" )]    Zoomed,
}

impl From<TkState> for Obj {
    fn from( state: TkState ) -> Obj {
        match state {
            TkState::Normal     => "normal"   .into(),
            TkState::Iconic     => "iconic"   .into(),
            TkState::Withdrawn  => "withdrawn".into(),
            TkState::Icon       => "icon"     .into(),
            #[cfg( any( target_os = "windows", target_os = "macos" ))]
            TkState::Zoomed     => "zoomed"   .into(),
        }
    }
}

#[derive( Copy, Clone, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub enum TkVisualClass {
    #[serde( rename = "directcolor" )] DirectColor,
    #[serde( rename = "grayscale"   )] GrayScale,
    #[serde( rename = "pseudocolor" )] PseudoColor,
    #[serde( rename = "staticcolor" )] StaticColor,
    #[serde( rename = "staticgray"  )] StaticGray,
    #[serde( rename = "truecolor"   )] TrueColor,
}

impl Display for TkVisualClass {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            TkVisualClass::DirectColor => write!( f, "directcolor" ),
            TkVisualClass::GrayScale   => write!( f, "grayscale" ),
            TkVisualClass::PseudoColor => write!( f, "pseudocolor" ),
            TkVisualClass::StaticColor => write!( f, "staticcolor" ),
            TkVisualClass::StaticGray  => write!( f, "staticgray" ),
            TkVisualClass::TrueColor   => write!( f, "truecolor" ),
        }
    }
}

#[derive( Copy, Clone, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub enum TkWindowingSystem {
    #[serde( rename = "x11"   )] X11,
    #[serde( rename = "win32" )] Win32,
    #[serde( rename = "aqua"  )] Aqua,
}

impl Display for TkWindowingSystem {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            TkWindowingSystem::X11   => write!( f, "x11" ),
            TkWindowingSystem::Win32 => write!( f, "win32" ),
            TkWindowingSystem::Aqua  => write!( f, "aqua" ),
        }
    }
}

#[derive( Clone, Debug, PartialEq, Eq )]
pub enum TtkInsertPos {
    Num( c_int ),
    Name( String ),
    End,
}

impl From<TtkInsertPos> for Obj {
    fn from( pos: TtkInsertPos ) -> Obj {
        match pos {
            TtkInsertPos::Num(n)  => n.into(),
            TtkInsertPos::Name(s) => s.into(),
            TtkInsertPos::End     => "end".into(),
        }
    }
}

#[derive( Copy, Clone, Debug, PartialEq, Eq )]
#[derive( serde::Serialize, serde::Deserialize )]
pub enum TtkTreeviewRegion {
    #[serde( rename = "heading"   )] Heading,
    #[serde( rename = "separator" )] Separator,
    #[serde( rename = "tree"      )] Tree,
    #[serde( rename = "cell"      )] Cell,
}
