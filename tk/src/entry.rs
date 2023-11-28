use crate::{
    InterpResult,
    TkInstance,
    TkXView,
    TkXViewIndex,
    Widget,
    range::{
        TkDefaultEnd,
        TkDefaultStart,
        TkRange,
    },
    traits::Delete,
};

use std::{
    ops::{
        RangeFrom,
        RangeInclusive,
        RangeToInclusive,
    },
    os::raw::c_int,
};

use tcl::Obj;

#[derive( Copy, Clone )]
pub struct TkEntry<Inst:TkInstance>( pub(crate) Widget<Inst> );

pub enum Index {
    Number( c_int ),
    Anchor,
    End,
    Insert,
    SelFirst,
    SelLast,
    At( c_int ),
}

impl From<c_int> for Index {
    fn from( number: c_int ) -> Self { Index::Number( number )}
}

impl TkDefaultStart for Index {
    fn default_start() -> Self { Index::Number(0) }
}

impl TkDefaultEnd for Index {
    fn default_end() -> Self { Index::End }
}

impl From<RangeFrom<c_int>> for TkRange<Index> { // a..
    fn from( r: RangeFrom<c_int> ) -> Self {
        TkRange {
            start : Index::Number( r.start ),
            end   : Index::default_end()
        }
    }
}

impl From<RangeInclusive<c_int>> for TkRange<Index> { // a..=b
    fn from( r: RangeInclusive<c_int> ) -> Self {
        TkRange {
            start : Index::Number( *r.start() ),
            end   : Index::Number( *r.end() )
        }
    }
}

impl From<RangeToInclusive<c_int>> for TkRange<Index> { // ..=b
    fn from( r: RangeToInclusive<c_int> ) -> Self {
        TkRange {
            start : Index::default_start(),
            end   : Index::Number( r.end ),
        }
    }
}

impl From<Index> for Obj {
    fn from( index: Index ) -> Obj {
        use Index::*;
        match index {
            Number( n ) => n                  .into(),
            Anchor      => "anchor"           .into(),
            End         => "end"              .into(),
            Insert      => "insert"           .into(),
            SelFirst    => "sel.first"        .into(),
            SelLast     => "sel.last"         .into(),
            At( n )     => format!( "@{}", n ).into(),
        }
    }
}

impl<Inst:TkInstance> TkEntry<Inst> {
    pub fn scan_mark( &self, x: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "mark", x ))
    }

    pub fn scan_dragto( &self, x: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "dragto", x ))
    }

    pub fn selection_adjust( &self, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "selection", "adjust", index ))
    }

    pub fn selection_from( &self, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "selection", "from", index ))
    }

    pub fn selection_to( &self, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "selection", "to", index ))
    }

    pub fn validate( &self ) -> InterpResult<bool> {
        self.0.tk()
            .eval(( self.0.path, "validate" ))
            .and_then( |obj| self.0.tk().boolean( obj ))
    }
}

impl<Inst:TkInstance> crate::TkBBoxTrait<Inst> for TkEntry<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> Delete<Inst> for TkEntry<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> crate::TkEntryTraits<Inst> for TkEntry<Inst> {
    type Index = Index;
}

impl<TK:TkInstance> TkXView<TK> for TkEntry<TK> {}

impl<TK:TkInstance> TkXViewIndex<TK> for TkEntry<TK> {
    type Index = Index;
}
