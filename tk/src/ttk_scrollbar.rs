use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    TkInstance,
    TkScrollbarDelta,
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
pub struct TtkScrollbar<TK:TkInstance>( pub(crate) Widget<TK> );

impl<TK:TkInstance> TtkScrollbar<TK> {
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

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkScrollbar<TK> {}
