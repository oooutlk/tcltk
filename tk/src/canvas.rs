use crate::{
    InterpError,
    InterpResult,
    OptPair,
    PathOptsWidgets,
    TkInstance,
    TkOption,
    TkRectangle,
    TkXView,
    TkYView,
    Widget,
    WmManage,
    error::{
        DeError,
        DeKind,
        NotList,
        TkCanvasItemTypeParseError
    },
    event::TkEventSeq,
    opt::{
        TkCanvasArcOpt,
        TkCanvasBitmapOpt,
        TkCanvasImageOpt,
        TkCanvasItemOpt,
        TkCanvasLineOpt,
        TkCanvasOvalOpt,
        TkCanvasPolygonOpt,
        TkCanvasPostscriptOpt,
        TkCanvasRectangleOpt,
        TkCanvasTextOpt,
        TkCanvasWindowOpt,
    },
    range::{
        TkDefaultStart,
        TkDefaultEnd,
        TkRange,
    },
};

use std::{
    fmt::{self, Display},
    ops::{
        RangeFrom,
        RangeInclusive,
        RangeToInclusive,
    },
    os::raw::{c_int, c_double},
    str::FromStr,
};

use tcl::Obj;

use tuplex::{IntoHomoTuple, NonZeroLen};

use enumx::export::*;
use enumx::predefined::*;
use cex::*;

#[derive( Copy, Clone )]
pub struct TkCanvas<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> WmManage<Inst> for TkCanvas<Inst> {}

pub enum ItemType {
    Arc, Bitmap, Image, Line, Oval, Polygon, Rectangle, Text, Window,
    ApplicationDefined( String ),
}

impl Display for ItemType {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        use ItemType::*;
        match self {
            Arc                     => "arc"        .fmt( f ),
            Bitmap                  => "bitmap"     .fmt( f ),
            Image                   => "image"      .fmt( f ),
            Line                    => "line"       .fmt( f ),
            Oval                    => "oval"       .fmt( f ),
            Polygon                 => "polygon"    .fmt( f ),
            Rectangle               => "rectangle"  .fmt( f ),
            Text                    => "text"       .fmt( f ),
            Window                  => "window"     .fmt( f ),
            ApplicationDefined( s ) => s            .fmt( f ),
        }
    }
}

impl FromStr for ItemType {
    type Err = TkCanvasItemTypeParseError;

    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        Ok( match s {
            "arc"       => ItemType::Arc,
            "bitmap"    => ItemType::Bitmap,
            "image"     => ItemType::Image,
            "line"      => ItemType::Line,
            "oval"      => ItemType::Oval,
            "polygon"   => ItemType::Polygon,
            "rectangle" => ItemType::Rectangle,
            "text"      => ItemType::Text,
            "window"    => ItemType::Window,
            _ => ItemType::ApplicationDefined( s.to_owned() ),
        })
    }
}

#[derive( Debug, Clone )]
pub struct ItemTag( pub String );
impl From<ItemTag> for Obj { fn from( tag: ItemTag ) -> Obj { tag.0.into() }}

#[derive( Debug, Clone )]
pub struct ItemId(  pub String );
impl From<ItemId> for Obj { fn from( id: ItemId ) -> Obj { id.0.into() }}

pub fn item_tag( name: &str ) -> ItemTag { ItemTag( name.to_owned() )}
pub fn item_id( name: &str ) -> ItemId { ItemId( name.to_owned() )}

pub enum TagOrId {
    Tag( String ),
    Id(  String ),
}

impl From<ItemTag> for TagOrId { fn from( tag: ItemTag ) -> Self { TagOrId::Tag( tag.0 )}}
impl From<ItemId> for TagOrId { fn from( id: ItemId ) -> Self { TagOrId::Id( id.0 )}}

impl From<TagOrId> for Obj {
    fn from( tag_or_id: TagOrId ) -> Obj {
        match tag_or_id {
            TagOrId::Tag(v) | TagOrId::Id(v) => v.into()
        }
    }
}

