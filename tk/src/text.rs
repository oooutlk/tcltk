use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use bitflags::bitflags;

use crate::{
    InterpResult,
    PathOptsWidgets,
    TkDLine,
    TkDefaultEnd,
    TkInstance,
    TkOption,
    TkXView,
    TkXViewIndex,
    TkYView,
    TkYViewIndex,
    Widget,
    error::{
        TagRangesNotInPair,
        TkDumpParseError,
        TkIndexParseError,
        TkTextMarkGravityParseError,
    },
    opt::{
        OptPair,
        TkEmbededImageOpt,
        TkEmbededWindowOpt,
        TkTextTagOpt,
    },
    range::TkRange,
    traits::Delete,
};

use std::{
    fmt::{self, Display},
    ops::{
        Bound,
        Range,
        RangeBounds,
    },
    os::raw::{
        c_double,
        c_int,
    },
    str::FromStr,
};

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
        NotList,
    },
    from_obj,
};

use tuplex::*;

#[derive( Copy, Clone )]
pub struct TkText<Inst:TkInstance>( pub(crate) Widget<Inst> );

#[derive( Clone, Debug, PartialEq, Eq)]
pub enum Index {
    LineChar( c_int, c_int, Option<String> ),
    LineEnd( c_int, Option<String> ),
    At( c_int, c_int, Option<String> ),
    End( Option<String> ),
    Mark( String, Option<String> ),
    TagFirst( String, Option<String> ),
    TagLast( String, Option<String> ),
    Path( String, Option<String> ),
    Image( String, Option<String> ),
}

impl TkDefaultEnd for Index {
    fn default_end() -> Self { Index::End( None )}
}

// Only "line.char" is acceptable
impl FromStr for Index {
    type Err = TkIndexParseError;
    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        let line_char = s.split('.').collect::<Vec<_>>();
        if line_char.len() == 2 {
            let result: Result<Self, std::num::ParseIntError> = ( || {
                let line = line_char[0].parse::<c_int>()?;
                let ch   = line_char[1].parse::<c_int>()?;
                Ok( Index::LineChar( line, ch, None ))
            })();
            if let Ok( result ) = result {
                return Ok( result );
            }
        }
        return Err( TkIndexParseError( s.to_owned() ));
    }
}

