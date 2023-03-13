use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    TkInstance,
    Widget,
    entry::Index,
    error::TkSpinboxElementParseError,
    traits::Delete,
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
pub struct TkSpinbox<Inst:TkInstance>( pub(crate) Widget<Inst> );

#[derive( Debug )]
pub enum TkSpinboxElement {
    None, ButtonDown, ButtonUp, Entry,
}

#[derive( Debug )]
pub enum TkSpinboxInvokableElement {
    ButtonDown, ButtonUp,
}

impl From<TkSpinboxInvokableElement> for Obj {
    fn from( element: TkSpinboxInvokableElement ) -> Obj {
        match element {
            TkSpinboxInvokableElement::ButtonDown => "buttondown".into(),
            TkSpinboxInvokableElement::ButtonUp   => "buttonup"  .into(),
        }
    }
}

impl<Inst:TkInstance> TkSpinbox<Inst> {
    pub fn get( &self ) -> InterpResult<String> {
        self.tk().eval(( self.0.path, "get" )).map( |obj| obj.to_string() )
    }

    pub fn icursor( &self, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "icursor", index ))
    }

    #[cex]
    pub fn identify( &self, x: c_int, y: c_int )
        -> Result!( Option<TkSpinboxElement> throws InterpError, TkSpinboxElementParseError )
    {
        let obj = self.tk().eval(( self.path, "identify", x, y ))?;
        let s = obj.to_string();
        ret!( match s.as_str() {
            "none"       => Some( TkSpinboxElement::None       ),
            "buttondown" => Some( TkSpinboxElement::ButtonDown ),
            "buttonup"   => Some( TkSpinboxElement::ButtonUp   ),
            "entry"      => Some( TkSpinboxElement::Entry      ),
            ""           => None,
            _            => throw!( TkSpinboxElementParseError( s )),
        })
    }

    pub fn index( &self, index: Index ) -> InterpResult<c_int> {
        let int = self.tk().eval(( self.0.path, "index", index ))?;
        self.tk().int( int )
    }

    pub fn insert( &self, index: Index, string: &str ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "insert", index, string ))
    }

    pub fn invoke( &self, element: TkSpinboxInvokableElement ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "invoke", element ))
    }

    pub fn scan_mark( &self, x: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "scan", "mark", x ))
    }

    pub fn scan_dragto( &self, x: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "scan", "dragto", x ))
    }

    pub fn selection_adjust( &self, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "selection", "adjust", index ))
    }

    pub fn selection_clear( &self ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "selection", "clear" ))
    }

    pub fn selection_element( &self,  ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "selection", "clear" ))
    }

    pub fn selection_from( &self, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "selection", "from", index ))
    }

    pub fn selection_present( &self ) -> InterpResult<bool> {
        let boolean = self.tk().eval(( self.0.path, "selection", "present" ))?;
        self.tk().boolean( boolean )
    }

    pub fn selection_range( &self, start: Index, end: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "selection", "range", start, end ))
    }

    pub fn selection_to( &self, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "selection", "to", index ))
    }

    pub fn set( &self, value: &str ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "set", value ))
    }

    pub fn validate( &self ) -> InterpResult<bool> {
        let boolean = self.tk().eval(( self.0.path, "validate" ))?;
        self.tk().boolean( boolean )
    }

    #[cex]
    pub fn xview( &self ) -> Result!( (c_double, c_double) throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.0.path, "xview" ))?;
        ret!( from_obj::<(c_double, c_double)>( obj ));
    }

    pub fn xview_index( &self, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "xview", index ))
    }

    pub fn xview_moveto( &self, fraction: c_double ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "xview", "moveto", fraction ))
    }

    pub fn xview_scroll_units( &self, number: c_double ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "xview", "scroll", number, "units" ))
    }

    pub fn xview_scroll_pages( &self, number: c_double ) -> InterpResult<()> {
        self.tk().run(( self.0.path, "xview", "scroll", number, "pages" ))
    }
}

impl<Inst:TkInstance> crate::TkBBoxTrait<Inst> for TkSpinbox<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> Delete<Inst> for TkSpinbox<Inst> {
    type Index = Index;
}