pub enum SearchSpec {
    Above( TagOrId ),
    All,
    Below( TagOrId ),
    Closest{ x: c_int, y: c_int, halo: Option<c_double>, start: Option<TagOrId> },
    Enclosed{ x1: c_int, y1: c_int, x2: c_int, y2: c_int },
    Overlapping{ x1: c_int, y1: c_int, x2: c_int, y2: c_int },
    WithTag( TagOrId ),
}

pub enum Index {
    Number( c_int ),
    End,
    Insert,
    SelFirst,
    SelLast,
    At( c_int, c_int ),
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
            Number( number )    => number.into(),
            End                 => "end".into(),
            Insert              => "insert".into(),
            SelFirst            => "sel.first".into(),
            SelLast             => "sel.last".into(),
            At( x, y )          => format!( "@{},{}", x, y ).into(),
        }
    }
}

impl<Inst:TkInstance> TkCanvas<Inst> {
    pub fn addtag( &self, tag: &str, search_spec: SearchSpec ) -> InterpResult<()> {
        use SearchSpec::*;
        match search_spec {
            Above( tag_or_id ) => self.tk().run(( self.path, "addtag", tag, "above", tag_or_id )),
            All => self.tk().run(( self.path, "addtag", tag, "all" )),
            Below( tag_or_id ) => self.tk().run(( self.path, "addtag", tag, "below", tag_or_id )),
            Closest{ x, y, halo, start } => self.tk().run(( self.path, "addtag", tag, "closest", x, y, halo, start )),
            Enclosed{ x1, y1, x2, y2 } => self.tk().run(( self.path, "addtag", tag, "enclosed", x1, y1, x2, y2 )),
            Overlapping{ x1, y1, x2, y2 } => self.tk().run(( self.path, "addtag", tag, "overlapping", x1, y1, x2, y2 )),
            WithTag( tag_or_id ) => self.tk().run(( self.path, "addtag", tag, "withtag", tag_or_id )),
        }
    }

    #[cex]
    pub fn bbox<ListOfTagOrId,TupleOfTagOrId>( &self, list_of_tag_or_id: ListOfTagOrId )
        -> Result!( Option<TkRectangle> throws InterpError, DeError )
        where ListOfTagOrId: IntoHomoTuple<Obj> + NonZeroLen<TupleOfTagOrId>
            , <ListOfTagOrId as IntoHomoTuple<Obj>>::Output : Into<Obj>
    {
        let obj = self.tk().eval(( "eval", self.path, "bbox", list_of_tag_or_id.into_homo_tuple() ))?;
        if obj.is_empty() {
            return Ok( None );
        }

        let mut ints = Vec::with_capacity( 4 );
        let err_obj= obj.clone();
        let list = obj.get_elements().map_err( |_| DeError::new( DeKind::NotList, err_obj.clone() ))?;
        for elem in list {
            ints.push( self.tk().int( elem )? );
        }

        if ints.len() == 4 {
            return Ok( Some( TkRectangle{ left:ints[0], top:ints[1], right:ints[2], bottom:ints[3] }));
        } else {
            throw!( DeError::new( DeKind::ListLen{ expected:4, got: ints.len() }, err_obj ));
        }
    }

    pub fn bind( &self, tag_or_id: impl Into<TagOrId>, sequence: impl Into<TkEventSeq>, command: impl Into<Obj> )
        -> InterpResult<()>
    {
        self.tk().run(( self.path, "bind", tag_or_id.into(), sequence.into(), command ))
    }

    pub fn bind_more( &self, tag_or_id: impl Into<TagOrId>, sequence: impl Into<TkEventSeq>, command: impl Into<Obj> )
        -> InterpResult<()>
    {
        let command = format!( "+{}", command.into().to_string() );
        self.tk().run(( self.path, "bind", tag_or_id.into(), sequence.into(), command ))
    }