impl Display for Index {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        use Index::*;
        match self {
            LineChar( line, ch, modifiers ) =>
                write!( formatter, "{}.{}{}", line, ch, modifiers.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
            LineEnd( line, modifiers ) =>
                write!( formatter, "{}.end{}", line, modifiers.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
            At( x, y, modifiers ) =>
                write!( formatter, "@{},{}{}", x, y, modifiers.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
            End( modifiers ) =>
                write!( formatter, "end{}", modifiers.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
            Mark(s,m) | Path(s,m) | Image(s,m) =>
                write!( formatter, "{}{}", s, m.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
            TagFirst(s,m) =>
                write!( formatter, "{}.first{}", s, m.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
            TagLast(s,m) =>
                write!( formatter, "{}.last{}", s, m.as_ref().map(|m|m.as_str()).unwrap_or_default() ),
        }
    }
}

impl From<Index> for Obj {
    fn from( index: Index ) -> Obj {
        Obj::from( index.to_string() )
    }
}

impl Index {
    pub fn line_char( line: c_int, ch: c_int ) -> Self { Index::LineChar( line, ch, None )}
    pub fn line_end( line: c_int ) -> Self { Index::LineEnd( line, None )}
    pub fn at( x: c_int, y: c_int ) -> Self { Index::At( x, y, None )}
    pub fn end() -> Self { Index::End( None )}
    pub fn mark( name: &str ) -> Self { Index::Mark( name.to_owned(), None )}
    pub fn tag_first( name: &str ) -> Self { Index::TagFirst( name.to_owned(), None )}
    pub fn tag_last( name: &str ) -> Self { Index::TagLast( name.to_owned(), None )}
    pub fn path( name: &str ) -> Self { Index::Path( name.to_owned(), None )}
    pub fn image( name: &str ) -> Self { Index::Image( name.to_owned(), None )}

    fn counter_modifier( mut self, count: c_int, modifier: &'static str ) -> Self {
        use Index::*;
        match &mut self {
            LineChar( _, _, m ) |
            LineEnd( _, m )     |
            At( _, _, m )       |
            End( m )            |
            Mark( _, m )        |
            TagFirst( _, m )    |
            TagLast( _, m )     |
            Path( _, m )        |
            Image( _, m ) => {
                if let Some( m ) = m {
                    m.push_str( &format!( " {:+} {}", count, modifier ));
                } else {
                    *m = Some( format!( " {:+} {}", count, modifier ));
                }
                self
            },
        }
    }

    pub fn chars( self, count: c_int ) -> Self { self.counter_modifier( count, "chars" )}
    pub fn any_chars( self, count: c_int ) -> Self { self.counter_modifier( count, "any chars" )}
    pub fn display_chars( self, count: c_int ) -> Self { self.counter_modifier( count, "display chars" )}

    pub fn indices( self, count: c_int ) -> Self { self.counter_modifier( count, "indices" )}
    pub fn any_indices( self, count: c_int ) -> Self { self.counter_modifier( count, "any indices" )}
    pub fn display_indices( self, count: c_int ) -> Self { self.counter_modifier( count, "display indices" )}

    pub fn lines( self, count: c_int ) -> Self { self.counter_modifier( count, "lines" )}
    pub fn any_lines( self, count: c_int ) -> Self { self.counter_modifier( count, "any lines" )}
    pub fn display_lines( self, count: c_int ) -> Self { self.counter_modifier( count, "display lines" )}

    fn positional_modifier( mut self, modifier: &'static str ) -> Self {
        use Index::*;
        match &mut self {
            LineChar( _, _, m ) |
            LineEnd( _, m )     |
            At( _, _, m )       |
            End( m )            |
            Mark( _, m )        |
            TagFirst( _, m )    |
            TagLast( _, m )     |
            Path( _, m )        |
            Image( _, m ) => {
                if let Some( m ) = m {
                    m.push_str( modifier );
                } else {
                    *m = Some( modifier.to_owned() );
                }
                self
            },
        }
    }

    pub fn linestart( self ) -> Self { self.positional_modifier( " linestart" )}
    pub fn any_linestart( self ) -> Self { self.positional_modifier( " any linestart" )}
    pub fn display_linestart( self ) -> Self { self.positional_modifier( " display linestart" )}

    pub fn lineend( self ) -> Self { self.positional_modifier( " lineend" )}
    pub fn any_lineend( self ) -> Self { self.positional_modifier( " any lineend" )}
    pub fn display_lineend( self ) -> Self { self.positional_modifier( " display lineend" )}

    pub fn wordstart( self ) -> Self { self.positional_modifier( " wordstart" )}
    pub fn any_wordstart( self ) -> Self { self.positional_modifier( " any wordstart" )}
    pub fn display_wordstart( self ) -> Self { self.positional_modifier( " display wordstart" )}

    pub fn wordend( self ) -> Self { self.positional_modifier( " wordend" )}
    pub fn any_wordend( self ) -> Self { self.positional_modifier( " any wordend" )}
    pub fn display_wordend( self ) -> Self { self.positional_modifier( " display wordend" )}
}

pub enum TkCmp {
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equal,
    NotEqual,
}

impl From<TkCmp> for Obj {
    fn from( tk_cmp: TkCmp ) -> Obj {
        match tk_cmp {
            TkCmp::Less      => "<",
            TkCmp::LessEq    => "<=",
            TkCmp::Greater   => ">",
            TkCmp::GreaterEq => ">=",
            TkCmp::Equal     => "==",
            TkCmp::NotEqual  => "!=",
        }.into()
    }
}

bitflags! {
    pub struct TkTextDump: u32 {
        const IMAGE     = 0b00001;
        const MARK      = 0b00010;
        const TAG       = 0b00100;
        const TEXT      = 0b01000;
        const WINDOW    = 0b10000;
    }
}

impl Default for TkTextDump {
    fn default() -> TkTextDump {
        TkTextDump::all()
    }
}

impl From<TkTextDump> for Obj {
    fn from( dump: TkTextDump ) -> Obj {
        const TEXT_DUMP: [(TkTextDump, &'static str); 5] = [
            (TkTextDump::IMAGE , "-image"  ),
            (TkTextDump::MARK  , "-mark"   ),
            (TkTextDump::TAG   , "-tag"    ),
            (TkTextDump::TEXT  , "-text"   ),
            (TkTextDump::WINDOW, "-window" ),
        ];

        let mut list = Vec::new();
        TEXT_DUMP.iter().for_each( |(flag, switch)| if dump.contains( *flag ) {
            list.push( *switch );
        });
        list.into()
    }
}

#[derive( Debug )]
pub enum TkDump {
    Text{   value: String, index: Index },
    Mark{   value: String, index: Index },
    TagOn{  value: String, index: Index },
    TagOff{ value: String, index: Index },
    Image{  value: String, index: Index },
    Window{ value: String, index: Index },
}

impl TkDump {
    fn new( key: String, value: String, index: Index ) -> Option<Self> {
        match key.as_str() {
            "text"   => Some( TkDump::Text{   value, index }),
            "mark"   => Some( TkDump::Mark{   value, index }),
            "tagon"  => Some( TkDump::TagOn{  value, index }),
            "tagoff" => Some( TkDump::TagOff{ value, index }),
            "image"  => Some( TkDump::Image{  value, index }),
            "window" => Some( TkDump::Window{ value, index }),
            _        => None,
        }
    }
}

pub enum TkTextMarkGravity {
    Left,
    Right,
}

impl From<TkTextMarkGravity> for Obj {
    fn from( gravity: TkTextMarkGravity ) -> Obj {
        match gravity {
            TkTextMarkGravity::Left  => "left".into(),
            TkTextMarkGravity::Right => "right".into(),
        }
    }
}

bitflags! {
    pub struct TkTextSearch: u32 {
        const __            = 0b00000001;
        const BACKWARDS     = 0b00000010;
        const REGEXP        = 0b00000100;
        const NOLINESTOP    = 0b00001100;
        const NOCASE        = 0b00010000;
        const ELIDE         = 0b00100000;
        const STRICTLIMITS  = 0b01000000;
    }
}

bitflags! {
    pub struct TkTextSearchAll: u32 {
        const __            = 0b00000001;
        const BACKWARDS     = 0b00000010;
        const REGEXP        = 0b00000100;
        const NOLINESTOP    = 0b00001100;
        const NOCASE        = 0b00010000;
        const ELIDE         = 0b00100000;
        const STRICTLIMITS  = 0b01000000;
        const OEVERLAP      = 0b10000000;
    }
}

impl From<TkTextSearch> for Obj {
    fn from( search: TkTextSearch ) -> Obj {
        const TEXT_SEARCH: [(TkTextSearch, &'static str); 6] = [
            (TkTextSearch::BACKWARDS   , "-backwards"    ),
            (TkTextSearch::REGEXP      , "-regexp"       ),
            (TkTextSearch::NOLINESTOP  , "-nolinestop"   ),
            (TkTextSearch::NOCASE      , "-nocase"       ),
            (TkTextSearch::ELIDE       , "-elide"        ),
            (TkTextSearch::STRICTLIMITS, "-strictlimits" ),
        ];

        let mut list = Vec::new();
        TEXT_SEARCH.iter().for_each( |(flag, switch)| if search.contains( *flag ) {
            list.push( *switch );
        });
        list.into()
    }
}

impl TkTextSearch {
    fn check( self, pattern: impl AsRef<str> ) -> Self {
        if pattern.as_ref().starts_with("-") {
            self | TkTextSearch::__
        } else {
            self
        }
    }
}

impl From<TkTextSearchAll> for Obj {
    fn from( search: TkTextSearchAll ) -> Obj {
        const TEXT_SEARCH: [(TkTextSearchAll, &'static str); 7] = [
            (TkTextSearchAll::BACKWARDS   , "-backwards"    ),
            (TkTextSearchAll::REGEXP      , "-regexp"       ),
            (TkTextSearchAll::NOLINESTOP  , "-nolinestop"   ),
            (TkTextSearchAll::NOCASE      , "-nocase"       ),
            (TkTextSearchAll::ELIDE       , "-elide"        ),
            (TkTextSearchAll::STRICTLIMITS, "-strictlimits" ),
            (TkTextSearchAll::OEVERLAP    , "-overlap"      ),
        ];

        let mut list = vec![ "-all" ];
        TEXT_SEARCH.iter().for_each( |(flag, switch)| if search.contains( *flag ) {
            list.push( *switch );
        });
        list.into()
    }
}

impl TkTextSearchAll {
    fn check( self, pattern: impl AsRef<str> ) -> Self {
        if pattern.as_ref().starts_with("-") {
            self | TkTextSearchAll::__
        } else {
            self
        }
    }
}

impl<Inst:TkInstance> self::TkText<Inst> {
    pub fn compare( &self, index1: Index, op: TkCmp, index2: Index ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "compare", index1, op, index2 ))?;
        self.0.tk().boolean( obj )
    }

    fn count( &self, options: &'static str, index1: Index, index2: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "count", options, index1, index2 ))
    }

    pub fn count_chars( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "chars", index1, index2 )
    }

    pub fn count_displaychars( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "displaychars", index1, index2 )
    }

    pub fn count_displayindices( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "displayindices", index1, index2 )
    }

    pub fn count_displaylines( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "displaylines", index1, index2 )
    }

    pub fn count_indices( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "indices", index1, index2 )
    }

    pub fn count_lines( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "lines", index1, index2 )
    }

