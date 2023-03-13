use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    PathOptsWidgets,
    TkInstance,
    TkOption,
    TtkCommonTraits,
    TtkInsertPos,
    Widget,
    error::WidgetNotFound,
    opt::{
        OptPair,
        TtkPaneOpt,
    },
};

use std::os::raw::c_int;

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
        NotList,
    },
};

use tuplex::*;

#[derive( Copy, Clone )]
pub struct TtkPanedwindow<TK:TkInstance>( pub(crate) Widget<TK> );

pub enum TtkPane<'w, TK:TkInstance> {
    Num( c_int ),
    Widget( &'w Widget<TK> ),
}

impl<'w, TK:TkInstance> From<TtkPane<'w,TK>> for Obj {
    fn from( pane: TtkPane<'w,TK> ) -> Obj {
        match pane {
            TtkPane::Num(n) => n.into(),
            TtkPane::Widget(w) => w.path.into(),
        }
    }
}

impl<TK:TkInstance> TtkPanedwindow<TK> {
    pub fn add<Opts>( &self, widget: &Widget<TK>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<TtkPaneOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "add".into() );
        command.push( widget.path.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn forget( &self, pane: TtkPane<TK> ) -> InterpResult<()> {
        self.tk().run(( self.path, "forget", pane ))
    }

    pub fn identify_sash( &self, x: c_int, y: c_int ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "identify", x, y ))?;
        self.0.tk().int( obj )
    }

    pub fn insert<Opts>( &self, pos: TtkInsertPos, widget: &Widget<TK>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TtkPaneOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( self.path.into() );
        command.push( "insert".into() );
        command.push( pos.into() );
        command.push( widget.path.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn set_pane<Opts>( &self, pane: TtkPane<TK>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TtkPaneOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "pane".into() );
        command.push( pane.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn pane<Opt,Val>( &self, pane: TtkPane<TK>, _name_fn: fn(Val)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TtkPaneOpt>
            , Val : Into<Obj>
    {
        self.0.tk().eval(( self.0.path, "pane", pane, <Opt as TkOption>::NAME ))
    }

    #[cex]
    pub fn panes( &self ) -> Result!( Vec<Widget<TK>> throws DeError, InterpError, NotList, WidgetNotFound ) {
        let children = self.tk().eval(( self.path, "panes" ))?;
        ret!( self.tk().widgets_from_obj( children )? );
    }

    pub fn set_sashpos( &self, index: c_int, newpos: c_int ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "sashpos", index, newpos ))?;
        self.tk().int( obj )
    }

    pub fn sashpos( &self, index: c_int ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "sashpos", index ))?;
        self.tk().int( obj )
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkPanedwindow<TK> {}