    pub fn canvasx( &self, screenx: c_double, gridspacing: Option<c_double> ) -> InterpResult<c_double> {
        let result = match gridspacing {
            Some( gridspacing ) => self.tk().eval(( self.path, "canvasx", screenx, gridspacing )),
            None => self.tk().eval(( self.path, "canvasx", screenx )),
        };
        self.tk().double( result? )
    }

    pub fn canvasy( &self, screenx: c_double, gridspacing: Option<c_double> ) -> InterpResult<c_double> {
        let result = match gridspacing {
            Some( gridspacing ) => self.tk().eval(( self.path, "canvasy", screenx, gridspacing )),
            None => self.tk().eval(( self.path, "canvasy", screenx )),
        };
        self.tk().double( result? )
    }

    pub fn coords( &self, tag_or_id: impl Into<TagOrId> ) -> InterpResult<Obj> {
        self.tk().eval(( self.path, "coords", tag_or_id.into() ))
    }

    pub fn set_coords( &self, tag_or_id: impl Into<TagOrId>, coords_list: Obj ) -> InterpResult<()> {
        self.tk().run(( self.path, "coords", tag_or_id.into(), coords_list ))
    }

    pub fn create_arc<Opts>( &self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasArcOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 7 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "arc".into() );
        command.push( x1.into() );
        command.push( y1.into() );
        command.push( x2.into() );
        command.push( y2.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_bitmap<Opts>( &self, x: c_double, y: c_double, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasBitmapOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 5 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "bitmap".into() );
        command.push( x.into() );
        command.push( y.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_image<Opts>( &self, x: c_double, y: c_double, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasImageOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 5 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "image".into() );
        command.push( x.into() );
        command.push( y.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_line<Opts>( &self, coord_list: &[(c_double,c_double)], opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasLineOpt>
                  + IntoHomoTuple<OptPair>
    {
        let opts_len = <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN;
        let mut command = Vec::<Obj>::with_capacity( coord_list.len() * 2 + opts_len * 2 + 3 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "line".into() );
        coord_list.iter().for_each( |(x, y)| {
            command.push( (*x).into() );
            command.push( (*y).into() );
        });
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_oval<Opts>( &self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasOvalOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 7 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "oval".into() );
        command.push( x1.into() );
        command.push( y1.into() );
        command.push( x2.into() );
        command.push( y2.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_polygon<Opts>( &self, coord_list: &[(c_double,c_double)], opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasPolygonOpt>
                  + IntoHomoTuple<OptPair>
    {
        let opts_len = <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN;
        let mut command = Vec::<Obj>::with_capacity( coord_list.len() * 2 + opts_len * 2 + 3 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "polygon".into() );
        coord_list.iter().for_each( |(x, y)| {
            command.push( (*x).into() );
            command.push( (*y).into() );
        });
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_rectangle<Opts>( &self,
        x1: c_double, y1: c_double, x2: c_double, y2: c_double,
        opts: impl Into<PathOptsWidgets<Opts,()>>
    )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasRectangleOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 7 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "rectangle".into() );
        command.push( x1.into() );
        command.push( y1.into() );
        command.push( x2.into() );
        command.push( y2.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_text<Opts>( &self, x: c_double, y: c_double, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasTextOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 5 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "text".into() );
        command.push( x.into() );
        command.push( y.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn create_window<Opts>( &self, x: c_double, y: c_double, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<ItemId>
        where Opts: IntoHomoTuple<TkCanvasWindowOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 5 );
        command.push( self.path.into() );
        command.push( "create".into() );
        command.push( "window".into() );
        command.push( x.into() );
        command.push( y.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        Ok( ItemId( self.tk().eval( command )?.to_string() ))
    }

    pub fn dchars( &self, tag_or_id: impl Into<TagOrId>, index: impl Into<Index> ) -> InterpResult<()> {
        self.tk().run(( self.path, "dchars", tag_or_id.into(), index.into() ))
    }

    pub fn dchars_range( &self, tag_or_id: impl Into<TagOrId>, range: impl Into<TkRange<Index>> ) -> InterpResult<()> {
        let range = range.into();
        self.tk().run(( self.path, "dchars", tag_or_id.into(), range.start, range.end ))
    }

    pub fn delete<ListOfTagOrId,TupleOfTagOrId>( &self, list_of_tag_or_id: ListOfTagOrId ) -> InterpResult<()>
        where ListOfTagOrId: IntoHomoTuple<Obj> + NonZeroLen<TupleOfTagOrId>
            , <ListOfTagOrId as IntoHomoTuple<Obj>>::Output : Into<Obj>
    {
        self.tk().run(( "eval", self.path, "delete", list_of_tag_or_id.into_homo_tuple() ))
    }

    pub fn dtag( &self, tag_or_id: impl Into<TagOrId>, tag_to_delete: Option<ItemTag> ) -> InterpResult<()> {
        match tag_to_delete {
            Some( tag ) => self.tk().run(( self.path, "dtag", tag_or_id.into(), tag )),
            None        => self.tk().run(( self.path, "dtag", tag_or_id.into() )),
        }
    }

    pub fn find( &self, search_spec: SearchSpec ) -> InterpResult<Obj> {
        use SearchSpec::*;
        match search_spec {
            Above( tag_or_id ) => self.tk().eval(( self.path, "find", "above", tag_or_id )),
            All => self.tk().eval(( self.path, "find", "all" )),
            Below( tag_or_id ) => self.tk().eval(( self.path, "find", "below", tag_or_id )),
            Closest{ x, y, halo, start } => self.tk().eval(( self.path, "find", "closest", x, y, halo, start )),
            Enclosed{ x1, y1, x2, y2 } => self.tk().eval(( self.path, "find", "enclosed", x1, y1, x2, y2 )),
            Overlapping{ x1, y1, x2, y2 } => self.tk().eval(( self.path, "find", "overlapping", x1, y1, x2, y2 )),
            WithTag( tag_or_id ) => self.tk().eval(( self.path, "find", "withtag", tag_or_id )),
        }
    }

    pub fn focus( &self ) -> InterpResult<Option<ItemId>> {
        let item_id = self.tk().eval(( self.path, "focus" ))?.to_string();
        if item_id.is_empty() {
            Ok( None )
        } else {
            Ok( Some( ItemId( item_id )))
        }
    }

    pub fn set_focus( &self, tag_or_id: impl Into<TagOrId> ) -> InterpResult<()> {
        self.tk().run(( self.path, "focus", tag_or_id.into() ))
    }

    pub fn clear_focus( &self ) -> InterpResult<()> {
        self.tk().run(( self.path, "focus", "" ))
    }

    #[cex]
    pub fn gettags( &self, tag_or_id: impl Into<TagOrId> ) -> Result!( Vec<ItemTag> throws InterpError, NotList ) {
        let obj = self.tk().eval(( self.path, "gettags", tag_or_id.into() ))?;
        Ok( obj .get_elements()?
                .map( |obj| ItemTag( obj.to_string() ))
                .collect::<Vec<_>>() )
    }

    pub fn icursor( &self, tag_or_id: impl Into<TagOrId>, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.path, "icursor", tag_or_id.into(), index ))
    }

    pub fn imove( &self, tag_or_id: impl Into<TagOrId>, index: Index, x: c_double, y: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "imove", tag_or_id.into(), index, x, y ))
    }

    pub fn index( &self, tag_or_id: impl Into<TagOrId>, index: Index ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "index", tag_or_id.into(), index ))?;
        self.tk().int( obj )
    }

    pub fn insert( &self, tag_or_id: impl Into<TagOrId>, before_this: Index, to_insert: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( self.path, "insert", tag_or_id.into(), before_this, to_insert ))
    }

    pub fn itemcget<Opt,Val>( &self, tag_or_id: impl Into<TagOrId>, _name_fn: fn(Val)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkCanvasItemOpt>
            , Val : Into<Obj>
    {
        self.tk().eval(( self.path, "itemcget", tag_or_id.into(), <Opt as TkOption>::NAME ))
    }

    pub fn itemconfigure<Opts>( &self, tag_or_id: impl Into<TagOrId>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkCanvasItemOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "itemconfigure".into() );
        command.push( tag_or_id.into().into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn lower( &self, tag_or_id: impl Into<TagOrId>, below_this: Option<TagOrId> ) -> InterpResult<()> {
        match below_this {
            Some( below_this ) => self.tk().run(( self.path, "lower", tag_or_id.into(), below_this )),
            None => self.tk().run(( self.path, "lower", tag_or_id.into() )),
        }
    }

    pub fn move_( &self, tag_or_id: impl Into<TagOrId>, x_amount: c_double, y_amount: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "move", tag_or_id.into(), x_amount, y_amount ))
    }

    pub fn move_to( &self, tag_or_id: impl Into<TagOrId>, x_pos: c_double, y_pos: c_double ) -> InterpResult<()> {
        self.tk().run(( self.path, "moveto", tag_or_id.into(), x_pos, y_pos ))
    }

    pub fn postscript<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts: IntoHomoTuple<TkCanvasPostscriptOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );
        command.push( self.path.into() );
        command.push( "postscript".into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().eval( command )
    }

    pub fn raise( &self, tag_or_id: impl Into<TagOrId>, above_this: Option<TagOrId> ) -> InterpResult<()> {
        match above_this {
            Some( above_this ) => self.tk().run(( self.path, "raise", tag_or_id.into(), above_this )),
            None => self.tk().run(( self.path, "raise", tag_or_id.into() )),
        }
    }

    pub fn rchars( &self, tag_or_id: impl Into<TagOrId>, first: Index, last: Index, to_replace: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( self.path, "rchars", tag_or_id.into(), first, last, to_replace ))
    }

