use crate::*;

use tcl::Obj;

use tuplex::*;

pub struct Bitmap<Inst:TkInstance> {
    pub(crate) name : Obj,
    pub(crate) inst : Inst,
}

impl<Inst:TkInstance> From<Bitmap<Inst>> for Obj {
    fn from( bitmap: Bitmap<Inst> ) -> Obj {
        bitmap.name
    }
}

impl<Inst:TkInstance> Bitmap<Inst> {
    pub fn from_name( inst: Inst, name: &str ) -> Option<Self> {
        let tk = Tk::from_inst( inst );
        if tk.eval(( "image", "type", name ))
            .map( |obj| obj.to_string() == "bitmap" )
            .unwrap_or( false )
        {
            Some( Bitmap{ name: name.into(), inst })
        } else {
            None
        }
    }

    pub fn cget<Opt>( &self, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<opt::TkBitmapOpt>
    {
        self.tk().eval(( self.name.clone(), "cget", <Opt as TkOption>::NAME ))
    }

    pub fn configure<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<opt::TkBitmapOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );
        command.push( self.name.clone() );
        command.push( "configure".into() );

        cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }
}

impl<Inst:TkInstance> Image<Inst> for Bitmap<Inst> {
    fn tk( &self ) -> Tk<Inst> { Tk::from_inst( self.inst )}

    fn name( &self ) -> Obj { self.name.clone() }
}
