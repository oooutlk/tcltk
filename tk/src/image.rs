use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::*;

use crate::{
    bitmap::Bitmap,
    error::NotList,
    photo::Photo,
};

use tuplex::*;

impl<Inst:TkInstance> Tk<Inst> {
    pub fn image_create_bitmap<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Bitmap<Inst>>
        where Opts: IntoHomoTuple<opt::TkBitmapOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "image"   .into() );
        command.push( "create"  .into() );
        command.push( "bitmap"  .into() );
        cmd::append_opts( &mut command, opts.into().opts );
        self.eval( command ).map( |name| Bitmap{ name, inst: self.inst })
    }

    pub fn image_create_photo<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Photo<Inst>>
        where Opts: IntoHomoTuple<opt::TkPhotoOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "image"   .into() );
        command.push( "create"  .into() );
        command.push( "photo"   .into() );
        cmd::append_opts( &mut command, opts.into().opts );
        self.eval( command ).map( |name| Photo{ name, inst: self.inst })
    }

    pub fn image_delete( &self, image: Obj ) -> InterpResult<()> {
        self.run(( "image", "delete", image ))
    }

    #[cex]
    pub fn image_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.eval(( "image", "names" ))?;
        Ok( obj .get_elements()?
                .map( |elem| elem.to_string() )
                .collect() )
    }

    #[cex]
    pub fn image_types( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.eval(( "image", "types" ))?;
        Ok( obj .get_elements()?
                .map( |elem| elem.to_string() )
                .collect() )
    }
}

pub trait Image<Inst:TkInstance> {
    fn tk( &self ) -> Tk<Inst>;
    fn name( &self ) -> Obj;

    fn height( &self ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( "image", "height", self.name() ))?;
        self.tk().int( obj )
    }

    fn inuse( &self ) -> InterpResult<bool> {
        let obj = self.tk().eval(( "image", "inuse" ))?;
        self.tk().boolean( obj )
    }

    fn type_( &self ) -> InterpResult<String> {
        self.tk().eval(( "image", "type", self.name() ))
            .map( |obj| obj.to_string() )
    }

    fn width( &self ) -> InterpResult<c_int> {
        let obj = self.tk().eval(( "image", "width", self.name() ))?;
        self.tk().int( obj )
    }
}