    pub fn count_xpixels( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "xpixels", index1, index2 )
    }

    pub fn count_ypixels( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.count( "ypixels", index1, index2 )
    }

    pub fn count_update_ypixels( &self, index1: Index, index2: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "count", "-update", "ypixels", index1, index2 ))
    }

    pub fn debug( &self ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "debug" ))?;
        self.0.tk().boolean( obj )
    }

    pub fn delete( &self, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "delete", index ))
    }

    pub fn delete_ranges( &self, ranges: Vec<Range<Index>> ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( ranges.len() * 2 + 2 );
        command.push( self.path.into() );
        command.push( "delete".into() );
        ranges.into_iter().for_each( |range| {
            command.push( range.start.into() );
            command.push( range.end.into() );
        });
        self.tk().run( command )
    }

    #[cex]
    pub fn dlineinfo( &self, index: Index ) -> Result!( Option<TkDLine> throws DeError, InterpError ) {
        let obj = self.0.tk().eval(( self.0.path, "dlineinfo", index ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            ret!( from_obj::<TkDLine>( obj ).map( Some ));
        }
    }

    #[cex]
    pub fn dump( &self, switches: TkTextDump, index: impl Into<Index> )
        -> Result!( Vec<TkDump> throws InterpError, NotList, TkDumpParseError, TkIndexParseError )
    {
        self.do_dump( self.tk().eval(( self.path, "dump", switches, index.into() ))? )
    }

    #[cex]
    pub fn dump_range( &self, switches: TkTextDump, range: impl Into<TkRange<Index>> )
        -> Result!( Vec<TkDump> throws InterpError, NotList, TkDumpParseError, TkIndexParseError )
    {
        let range = range.into();
        self.do_dump( self.tk().eval(( self.path, "dump", switches, range.start, range.end ))? )
    }

    #[cex]
    fn do_dump( &self, obj: Obj )
        -> Result!( Vec<TkDump> throws InterpError, NotList, TkDumpParseError, TkIndexParseError )
    {
        let output = obj
            .clone()
            .get_elements()?
            .map( |obj| obj.to_string() )
            .collect::<Vec<_>>();
        if output.len() % 3 != 0 {
            throw!( TkDumpParseError( obj ));
        }
        let mut dump = Vec::new();
        let mut key = None;
        let mut value = None;
        for (i, v) in output.into_iter().enumerate() {
            match i % 3 {
                0 => key   = Some(v),
                1 => value = Some(v),
                2 => {
                    let index = v.parse::<Index>()?;

                    //unwrap for previous 0,1 arms
                    if let Some( item ) = TkDump::new( key.take().unwrap(), value.take().unwrap(), index ) {
                        dump.push( item );
                    } else {
                        throw!( TkDumpParseError( obj ));
                    }
                },
                _ => unreachable!(),
            }
        }
        ret!( dump )
    }

    pub fn edit_canredo( &self ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "edit", "canredo" ))?;
        self.0.tk().boolean( obj )
    }

    pub fn edit_canundo( &self ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "edit", "canundo" ))?;
        self.0.tk().boolean( obj )
    }

    pub fn edit_modified( &self ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "edit", "modified" ))?;
        self.0.tk().boolean( obj )
    }

    pub fn set_edit_modified( &self, modified: bool ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "edit", "modified", modified ))
    }

    pub fn edit_redo( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "edit", "redo" ))
    }

    pub fn edit_reset( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "edit", "reset" ))
    }

    pub fn edit_separator( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "edit", "separator" ))
    }

    pub fn edit_undo( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "edit", "undo" ))
    }

    pub fn get( &self, index: impl Into<Index> ) -> InterpResult<String> {
        self.tk().eval(( self.path, "get", index.into() )).map( |obj| obj.to_string() )
    }

    pub fn get_range( &self, range: impl Into<TkRange<Index>> ) -> InterpResult<String> {
        let range = range.into();
        self.tk().eval(( self.path, "get", range.start, range.end )).map( |obj| obj.to_string() )
    }

    pub fn get_displaychars( &self, index: impl Into<Index> ) -> InterpResult<String> {
        self.tk().eval(( self.path, "get", "-displaychars", index.into() )).map( |obj| obj.to_string() )
    }

    pub fn get_displaychars_range( &self, range: impl Into<TkRange<Index>> ) -> InterpResult<String> {
        let range = range.into();
        self.tk().eval(( self.path, "get", "-displaychars", range.start, range.end )).map( |obj| obj.to_string() )
    }

    #[cex]
    pub fn get_ranges( &self, ranges: Vec<Range<Index>> ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let mut command = Vec::<Obj>::with_capacity( ranges.len() * 2 + 2 );
        command.push( self.path.into() );
        command.push( "get".into() );
        ranges.into_iter().for_each( |range| {
            command.push( range.start.into() );
            command.push( range.end.into() );
        });
        let obj = self.tk().eval( command )?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    #[cex]
    pub fn get_displaychars_ranges( &self, ranges: Vec<Range<Index>> ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let mut command = Vec::<Obj>::with_capacity( ranges.len() * 2 + 3 );
        command.push( self.path.into() );
        command.push( "get".into() );
        command.push( "-displaychars".into() );
        ranges.into_iter().for_each( |range| {
            command.push( range.start.into() );
            command.push( range.end.into() );
        });
        let obj = self.tk().eval( command )?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn image_cget<Opt>( &self, index: Index, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkEmbededImageOpt>
    {
        self.0.tk().eval(( self.0.path, "image", "cget", index, <Opt as TkOption>::NAME ))
    }

    pub fn image_configure<Opts>( &self, index: Index, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkEmbededImageOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "image".into() );
        command.push( "configure".into() );
        command.push( index.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn image_create<Opts>( &self, index: Index, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<String>
        where Opts: IntoHomoTuple<TkEmbededImageOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "image".into() );
        command.push( "create".into() );
        command.push( index.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().eval( command ).map( |obj| obj.to_string() )
    }

    #[cex]
    pub fn image_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.0.tk().eval(( self.0.path, "image", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    #[cex]
    pub fn index( &self, index: Index ) -> Result!( Index
        throws InterpError, TkIndexParseError )
    {
        let obj = self.0.tk().eval(( self.0.path, "index", index ))?;
        ret!( obj.to_string().parse::<Index>() );
    }

    pub fn insert( &self, index: Index, chars: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "insert", index, chars ))
    }

    pub fn insert_with_tags( &self, index: Index, chars_with_tags: &[(&str, &[&str])] )
        -> InterpResult<()>
    {
        let mut command = Vec::<Obj>::with_capacity( chars_with_tags.len() * 2 + 3 );
        command.push( self.path.into() );
        command.push( "insert".into() );
        command.push( index.into() );
        chars_with_tags.into_iter().for_each( |chars_tags| {
            command.push( chars_tags.0.into() );
            command.push( chars_tags.1.into() );
        });

        self.tk().run( command )
    }

    #[cex]
    pub fn mark_gravity( &self, name: &str ) -> Result!( TkTextMarkGravity
        throws InterpError, TkTextMarkGravityParseError )
    {
        let obj = self.0.tk().eval(( self.0.path, "mark", "gravity", name ))?;
        let s = obj.to_string();
        ret!( match s.as_str() {
            "left"  => TkTextMarkGravity::Left,
            "right" => TkTextMarkGravity::Right,
            _       => throw!( TkTextMarkGravityParseError( s )),
        })
    }

    pub fn set_mark_gravity( &self, name: &str, gravity: TkTextMarkGravity ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "mark", "gravity", name, gravity ))
    }

    #[cex]
    pub fn mark_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.0.tk().eval(( self.0.path, "mark", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn mark_next( &self, index: Index ) -> InterpResult<String> {
        self.0.tk().eval(( self.0.path, "mark", "next", index ))
            .map( |obj| obj.to_string() )
    }

    pub fn mark_previous( &self, index: Index ) -> InterpResult<String> {
        self.0.tk().eval(( self.0.path, "mark", "previous", index ))
            .map( |obj| obj.to_string() )
    }

    pub fn mark_set( &self, name: &str, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "mark", "set", name, index ))
    }

    pub fn mark_unset( &self, name: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "mark", "unset", name ))
    }

    pub fn peer_create<Opts>( &self, new_path_name: &str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkEmbededImageOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "peer".into() );
        command.push( "create".into() );
        command.push( new_path_name.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    #[cex]
    pub fn peer_names<Opts>( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.0.tk().eval(( self.0.path, "peer", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn pendingsync( &self ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "pendingsync" ))?;
        self.0.tk().boolean( obj )
    }

    pub fn replace( &self, index1: Index, index2: Index, chars: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "replace", index1, index2, chars ))
    }

    pub fn replace_with_tags( &self, index1: Index, index2: Index, chars_with_tags: &[(&str, &[&str])] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( chars_with_tags.len() * 2 + 4 );
        command.push( self.path.into() );
        command.push( "replace".into() );
        command.push( index1.into() );
        command.push( index2.into() );
        chars_with_tags.into_iter().for_each( |chars_tags| {
            command.push( chars_tags.0.into() );
            command.push( chars_tags.1.into() );
        });

        self.tk().run( command )
    }

    pub fn scan_mark( &self, x: c_int, y: c_int ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "mark", x, y ))
    }

    pub fn scan_dragto( &self, x: c_int, y: c_int ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "dragto", x, y ))
    }

    #[cex]
    pub fn search( &self, switches: TkTextSearch, pattern: &str, index: impl Into<Index> ) -> Result!( Index
        throws InterpError, TkIndexParseError )
    {
        let obj = self.tk().eval(( self.path, "search", switches.check( &pattern ), pattern, index.into() ))?;
        ret!( obj.to_string().parse::<Index>() );
    }

    #[cex]
    pub fn search_range( &self, switches: TkTextSearch, pattern: &str, range: impl Into<TkRange<Index>> )
        -> Result!( Index throws InterpError, TkIndexParseError )
    {
        let range = range.into();
        let obj = self.tk().eval(( self.path, "search", switches.check( &pattern ), pattern, range.start, range.end ))?;
        ret!( obj.to_string().parse::<Index>() );
    }

    #[cex]
    pub fn search_all( &self, switches: TkTextSearchAll, pattern: &str, index: impl Into<Index> )
        -> Result!( Vec<Index> throws InterpError, NotList, TkIndexParseError )
    {
        let obj = self.tk().eval(( self.path, "search", switches.check( &pattern ), pattern, index.into() ))?;
        let mut result = Vec::new();
        for elem in obj.get_elements()? {
            result.push( elem.to_string().parse::<Index>()? );
        }
        Ok( result )
    }

    #[cex]
    pub fn search_all_range( &self, switches: TkTextSearchAll, pattern: &str, range: impl Into<TkRange<Index>> )
        -> Result!( Vec<Index> throws InterpError, NotList, TkIndexParseError )
    {
        let range = range.into();
        let obj = self.tk().eval(( self.path, "search", switches.check( &pattern ), pattern, range.start, range.end ))?;
        let mut result = Vec::new();
        for elem in obj.get_elements()? {
            result.push( elem.to_string().parse::<Index>()? );
        }
        Ok( result )
    }

    pub fn see( &self, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "see", index ))
    }

    pub fn sync( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "sync" ))
    }

    pub fn sync_command( &self, command: impl Into<Obj> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "sync", "-command", command.into() ))
    }

    pub fn tag_add( &self, tag_name: &str, ranges: Vec<Range<Index>> ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( ranges.len() * 2 + 4 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "add".into() );
        command.push( tag_name.into() );
        ranges.into_iter().for_each( |range| {
            command.push( range.start.into() );
            command.push( range.end.into() );
        });
        self.tk().run( command )
    }

    pub fn tag_add_to_single_char( &self, tag_name: &str, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "tag", "add", tag_name, index ))
    }

    pub fn tag_bind( &self, tag_name: &str, sequence: impl Into<Obj>, script: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( self.path, "tag", "bind", tag_name, sequence.into(), script.into() ))
    }

    pub fn tag_cget<Opt>( &self, tag_name: &str, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkTextTagOpt>
    {
        self.0.tk().eval(( self.0.path, "tag", "cget", tag_name, <Opt as TkOption>::NAME ))
    }

    pub fn tag_configure<Opts>( &self, tag_name: &str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkTextTagOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "configure".into() );
        command.push( tag_name.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn tag_delete( &self, tag_names: &[&str] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( tag_names.len() + 3 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "delete".into() );

        tag_names.into_iter().for_each( |&tag_name| {
            command.push( tag_name.into() );
        });

        self.tk().run( command )
    }

    pub fn tag_lower( &self, tag_name: &str, below_this: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "tag", "lower", tag_name, below_this ))
    }

    pub fn tag_lower_below_all( &self, tag_name: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "tag", "lower", tag_name ))
    }

    #[cex]
    pub fn tag_names( &self, index: Index ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.0.tk().eval(( self.0.path, "tag", "names", index ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>()
        )
    }

    #[cex]
    pub fn tag_names_all( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.0.tk().eval(( self.0.path, "tag", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    #[cex]
    fn next_index( &self, index: &Index ) -> Result!( Index throws InterpError, TkIndexParseError ) {
        match self.index( index.clone() )? {
            Index::LineChar( line, ch, None ) => ret!( Index::LineChar( line, ch+1, None )),
            Index::LineEnd( line, None ) => ret!( Index::LineEnd( line, None )),
            _ => throw!( TkIndexParseError(
                format!( "index {:?} should be in the format of line(,char) without modifiers", index ))),
        }
    }

    #[cex]
    fn range_to_indice( &self, range: impl RangeBounds<Index> ) -> Result!( (Index, Index)
        throws InterpError, TkIndexParseError )
    {
        let index1 = match range.start_bound() {
            Bound::Included( bound ) => bound.clone(),
            Bound::Excluded( bound ) => self.next_index( bound )?, // unreachable
            Bound::Unbounded => Index::LineChar( 1, 0, None ),
        };

        let index2 = match range.start_bound() {
            Bound::Included( bound ) => self.next_index( bound )?,
            Bound::Excluded( bound ) => bound.clone(),
            Bound::Unbounded => Index::End( None ),
        };

        ret!(( index1, index2 ));
    }

    #[cex]
    pub fn tag_nextrange( &self, tag_name: &str, range: impl RangeBounds<Index> ) -> Result!( Option<Range<Index>>
        throws InterpError, NotList, TkIndexParseError )
    {
        let (index1, index2) = self.range_to_indice( range )?;
        let obj = self.0.tk().eval(( self.0.path, "tag", "nextrange", tag_name, index1, index2 ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            let mut iter = obj
                .get_elements()?
                .map( |obj| obj.to_string().parse::<Index>() );
            let first_line_char = iter.next().unwrap()?; // !obj.is_empty()
            let second_line_char = iter.next().unwrap_or( Err( TkIndexParseError::default() ))?;
            Ok( Some( first_line_char .. second_line_char ))
        }
    }

    #[cex]
    pub fn tag_prevrange( &self, tag_name: &str, range: impl RangeBounds<Index> ) -> Result!( Option<Range<Index>>
        throws InterpError, NotList, TkIndexParseError )
    {
        let (index1, index2) = self.range_to_indice( range )?;
        let obj = self.0.tk().eval(( self.0.path, "tag", "prevrange", tag_name, index1, index2 ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            let mut iter = obj
                .get_elements()?
                .map( |obj| obj.to_string().parse::<Index>() );
            let first_line_char = iter.next().unwrap()?; // !obj.is_empty()
            let second_line_char = iter.next().unwrap_or( Err( TkIndexParseError::default() ))?;
            Ok( Some( first_line_char .. second_line_char ))
        }
    }

    pub fn tag_raise( &self, tag_name: &str, above_this: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "tag", "raise", tag_name, above_this ))
    }

    pub fn tag_raise_above_all( &self, tag_name: &str ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "tag", "raise", tag_name ))
    }

    #[cex]
    pub fn tag_ranges( &self, tag_name: &str ) -> Result!( Vec<Range<Index>>
        throws InterpError, NotList, TkIndexParseError, TagRangesNotInPair )
    {
        let obj = self.0.tk().eval(( self.0.path, "tag", "ranges", tag_name ))?;
        let mut output = Vec::new();
        for elem in obj.clone().get_elements()? {
            output.push( elem.to_string().parse::<Index>()? );
        }
        if output.len() % 2 != 0 {
            throw!( TagRangesNotInPair( obj ));
        }
        let mut ranges = Vec::new();
        let mut start = None;
        for (i, v) in output.into_iter().enumerate() {
            match i % 2 {
                0 => start = Some(v),
                1 => ranges.push( start.take().unwrap() .. v ), // unwrap previous 0 arms
                _ => unreachable!(),
            }
        }
        Ok( ranges )
     }

     pub fn tag_remove( &self, tag_name: &str, ranges: Vec<Range<Index>> ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( ranges.len() * 2 + 4 );
        command.push( self.path.into() );
        command.push( "tag".into() );
        command.push( "remove".into() );
        command.push( tag_name.into() );
        ranges.into_iter().for_each( |range| {
            command.push( range.start.into() );
            command.push( range.end.into() );
        });
        self.tk().run( command )
    }

    pub fn tag_remove_at_single_char( &self, tag_name: &str, index: Index ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "tag", "remove", tag_name, index ))
    }

    pub fn window_cget<Opt>( &self, window_name: &str, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkEmbededWindowOpt>
    {
        self.0.tk().eval(( self.0.path, "window", "cget", window_name, <Opt as TkOption>::NAME ))
    }

    pub fn window_configure<Opts>( &self, window_name: &str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkEmbededWindowOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "window".into() );
        command.push( "configure".into() );
        command.push( window_name.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn window_create<Opts>( &self, index: Index, opts: impl Into<PathOptsWidgets<Opts,()>> )
        -> InterpResult<String>
        where Opts: IntoHomoTuple<TkEmbededWindowOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "window".into() );
        command.push( "create".into() );
        command.push( index.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().eval( command ).map( |obj| obj.to_string() )
    }

    #[cex]
    pub fn window_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.0.tk().eval(( self.0.path, "window", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn xview_scroll_pixels( &self, number: c_double ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "xview", "scroll", number, "pixels" ))
    }

    pub fn yview_scroll_pixels( &self, number: c_double ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "yview", "scroll", number, "pixels" ))
    }
}

impl<Inst:TkInstance> crate::TkBBoxTrait<Inst> for self::TkText<Inst> {
    type Index = Index;
}

impl<Inst:TkInstance> Delete<Inst> for TkText<Inst> {
    type Index = Index;
}

impl<TK:TkInstance> TkXView<TK> for TkText<TK> {}

impl<TK:TkInstance> TkXViewIndex<TK> for TkText<TK> {
    type Index = Index;
}

impl<TK:TkInstance> TkYView<TK> for TkText<TK> {}

impl<TK:TkInstance> TkYViewIndex<TK> for TkText<TK> {
    type Index = Index;
}
