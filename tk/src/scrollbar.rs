use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    TkInstance,
    Widget,
    error::{
        TkScrollbarElementParseError,
        UnexpectedScrollbarElementActivatedError,
    },
};

use std::os::raw::{
    c_double,
    c_int,
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
pub struct TkScrollbar<Inst:TkInstance>( pub(crate) Widget<Inst> );

pub enum TkScrollbarElement {
    Arrow1, Trough1, Slider, Trough2, Arrow2,
}

impl From<TkScrollbarElement> for Obj {
    fn from( element: TkScrollbarElement ) -> Obj {
        use TkScrollbarElement::*;
        match element {
            Arrow1  => "arrow1" .into(),
            Trough1 => "trough1".into(),
            Slider  => "slider" .into(),
            Trough2 => "trough2".into(),
            Arrow2  => "arrow2" .into(),
        }
    }
}

pub enum TkScrollbarDelta {
    Horizontal( c_int ),
    Vertical( c_int ),
}

impl From<TkScrollbarDelta> for Obj {
    fn from( delta: TkScrollbarDelta ) -> Obj {
        match delta {
            TkScrollbarDelta::Horizontal( x ) => (x, 0).into(),
            TkScrollbarDelta::Vertical(   y ) => (0, y).into(),
        }
    }
}

impl<Inst:TkInstance> TkScrollbar<Inst> {
    /// Marks the element indicated by element as active, which causes it to be
    /// displayed as specified by the -activebackground and -activerelief
    /// options. The only element values understood by this command are
    /// `TkScrollbarElement::Arrow1`, `TkScrollbarElement::Slider`, or
    /// `TkScrollbarElement::Arrow2`. If any other value is specified then no
    /// element of the scrollbar will be active.
    pub fn activate( &self, element: TkScrollbarElement ) -> InterpResult<()> {
         self.tk().run(( self.path, "activate", element ))
    }

    /// Returns the name of the element that is currently active.
    #[cex]
    pub fn activated( &self ) -> Result!( Option<TkScrollbarElement>
        throws InterpError, UnexpectedScrollbarElementActivatedError )
    {
        let obj = self.tk().eval(( self.path, "activate" ))?;
        let s = obj.to_string();
        match s.as_str() {
            "arrow1"  => Ok( Some( TkScrollbarElement::Arrow1  )),
            "slider"  => Ok( Some( TkScrollbarElement::Slider  )),
            "arrow2"  => Ok( Some( TkScrollbarElement::Arrow2  )),
            ""        => Ok( None ),
            _ => throw!( UnexpectedScrollbarElementActivatedError( s )),
        }
    }

    /// Returns a real number indicating the fractional change in the scrollbar
    /// setting that corresponds to a given change in slider position. For
    /// example, if the scrollbar is horizontal, the result indicates how much
    /// the scrollbar setting must change to move the slider delta pixels to the
    /// right. If the scrollbar is vertical, the result indicates how much the
    /// scrollbar setting must change to move the slider delta pixels down. The
    /// arguments and the result may be zero or negative.
    pub fn delta( &self, delta: TkScrollbarDelta ) -> InterpResult<c_double> {
         let obj = self.tk().eval(( self.path, "delta", delta ))?;
         self.tk().double( obj )
    }

    /// Returns a real number between 0 and 1 indicating where the point given
    /// by x and y lies in the trough area of the scrollbar. The value 0
    /// corresponds to the top or left of the trough, the value 1 corresponds to
    /// the bottom or right, 0.5 corresponds to the middle, and so on. X and y
    /// must be pixel coordinates relative to the scrollbar widget. If x and y
    /// refer to a point outside the trough, the closest point in the trough is
    /// used.
    pub fn fraction( &self, x: c_int, y: c_int ) -> InterpResult<c_double> {
         let obj = self.tk().eval(( self.path, "fraction", x, y ))?;
         self.tk().double( obj )
    }

    /// Returns the scrollbar settings that are the arguments to the most recent
    /// `set()` method.
    #[cex]
    pub fn get( &self ) -> Result!( (c_double, c_double) throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "get" ))?;
        ret!( from_obj::<(c_double, c_double)>( obj ));
    }

    /// Returns `Some` element under the point given by x and y (such
    /// as `TkScrollbarElement::Arrow1`), or `None` if the point does not lie
    /// in any element of the scrollbar. X and y must be pixel coordinates
    /// relative to the scrollbar widget.
    #[cex]
    pub fn identify( &self, x: c_int, y: c_int )
        -> Result!( Option<TkScrollbarElement> throws InterpError, TkScrollbarElementParseError )
    {
        let obj = self.tk().eval(( self.path, "identify", x, y ))?;
        let s = obj.to_string();
        ret!( match s.as_str() {
            "arrow1"  => Some( TkScrollbarElement::Arrow1  ),
            "trough1" => Some( TkScrollbarElement::Trough1 ),
            "slider"  => Some( TkScrollbarElement::Slider  ),
            "trough2" => Some( TkScrollbarElement::Trough2 ),
            "arrow2"  => Some( TkScrollbarElement::Arrow2  ),
            ""        => None,
            _         => throw!( TkScrollbarElementParseError( s )),
        })
    }

    /// This method is invoked by the scrollbar's associated widget to tell the
    /// scrollbar about the current view in the widget. The method takes two
    /// arguments, each of which is a real fraction between 0 and 1. The
    /// fractions describe the range of the document that is visible in the
    /// associated widget. For example, if first is 0.2 and last is 0.4, it
    /// means that the first part of the document visible in the window is 20%
    /// of the way through the document, and the last visible part is 40% of the
    /// way through.
    pub fn set( &self, first: c_double, last: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "set", first, last ))
    }

    pub fn moveto( &self, fraction: c_double ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "moveto", fraction ))
    }

    pub fn scroll_units( &self, number: c_double ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "scroll", number, "units" ))
    }

    pub fn scroll_pages( &self, number: c_double ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "scroll", number, "pages" ))
    }
}
