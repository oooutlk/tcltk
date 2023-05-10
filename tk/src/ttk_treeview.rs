use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    PathOptsWidgets,
    TkBBox,
    TkInstance,
    TkOption,
    TkXView,
    TkYView,
    TtkCommonTraits,
    TtkTreeviewRegion,
    Widget,
    cmd::append_opts,
    opt::{
        OptPair,
        TtkTreeviewColumnOpt,
        TtkTreeviewHeadingOpt,
        TtkTreeviewItemOpt,
        TtkTreeviewTagOpt,
    },
    range::{
        TkDefaultStart,
        TkDefaultEnd,
        TkRange,
    },
};

use std::{
    collections::HashMap,
    ops::{
        RangeFrom,
        RangeInclusive,
        RangeToInclusive,
    },
    os::raw::c_int,
};

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
        NotDict,
        NotList,
    },
    from_obj,
};

use tuplex::*;

#[derive( Copy, Clone )]
pub struct TtkTreeview<TK:TkInstance>( pub(crate) Widget<TK> );

pub enum Index {
    Number( c_int ),
    End
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
            Index::Number(n) => n    .into(),
            Index::End       => "end".into(),
        }
    }
}

pub enum Column {
    Name( &'static str ),
    Nth( c_int ),
    NthDisplay( c_int ),
}

impl From<&'static str> for Column {
    fn from( name: &'static str ) -> Self { Column::Name( name )}
}

impl From<Column> for Obj {
    fn from( id: Column ) -> Obj {
        use Column::*;
        match id {
            Name(       n ) => n.into(),
            Nth(        n ) => n.into(),
            NthDisplay( n ) => n.into(),
        }
    }
}

impl<TK:TkInstance> TtkTreeview<TK> {
    #[cex]
    pub fn bbox( &self, item: &str ) -> Result!( Option<TkBBox> throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "bbox", item ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            let v = from_obj::<[c_int; 4]>( obj )?;
            ret!( Some( TkBBox{ x: v[0], y: v[1], w: v[2], h: v[3] }))
        }
    }

