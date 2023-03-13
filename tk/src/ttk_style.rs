use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    OptPair,
    PathOptsWidgets,
    Tk,
    TkInstance,
    TkOption,
    TtkState,
    TtkStateSpec,
    error::TtkStateParseError,
    opt::TkThemeCreateOpt,
};

use std::{
    str::FromStr,
};

use tuplex::*;

use tcl::{
    Obj,
    error::{
        InterpError,
        NotList,
    },
};

pub struct TtkStyle<TK:TkInstance> {
    name : String,
    inst : TK,
}

impl<TK:TkInstance> Tk<TK> {
    pub fn new_ttk_style( &self, name: &str, base_style: Option<TtkStyle<TK>> ) -> TtkStyle<TK> {
        let name = match base_style {
            Some( base_style ) => format!( "{}.{}", base_style.name, name ),
            None               => name.to_owned(),
        };
        TtkStyle{ name, inst: self.inst }
    }

    pub fn element( &self, name: &str ) -> TtkElement<TK> {
        TtkElement{ name: name.to_owned(), inst: self.inst }
    }

    pub fn element_create( &self, element_name: &str, type_: &str, args: Option<Obj> ) -> InterpResult<TtkElement<TK>> {
        self.run(( "eval", "ttk::style", "element", "create", (element_name,), (type_,), args ))?;
        Ok( TtkElement{ name: element_name.to_owned(), inst: self.inst })
    }

    #[cex]
    pub fn element_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.eval(( "ttk::style", "element", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect() )
    }

    pub fn theme_create<Opts>( &self, name_parent_settings: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<TtkTheme<TK>>
        where Opts : IntoHomoTuple<TkThemeCreateOpt>
                   + IntoHomoTuple<OptPair>
    {
        let name_parent_settings = name_parent_settings.into();
        let name = name_parent_settings.path;
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( "ttk::style".into() );
        command.push( "theme".into() );
        command.push( "create".into() );
        command.push( name.into() );
        crate::cmd::append_opts( &mut command, name_parent_settings.opts );
        self.run( command )?;
        Ok( TtkTheme{ name: name.to_owned(), inst: self.inst })
    }

    #[cex]
    pub fn theme_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.eval(( "ttk::style", "theme", "names" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect() )
    }

    pub fn theme_in_use( &self ) -> InterpResult<TtkTheme<TK>> {
        self.eval(( "ttk::style", "theme", "use" ))
            .map( |obj| TtkTheme{ name: obj.to_string(), inst: self.inst })
    }
}

impl<TK:TkInstance> TtkStyle<TK> {
    pub(crate) fn tk( &self ) -> Tk<TK> { Tk::from_inst( self.inst )}

    pub fn configure<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "ttk::style".into() );
        command.push( "configure".into() );
        command.push( self.name.as_str().into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    // faked cget
    pub fn cget<Opt:TkOption>( &self, _opt: fn(Obj)->Opt ) -> InterpResult<Obj>
    {
        self.tk().eval(( "ttk::style", "configure", self.name.as_str(), <Opt as TkOption>::NAME ))
    }

    pub fn map<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "ttk::style".into() );
        command.push( "map".into() );
        command.push( self.name.as_str().into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    #[cex]
    pub fn mapped( &self ) -> Result!( Vec<(TtkState,Obj)> throws InterpError, NotList, TtkStateParseError ) {
        let obj = self.tk().eval(( "ttk::style", "map", self.name.as_str() ))?;
        let list = obj.get_elements()?;

        let mut v = Vec::with_capacity( list.size_hint().0 );
        let mut state_enum = None;
        for obj in list {
            match state_enum {
                Some( state_enum ) => v.push(( state_enum, obj )),
                None => state_enum = Some( TtkState::from_str( obj.to_string().as_str() )? ),
            }
        }
        ret!( v );
    }

    pub fn lookup_normal<Opt:TkOption>( &self, _opt: fn(Obj)->Opt )
        -> InterpResult<Obj>
    {
        self.tk().eval(( "ttk::style", "lookup", self.name.as_str(), <Opt as TkOption>::NAME ))
    }

    pub fn lookup<Opt:TkOption>( &self, _opt: fn(Obj)->Opt, state_spec: impl Into<TtkStateSpec> )
        -> InterpResult<Obj>
    {
        self.tk().eval(( "ttk::style", "lookup", self.name.as_str(), <Opt as TkOption>::NAME, state_spec.into() ))
    }

    pub fn lookup_or<Opt:TkOption>( &self, _opt: fn(Obj)->Opt, state_spec: impl Into<TtkStateSpec>, default: Obj )
        -> InterpResult<Obj>
    {
        self.tk().eval(( "ttk::style", "lookup", self.name.as_str(), <Opt as TkOption>::NAME, state_spec.into(), default ))
    }

    pub fn set_layout( &self, layout_spec: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "ttk::style", "layout", self.name.as_str(), layout_spec ))
    }

    pub fn layout( &self ) -> InterpResult<Obj> {
        self.tk().eval(( "ttk::style", "layout", self.name.as_str() ))
    }
}

impl<TK:TkInstance> From<&TtkStyle<TK>> for Obj {
    fn from( style: &TtkStyle<TK> ) -> Obj {
        Obj::from( style.name.as_str() )
    }
}

pub struct TtkElement<TK:TkInstance> {
    name : String,
    inst : TK,
}

impl<TK:TkInstance> TtkElement<TK> {
    pub(crate) fn tk( &self ) -> Tk<TK> { Tk::from_inst( self.inst )}

    #[cex]
    pub fn element_options( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.tk().eval(( "ttk::style", "element", "options", self.name.as_str() ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect() )
    }
}

pub struct TtkTheme<TK:TkInstance> {
    pub name : String,
        inst : TK,
}

impl<TK:TkInstance> TtkTheme<TK> {
    pub(crate) fn tk( &self ) -> Tk<TK> { Tk::from_inst( self.inst )}

    pub fn theme_settings( &self, script: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "ttk::style", "theme", "settings", self.name.as_str(), script ))
    }

    pub fn theme_use( &self ) -> InterpResult<()> {
        self.tk().run(( "ttk::style", "theme", "use", self.name.as_str() ))
    }
}
