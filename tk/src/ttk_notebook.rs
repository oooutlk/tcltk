use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    PathOptsWidgets,
    TkInstance,
    TtkCommonTraits,
    TtkInsertPos,
    Widget,
    error::WidgetNotFound,
    opt::{
        OptPair,
        TtkNotebookTabOpt,
    },
};

use std::os::raw::c_int;

use tcl::{
    Obj,
    error::{
        InterpError,
        NotList,
    },
};

use tuplex::*;

#[derive( Copy, Clone )]
pub struct TtkNotebook<TK:TkInstance>( pub(crate) Widget<TK> );

pub enum TtkNotebookTabId<'w,TK:TkInstance> {
    Num( c_int ),
    Widget( &'w Widget<TK> ),
    At{ x: c_int, y: c_int },
    Current,
}

impl<'w,TK:TkInstance> From<TtkNotebookTabId<'w,TK>> for Obj {
    fn from( id: TtkNotebookTabId<'w,TK> ) -> Obj {
        match id {
            TtkNotebookTabId::Num(n)     => n.into(),
            TtkNotebookTabId::Widget(w)  => w.path.into(),
            TtkNotebookTabId::At{ x, y } => format!( "@{}{}", x, y ).into(),
            TtkNotebookTabId::Current    => "current".into(),
        }
    }
}

impl<TK:TkInstance> TtkNotebook<TK> {
    pub fn add<Opts>( &self, widget: &Widget<TK>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TtkNotebookTabOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "add".into() );
        command.push( widget.path.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn forget( &self, tab_id: TtkNotebookTabId<TK> ) -> InterpResult<()> {
        self.tk().run(( self.path, "forget", tab_id ))
    }

    pub fn hide( &self, tab_id: TtkNotebookTabId<TK> ) -> InterpResult<()> {
        self.tk().run(( self.path, "hide", tab_id ))
    }

    pub fn identify_tab( &self, x: c_int, y: c_int ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "identify", "tab", x, y ))?;
        self.tk().int( obj )
    }

    pub fn index( &self, tab_id: TtkNotebookTabId<TK> ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "index", tab_id ))?;
        self.tk().int( obj )
    }

    pub fn insert<Opts>( &self, pos: TtkInsertPos, widget: &Widget<TK>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TtkNotebookTabOpt>
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

    pub fn num_of_tabs( &self ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( self.path, "index", "end" ))?;
        self.tk().int( obj )
    }

    pub fn select( &self, tab_id: TtkNotebookTabId<TK> ) -> InterpResult<()> {
        self.tk().run(( self.path, "select", tab_id ))
    }

    pub fn tab<Opts>( &self, tab_id: TtkNotebookTabId<TK>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TtkNotebookTabOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "tab".into() );
        command.push( tab_id.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    #[cex]
    pub fn tabs( &self ) -> Result!( Vec<Widget<TK>> throws InterpError, NotList, WidgetNotFound ) {
        let windows = self.tk().eval(( self.path, "tabs" ))?;
        ret!( self.tk().widgets_from_obj( windows ))
    }
}

impl<TK:TkInstance> TtkCommonTraits<TK> for TtkNotebook<TK> {}
