use crate::*;

use crate::{
    error::{
        DeError,
        InterpError,
    },
    types::TkRGB,
};

use std::{
    path::Path,
};

use tcl::{
    Obj,
    from_obj,
};

use tuplex::*;

pub struct Photo<Inst:TkInstance> {
    pub(crate) name : Obj,
    pub(crate) inst : Inst,
}

impl<Inst:TkInstance> From<Photo<Inst>> for Obj {
    fn from( photo: Photo<Inst> ) -> Obj {
        photo.name
    }
}

impl<Inst:TkInstance> Photo<Inst> {
    pub fn from_name( inst: Inst, name: &str ) -> Option<Self> {
        let tk = Tk::from_inst( inst );
        if tk.eval(( "image", "type", name ))
            .map( |obj| obj.to_string() == "photo" )
            .unwrap_or( false )
        {
            Some( Photo{ name: name.into(), inst })
        } else {
            None
        }
    }

    pub fn blank( &self ) -> InterpResult<()> {
        self.tk().run(( self.name.clone(), "blank" ))
    }

    pub fn cget<Opt>( &self, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<opt::TkPhotoOpt>
    {
        self.tk().eval(( self.name.clone(), "cget", <Opt as TkOption>::NAME ))
    }

    pub fn configure<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkPhotoOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );
        command.push( self.name.clone() );
        command.push( "configure".into() );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn copy<Opts>( &self, source_image: &Photo<Inst>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkPhotoCopyOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( 20 );
        command.push( "eval".into() );
        command.push( self.name.clone() );
        command.push( "copy".into() );
        command.push( source_image.name.clone() );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn data<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts: IntoHomoTuple<opt::TkPhotoDataOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( 11 );
        command.push( "eval".into() );
        command.push( self.name.clone() );
        command.push( "data".into() );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().eval( command )
    }

    #[cex]
    pub fn get( &self, x: c_int, y: c_int ) -> Result!( TkRGB
        throws DeError, InterpError )
    {
        let obj = self.tk().eval(( self.name.clone(), "get", x, y ))?;
        let ( red, green, blue ) = from_obj::<(u8,u8,u8)>( obj )?;
        Ok( TkRGB( red as u16, green as u16, blue as u16 ))
    }

    pub fn put<Opts>( &self, data: Obj, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkPhotoPutOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( 8 );
        command.push( "eval".into() );
        command.push( self.name.clone() );
        command.push( "put".into() );
        command.push( data );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn read<Opts>( &self, filename: impl AsRef<Path>, opts: impl Into<PathOptsWidgets<Opts,()>> ) ->InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkPhotoReadOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( 12 );
        command.push( "eval".into() );
        command.push( self.name.clone() );
        command.push( "read".into() );
        command.push( filename.as_ref().to_string_lossy().into() );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn redither( &self ) -> InterpResult<()> {
        self.tk().run(( self.name.clone(), "redither" ))
    }

    pub fn transparency_get( &self, x: c_int, y: c_int ) -> InterpResult<bool> {
        let obj = self.tk().eval(( self.name.clone(), "transparency", "get", x, y ))?;
        self.tk().boolean( obj )
    }

    pub fn transparency_set( &self, x: c_int, y: c_int, transparent: bool ) -> InterpResult<()> {
        self.tk().run(( self.name.clone(), "transparency", "set", x, y, transparent ))
    }

    pub fn write<Opts>( &self, filename: impl AsRef<Path>, opts: impl Into<PathOptsWidgets<Opts,()>> ) ->InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkPhotoWriteOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( 11 );
        command.push( "eval".into() );
        command.push( self.name.clone() );
        command.push( "write".into() );
        command.push( filename.as_ref().to_string_lossy().into() );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }
}

impl<Inst:TkInstance> Image<Inst> for Photo<Inst> {
    fn tk( &self ) -> Tk<Inst> { Tk::from_inst( self.inst )}

    fn name( &self ) -> Obj { self.name.clone() }
}
