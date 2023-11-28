use crate::{
    InterpResult,
    TkBBoxTrait,
    TkEntryTraits,
    TkInstance,
    TkXView,
    TkXViewIndex,
    TtkCommonTraits,
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
pub struct TtkEntry<Inst:TkInstance>( pub(crate) Widget<Inst> );

#[derive( Clone )]
pub enum Index {
    Number( c_int ),
    At( c_int ),
    End,
    Insert,
    SelFirst,
    SelLast,
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
            At( n )     => format!( "@{}", n ).into(),
            End         => "end"              .into(),
            Insert      => "insert"           .into(),
            SelFirst    => "sel.first"        .into(),
            SelLast     => "sel.last"         .into(),
        }
    }
}

impl<Inst:TkInstance> TtkEntry<Inst> {
    pub fn icursor( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "icursor", index.into() ))
    }

    pub fn validate( &self ) -> InterpResult<bool> {
        let boolean = self.0.tk().eval(( self.0.path, "validate" ))?;
        self.0.tk().boolean( boolean )
    }
}

impl<Inst:TkInstance> TtkCommonTraits<Inst> for TtkEntry<Inst> {}

impl<Inst:TkInstance> TkBBoxTrait<Inst> for TtkEntry<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> Delete<Inst> for TtkEntry<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> TkEntryTraits<Inst> for TtkEntry<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> TkXView<Inst> for TtkEntry<Inst> {}

impl<Inst:TkInstance> TkXViewIndex<Inst> for TtkEntry<Inst> {
    type Index = Index;
}