    pub fn scale( &self, tag_or_id: impl Into<TagOrId>, x_origin: c_double, y_origin: c_double, x_scale: c_double, y_scale: c_double )
        -> InterpResult<()>
    {
        self.tk().run(( self.path, "scale", tag_or_id.into(), x_origin, y_origin, x_scale, y_scale ))
    }

    pub fn scan_mark( &self, x: c_int, y: c_int ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "mark", x, y ))
    }

    pub fn scan_dragto( &self, x: c_int, y: c_int, gain: Option<c_int> ) -> InterpResult<()> {
        match gain {
            Some( gain ) => self.tk().run(( self.path, "scan", "dragto", x, y, gain )),
            None         => self.tk().run(( self.path, "scan", "dragto", x, y )),
        }
    }

    pub fn select_adjust( &self, tag_or_id: impl Into<TagOrId>, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.path, "select", "adjust", tag_or_id.into(), index ))
    }

    pub fn select_clear( &self ) -> InterpResult<()> {
        self.tk().run(( self.path, "select", "clear" ))
    }

    pub fn select_from( &self, tag_or_id: impl Into<TagOrId>, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.path, "select", "from", tag_or_id.into(), index ))
    }

    pub fn select_item( &self ) -> InterpResult<Option<ItemId>> {
        let result = self.tk().eval(( self.path, "select", "item", ))?.to_string();
        if result.is_empty() {
            Ok( None )
        } else {
            Ok( Some( ItemId( result )))
        }
    }

    pub fn select_to( &self, tag_or_id: impl Into<TagOrId>, index: Index ) -> InterpResult<()> {
        self.tk().run(( self.path, "select", "to", tag_or_id.into(), index ))
    }

    pub fn type_( &self, tag_or_id: impl Into<TagOrId> ) -> InterpResult<Option<ItemType>> {
        let result = self.tk().eval(( self.path, "type", tag_or_id.into() ))?.to_string();
        if result.is_empty() {
            Ok( None )
        } else {
            Ok( Some( <ItemType as FromStr>::from_str( &result ).unwrap() ))
        }
    }
}

impl<TK:TkInstance> TkXView<TK> for TkCanvas<TK> {}

impl<TK:TkInstance> TkYView<TK> for TkCanvas<TK> {}