    #[cex]
    pub fn bbox_of_column( &self, item: &str, column: impl Into<Column> ) -> Result!( Option<TkBBox> throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "bbox", item, column.into() ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            let v = from_obj::<[c_int; 4]>( obj )?;
            ret!( Some( TkBBox{ x: v[0], y: v[1], w: v[2], h: v[3] }))
        }
    }

    pub fn set_children( &self, item: &str, new_children: Vec<String> ) -> InterpResult<()> {
        self.tk().run(( self.path, "children", item, new_children ))
    }

    #[cex]
    pub fn children( &self, item: &str ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.tk().eval(( self.path, "children", item ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn column<Opt>( &self, column: impl Into<Column>, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TtkTreeviewColumnOpt>
    {
        self.tk().eval(( self.path, "column", column.into(), <Opt as TkOption>::NAME ))
    }

    pub fn set_column<Opts>( &self, column: impl Into<Column>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TtkTreeviewColumnOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "column".into() );
        command.push( Obj::from( column.into() ));

        append_opts( &mut command, opts.into().opts );
        self.tk().eval( command )
    }

    pub fn delete( &self, item_list: Vec<String> ) -> InterpResult<()> {
        self.tk().run(( self.path, "delete", item_list ))
    }

    pub fn detach( &self, item_list: Vec<String> ) -> InterpResult<()> {
        self.tk().run(( self.path, "detach", item_list ))
    }

    pub fn exists( &self, item: &str ) -> InterpResult<bool> {
        let obj = self.tk().eval(( self.path, "exists", item ))?;
        self.tk().boolean( obj )
    }

    pub fn set_focus( &self, item: &str ) -> InterpResult<()> {
        self.tk().run(( self.path, "focus", item ))
    }

    pub fn focus( &self ) -> InterpResult<Option<String>> {
        let obj = self.tk().eval(( self.path, "focus" ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            Ok( Some( obj.to_string() ))
        }
    }

    pub fn heading<Opt>( &self, column: impl Into<Column>, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TtkTreeviewHeadingOpt>
    {
        self.tk().eval(( self.path, "heading", column.into(), <Opt as TkOption>::NAME ))
    }

    pub fn set_heading<Opts>( &self, column: impl Into<Column>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TtkTreeviewHeadingOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "heading".into() );
        command.push( Obj::from( column.into() ));

        append_opts( &mut command, opts.into().opts );
        self.tk().eval( command )
    }

    #[cex]
    pub fn identify_region( &self, x: c_int, y: c_int ) -> Result!( Option<TtkTreeviewRegion> throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "identify", "region", x, y ))?;

        if obj.is_empty() {
            Ok( None )
        } else {
            ret!( from_obj::<TtkTreeviewRegion>( obj ).map( Some ));
        }
    }

    pub fn identify_item( &self, x: c_int, y: c_int ) -> InterpResult<String> {
        self.tk().eval(( self.path, "identify", "item", x, y )).map( |obj| obj.to_string() )
    }

    pub fn identify_column( &self, x: c_int, y: c_int ) ->InterpResult<String> {
        self.tk().eval(( self.path, "identify", "column", x, y )).map( |obj| obj.to_string() )
    }

    pub fn index( &self, item: &str ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "index", item ))?;
        self.tk().int( obj )
    }

    pub fn insert<Opts>( &self, parent: &str, index: impl Into<Index>, id_opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<Option<String>>
        where Opts : IntoHomoTuple<TtkTreeviewItemOpt>
                   + IntoHomoTuple<OptPair>
    {
        let id_opts = id_opts.into();

        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 6 );
        command.push( self.path.into() );
        command.push( "insert".into() );
        command.push( parent.into() );
        command.push( index.into().into() );

        let id_assigned = !id_opts.path.is_empty();

        if id_assigned {
            command.push( "-id".into() );
            command.push( id_opts.path.into() );
        }

        append_opts( &mut command, id_opts.opts );

        if id_assigned {
            self.tk().run( command )?;
            Ok( None )
        } else {
            self.tk().eval( command ).map( |obj| Some( obj.to_string() ))
        }
    }

    pub fn item<Opt>( &self, column: impl Into<Column>, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TtkTreeviewItemOpt>
    {
        self.tk().eval(( self.path, "item", column.into(), <Opt as TkOption>::NAME ))
    }

    pub fn set_item<Opts>( &self, column: impl Into<Column>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TtkTreeviewItemOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "item".into() );
        command.push( Obj::from( column.into() ));

        append_opts( &mut command, opts.into().opts );
        self.tk().eval( command )
    }

    pub fn move_item( &self, item: &str, parent: &str, index: impl Into<Index> ) -> InterpResult<()> {
        self.tk().run(( self.path, "move", item, parent, index.into() ))
    }

    pub fn next_item( &self, item: &str ) -> InterpResult<Option<String>> {
        let obj = self.tk().eval(( self.path, "next", item ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            Ok( Some( obj.to_string() ))
        }
    }

    pub fn parent_item( &self, item: &str ) -> InterpResult<Option<String>> {
        let obj = self.tk().eval(( self.path, "parent", item ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            Ok( Some( obj.to_string() ))
        }
    }

    pub fn prev_item( &self, item: &str ) -> InterpResult<Option<String>> {
        let obj = self.tk().eval(( self.path, "prev", item ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            Ok( Some( obj.to_string() ))
        }
    }

    pub fn see_item( &self, item: &str ) -> InterpResult<()> {
        self.tk().run(( self.path, "see", item ))
    }

    fn selection( &self, sub_cmd: &'static str, item_list: Vec<String> ) -> InterpResult<()> {
        self.tk().run(( self.path, "selection", sub_cmd, item_list ))
    }

    pub fn selection_add( &self, item_list: Vec<String> ) -> InterpResult<()> {
        self.selection( "add", item_list )
    }

    pub fn selection_set( &self, item_list: Vec<String> ) -> InterpResult<()> {
        self.selection( "set", item_list )
    }

    pub fn selection_remove( &self, item_list: Vec<String> ) -> InterpResult<()> {
        self.selection( "remove", item_list )
    }

    pub fn selection_toggle( &self, item_list: Vec<String> ) -> InterpResult<()> {
        self.selection( "toggle", item_list )
    }

    pub fn set_item_at_column( &self, item: &str, column: impl Into<Column>, value: impl Into<Obj> )
        -> InterpResult<()>
    {
        self.tk().run(( self.path, "set", item, column.into(), value ))
    }

    pub fn item_at_column( &self, item: &str, column: impl Into<Column> ) -> InterpResult<Obj> {
        self.tk().eval(( self.path, "set", item, column.into() ))
    }

    #[cex]
    pub fn item_columns_values( &self, item: &str ) -> Result!( HashMap<String,Obj> throws InterpError, NotDict, NotList ) {
        let obj = self.tk().eval(( self.path, "set", item ))?;

        let list = obj.clone().get_elements()?;
        let mut map = HashMap::with_capacity( list.size_hint().0 / 2 );
        let mut column = None::<String>;
        for elem in list {
            if column.is_some() {
                map.entry( column.take().unwrap().to_string() ).or_insert( elem );
            } else {
                column = Some( elem.to_string() );
            }
        }
        if column.is_some() {
            throw!( NotDict( obj ))
        } else {
            Ok( map )
        }
    }

    pub fn tag_bind( &self, tag_name: &str, sequence: impl Into<Obj>, script: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( self.path, "tag", "bind", tag_name, sequence.into(), script.into() ))
    }

    pub fn tag<Opt>( &self, tag: &str, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TtkTreeviewTagOpt>
    {
        self.tk().eval(( self.path, "tag", tag, <Opt as TkOption>::NAME ))
    }

    pub fn tag_configure<Opts>( &self, tag_name: &str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TtkTreeviewTagOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "configure".into() );
        command.push( tag_name.into() );

        append_opts( &mut command, opts.into().opts );
        self.tk().eval( command )
    }

    #[cex]
    pub fn tag_has( &self, tag_name: &str ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.tk().eval(( self.path, "tag", "has", tag_name ))?;
        Ok( obj.get_elements()?.map( |elem| elem.to_string() ).collect() )
    }

    pub fn tag_has_item( &self, tag_name: &str, item: &str ) -> InterpResult<bool> {
        let obj = self.tk().eval(( self.path, "tag", "has", tag_name, item ))?;
        self.tk().boolean( obj )
    }

    #[cex]
    pub fn tag_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.tk().eval(( self.path, "tag", "names" ))?;
        Ok( obj.get_elements()?.map( |elem| elem.to_string() ).collect() )
    }

    pub fn tag_add( &self, tag: &str, items: &[&str] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( items.len() + 4 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "add".into() );
        command.push( tag.into() );

        items.into_iter().for_each( |&item| {
            command.push( item.into() );
        });

        self.tk().run( command )
    }

    pub fn tag_remove( &self, tag: &str, items: &[&str] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( items.len() + 4 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "remove".into() );
        command.push( tag.into() );

        items.into_iter().for_each( |&item| {
            command.push( item.into() );
        });

        self.tk().run( command )
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkTreeview<TK> {}

impl<TK:TkInstance> TkXView<TK> for TtkTreeview<TK> {
    type Index = Index;
}

impl<TK:TkInstance> TkYView<TK> for TtkTreeview<TK> {
    type Index = Index;
}
