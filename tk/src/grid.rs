use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    TkBBox,
    TkInstance,
    OptPair,
    PathOptsWidgets,
    InterpResult,
    Widget,
    cmd::append_opts,
    opt,
};

use std::os::raw::c_int;

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
    },
    from_obj,
};

use tuplex::IntoHomoTuple;

#[derive( Debug )]
pub struct Grid {
    pub column : c_int,
    pub row    : c_int,
}

#[derive( Debug )]
pub enum Grids {
    Unspecified,
    Single{    column: c_int, row: c_int, },
    Rectangle{ column: c_int, row: c_int, column2: c_int, row2: c_int },
}

impl From<Grids> for Obj {
    fn from( grids: Grids ) -> Obj {
        match grids {
            Grids::Unspecified => Obj::new(),
            Grids::Single{ column, row } => (column, row).into(),
            Grids::Rectangle{ column, row, column2, row2 } => (column, row, column2, row2).into(),
        }
    }
}

#[derive( Debug )]
pub struct Size {
    pub column_count : c_int,
    pub row_count    : c_int,
}

impl<Inst:TkInstance> Widget<Inst> {
    pub fn grid_anchor( &self, anchor: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "grid", "anchor", self.path, anchor.into() ))
    }

    #[cex]
    pub fn grid_bbox( &self, grids: Grids ) -> Result!( TkBBox throws DeError, InterpError ) {
        let obj = self.tk().eval(( "eval", "grid", "bbox", self.path, grids ))?;
        let v = from_obj::<[c_int; 4]>( obj )?;
        ret!( TkBBox{ x: v[0], y: v[1], w: v[2], h: v[3] });
    }

    pub fn grid_columnconfigure<Opts>( &self, index: impl Into<Obj>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkGridColumnConfigureOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( "grid".into() );
        command.push( "columnconfigure".into() );
        command.push( self.path.into() );
        command.push( index.into() );

        append_opts( &mut command, opts.into().opts );

        self.tk().run( command )
    }

    #[cex]
    pub fn grid_location( &self, x: c_int, y: c_int ) -> Result!( Grid throws DeError, InterpError ) {
        let obj = self.tk().eval(( "grid", "location", self.path, x, y ))?;
        let (column, row) = from_obj::<(c_int, c_int)>( obj )?;
        Ok( Grid{ column, row })
    }

    pub fn grid_propagate( &self, do_propagate: bool ) -> InterpResult<()> {
        self.tk().run(( "grid", "propagate", self.path, do_propagate ))
    }

    pub fn grid_propagated( &self ) -> InterpResult<bool> {
        let obj = self.tk().eval(( "grid", "propagate", self.path ))?;
        self.tk().boolean( obj )
    }

    pub fn grid_rowconfigure<Opts>( &self, index: impl Into<Obj>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkGridRowConfigureOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( "grid".into() );
        command.push( "rowconfigure".into() );
        command.push( self.path.into() );
        command.push( index.into() );

        append_opts( &mut command, opts.into().opts );

        self.tk().run( command )
    }

    pub fn grid_remove( &self ) -> InterpResult<()> {
        self.tk().eval(( "grid", "remove", self.path )).map( |_| () )
    }

    #[cex]
    pub fn grid_size( &self ) -> Result!( Size throws DeError, InterpError ) {
        let obj = self.tk().eval(( "grid", "size", self.path ))?;
        let (column_count, row_count) = from_obj::<(c_int, c_int)>( obj )?;
        Ok( Size{ column_count, row_count })
    }
}
