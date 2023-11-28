use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    TkInstance,
    Widget,
    range::TkRange,
    types::TkBBox,
};

use std::{
    ops::Deref,
    os::raw::{c_double, c_int},
};

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
    },
    from_obj,
};

pub trait TkBBoxTrait<Inst:TkInstance>
    where Self: Deref<Target=Widget<Inst>>
{
    type Index: Into<Obj>;

    #[cex]
    fn bbox( &self, index: Self::Index ) -> Result!( TkBBox throws DeError, InterpError ) {
        let widget = self.deref();
        let obj = widget.tk().eval(( widget.path, "bbox", index ))?;
        let v = from_obj::<[c_int; 4]>( obj )?;
        ret!( TkBBox{ x: v[0], y: v[1], w: v[2], h: v[3] })
    }
}

pub trait TkXView<Inst:TkInstance>
    where Self: Deref<Target=Widget<Inst>>
{
    #[cex]
    fn xview( &self ) -> Result!( (c_double, c_double) throws DeError, InterpError ) {
        let widget = self.deref();
        let obj = widget.tk().eval(( widget.path, "xview" ))?;
        ret!( from_obj::<(c_double, c_double)>( obj ));
    }

    fn xview_moveto( &self, fraction: c_double ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "xview", "moveto", fraction ))
    }

    fn xview_scroll_units( &self, number: c_double ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "xview", "scroll", number, "units" ))
    }

    fn xview_scroll_pages( &self, number: c_double ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "xview", "scroll", number, "pages" ))
    }

    fn xview_( &self, mut args: Vec<Obj> ) -> InterpResult<()> {
        let widget = self.deref();
        let mut command = Vec::<Obj>::with_capacity( 2 + args.len() );
        command.push( widget.path.into() );
        command.push( "xview".into() );
        command.append( &mut args );
        widget.tk().run( command )
    }
}

pub trait TkXViewIndex<Inst:TkInstance>
    where Self: Deref<Target=Widget<Inst>>
{
    type Index: Into<Obj>;

    fn xview_index( &self, index: Self::Index ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "xview", index ))
    }
}

pub trait TkYView<Inst:TkInstance>
    where Self: Deref<Target=Widget<Inst>>
{
    #[cex]
    fn yview( &self ) -> Result!( (c_double, c_double) throws DeError, InterpError ) {
        let widget = self.deref();
        let obj = widget.tk().eval(( widget.path, "yview" ))?;
        ret!( from_obj::<(c_double, c_double)>( obj ));
    }

    fn yview_moveto( &self, fraction: c_double ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "yview", "moveto", fraction ))
    }

    fn yview_scroll_units( &self, number: c_double ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "yview", "scroll", number, "units" ))
    }

    fn yview_scroll_pages( &self, number: c_double ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "yview", "scroll", number, "pages" ))
    }

    fn yview_( &self, mut args: Vec<Obj> ) -> InterpResult<()> {
        let widget = self.deref();
        let mut command = Vec::<Obj>::with_capacity( 2 + args.len() );
        command.push( widget.path.into() );
        command.push( "yview".into() );
        command.append( &mut args );
        widget.tk().run( command )
    }
}

pub trait TkYViewIndex<Inst:TkInstance>
    where Self: Deref<Target=Widget<Inst>>
{
    type Index: Into<Obj>;

    fn yview_index( &self, index: Self::Index ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "yview", index ))
    }
}

pub trait Delete<Inst:TkInstance>
    where Self: Deref<Target=Widget<Inst>>
{
    type Index: Into<Obj>;

    fn delete( &self, index: impl Into<Self::Index> ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "delete", index.into() ))
    }

    fn delete_range( &self, range: impl Into<TkRange<Self::Index>> ) -> InterpResult<()> {
        let widget = self.deref();
        let range = range.into();
        widget.tk().run(( widget.path, "delete", range.start, range.end ))
    }
}

pub trait TkEntryTraits<Inst:TkInstance>
    where Self : Deref<Target=Widget<Inst>>
               + Delete<Inst,Index=<Self as TkEntryTraits<Inst>>::Index>
{
    type Index: Into<Obj>;

    fn get( &self ) -> InterpResult<String> {
        let widget = self.deref();
        Ok( widget.tk().eval(( widget.path, "get" ))?.to_string() )
    }

    fn icursor( &self, index: impl Into<<Self as TkEntryTraits<Inst>>::Index> ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "icursor", index.into() ))
    }

    fn index( &self, index: impl Into<<Self as TkEntryTraits<Inst>>::Index> ) -> InterpResult<c_int> {
        let widget = self.deref();
        let int = widget.tk().eval(( widget.path, "index", index.into() ))?;
        widget.tk().int( int )
    }

    fn insert( &self, index: impl Into<<Self as TkEntryTraits<Inst>>::Index>, string: &str ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "insert", index.into(), string ))
    }

    fn selection_clear( &self ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "selection", "clear" ))
    }

    fn selection_present( &self ) -> InterpResult<bool> {
        let widget = self.deref();
        let boolean = widget.tk().eval(( widget.path, "selection", "present" ))?;
        widget.tk().boolean( boolean )
    }

    fn selection_range( &self,
        start : impl Into<<Self as TkEntryTraits<Inst>>::Index>,
        end   : impl Into<<Self as TkEntryTraits<Inst>>::Index>
    ) -> InterpResult<()> {
        let widget = self.deref();
        widget.tk().run(( widget.path, "selection", "range", start.into(), end.into() ))
    }
}
