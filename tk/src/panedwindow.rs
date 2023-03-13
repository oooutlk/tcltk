use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    PathOptsWidgets,
    TkCoord,
    TkInstance,
    TkOption,
    Widget,
    error::{
        UnexpectedPanedwindowIdentifyResult,
        WidgetNotFound,
    },
    opt::{
        TkPaneConfigureOpt,
        TkPanedwindowOpt,
        OptPair,
    },
};

use std::os::raw::{
    c_int,
    c_longlong,
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
pub struct TkPanedwindow<Inst:TkInstance>( pub(crate) Widget<Inst> );

pub enum TkSashOrHandle {
    Sash,
    Handle,
}

impl<Inst:TkInstance> TkPanedwindow<Inst> {
    pub fn add<Opts>( &self, widgs: &[ &Widget<Inst> ], opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<TkPanedwindowOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity(
              widgs.len()
            + <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2
            + 2
        );
        command.push( self.path.into() );
        command.push( "add".into() );
        command.extend( widgs.iter().map( |widget| widget.path.into() ));
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn forget( &self, widgs: &[ &Widget<Inst> ] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( widgs.len() + 2 );
        command.push( self.path.into() );
        command.push( "forget".into() );
        command.extend( widgs.iter().map( |widget| Obj::from( widget.path )));
        self.tk().run( command )
    }

    #[cex]
    pub fn identify( &self, x: c_int, y: c_int ) -> Result!( Option<(c_longlong, TkSashOrHandle)>
        throws DeError, InterpError, UnexpectedPanedwindowIdentifyResult )
    {
        let obj = self.tk().eval(( self.path, "identify", x, y ))?;
        let result = from_obj::<(c_longlong, String)>( obj )?;
        ret!( match result.1.as_str() {
            "sash"   => Some(( result.0, TkSashOrHandle::Sash   )),
            "handle" => Some(( result.0, TkSashOrHandle::Handle )),
            _ => throw!( UnexpectedPanedwindowIdentifyResult( result.1 )),
        })
    }

    pub fn panecget<Opt,Val>( &self, index: c_int, _name_fn: fn(Val)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkPaneConfigureOpt>
            , Val : Into<Obj>
    {
        self.0.tk().eval(( self.0.path, "panecget", index, <Opt as TkOption>::NAME ))
    }

    pub fn paneconfigure<Opts>( &self, index: c_int, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkPanedwindowOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "paneconfigure".into() );
        command.push( Obj::from( index as i32 ));
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    #[cex]
    pub fn panes( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let children = self.tk().eval(( self.path, "panes" ))?;
        ret!( self.tk().widgets_from_obj( children )? );
    }

    #[cex]
    pub fn proxy_coord( &self ) -> Result!( TkCoord throws DeError, InterpError, NotList ) {
        let obj = self.tk().eval(( self.path, "proxy", "coord" ))?;
        ret!( from_obj::<TkCoord>( obj ))
    }

    pub fn proxy_forget( &self ) -> InterpResult<()> {
        self.tk().run(( self.path, "proxy", "forget" ))
    }

    pub fn proxy_place( &self, x: c_int, y: c_int ) -> InterpResult<()> {
        self.tk().run(( self.path, "proxy", "place", x, y ))
    }

    #[cex]
    pub fn sash_coord( &self, index: c_int ) -> Result!( TkCoord throws DeError, InterpError ) {
        let obj = self.tk().eval(( self.path, "sash", "coord", index ))?;
        ret!( from_obj::<TkCoord>( obj ));
    }

    pub fn sash_dragto( &self, index: c_int, x: c_int, y: c_int ) -> InterpResult<()> {
        self.tk().run(( self.path, "sash", "dragto", index, x, y ))
    }

    pub fn sash_mark( &self, index: c_int, x: c_int, y: c_int ) -> InterpResult<()> {
        self.tk().run(( self.path, "sash", "mark", index, x, y ))
    }

    pub fn sash_place( &self, index: c_int, x: c_int, y: c_int ) -> InterpResult<()> {
        self.tk().run(( self.path, "sash", "place", index, x, y ))
    }
}
