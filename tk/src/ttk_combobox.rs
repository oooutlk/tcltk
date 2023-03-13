use crate::{
    InterpResult,
    TkEntryTraits,
    TkInstance,
    TkXView,
    TtkCommonTraits,
    Widget,
    range::{
        TkDefaultEnd,
        TkDefaultStart,
        TkRange,
    },
    traits::Delete,
};

use tcl::Obj;

use std::{
    ops::{
        RangeFrom,
        RangeInclusive,
        RangeToInclusive,
    },
    os::raw::c_int,
};

#[derive( Copy, Clone )]
pub struct TtkCombobox<TK:TkInstance>( pub(crate) Widget<TK> );

pub enum Index {
    Number( c_int ),
    End,
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
        match index {
            Index::Number( number ) => number.into(),
            Index::End              => "end" .into(),
        }
    }
}

impl<TK:TkInstance> TtkCombobox<TK> {
    pub fn set_current( &self, new_index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "current", new_index ))
    }

    pub fn current( &self ) -> InterpResult<c_int> {
        let obj = self.0.tk().eval(( self.0.path, "current" ))?;
        self.0.tk().int( obj )
    }

    pub fn get( &self ) -> InterpResult<String> {
        self.0.tk().eval(( self.0.path, "get" )).map( |obj| obj.to_string() )
    }

    pub fn set( &self, value: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "set", value ))
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkCombobox<TK> {}

impl<TK:TkInstance> crate::TkBBoxTrait<TK> for TtkCombobox<TK> {
    type Index = super::ttk_entry::Index;
}

impl<Inst:TkInstance> Delete<Inst> for TtkCombobox<Inst> {
    type Index = super::ttk_entry::Index;
}

impl<TK:TkInstance> TkEntryTraits<TK> for TtkCombobox<TK> {
    type Index = super::ttk_entry::Index;
}

impl<TK:TkInstance> TkXView<TK> for TtkCombobox<TK> {
    type Index = super::ttk_entry::Index;
}
