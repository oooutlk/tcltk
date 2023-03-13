use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    Tk,
    TkInstance,
    Widget,
    TkToplevel,
    event::TkEventSeq,
};

use tcl::{
    Obj,
    error::{
        InterpError,
        NotList,
    },
};

pub enum TkBindTag<'a, Inst:TkInstance> {
    Internal( &'a Widget<Inst> ),
    Toplevel( &'a TkToplevel<Inst> ),
    Class( &'a str ),
    All,
}

impl<'a, Inst:TkInstance> From<TkBindTag<'a, Inst>> for Obj {
    fn from( tag: TkBindTag<'a, Inst> ) -> Obj {
        use TkBindTag::*;
        match tag {
            Internal(   widget ) => widget.path.into(),
            Toplevel( toplevel ) => toplevel.path.into(),
            Class(        name ) => name.into(),
            All                  => "all".into(),
        }
    }
}

impl<Inst:TkInstance> Tk<Inst> {
    pub fn bind( &self, tag: TkBindTag<Inst>, sequence: impl Into<TkEventSeq>, script: impl Into<Obj> ) -> InterpResult<()> {
        self.run(( "bind", tag, sequence.into(), script ))
    }

    pub fn bind_more( &self, tag: TkBindTag<Inst>, sequence: impl Into<TkEventSeq>, script: impl Into<Obj> ) -> InterpResult<()> {
        let script = format!( "+{}", script.into().to_string() );
        self.run(( "bind", tag, sequence.into(), script ))
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    pub fn bind( &self, sequence: impl Into<TkEventSeq>, script: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "bind", self.path, sequence.into(), script ))
    }

    pub fn bind_more( &self, sequence: impl Into<TkEventSeq>, script: impl Into<Obj> ) -> InterpResult<()> {
        let script = format!( "+{}", script.into().to_string() );
        self.tk().run(( "bind", self.path, sequence.into(), script ))
    }

    pub fn bindtags( &self, tags: &[&str] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( tags.len() + 2 );

        command.push( self.path.into() );
        command.push( "bindtags".into() );

        for &tag in tags {
            command.push( tag.into() );
        }

        self.tk().run( command )
    }

    #[cex]
    pub fn bindedtags( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.tk().eval(( self.path, "bindtags" ))?;
        ret!( obj
            .get_elements()?
            .map( |elem| elem.to_string() )
            .collect::<Vec<_>>() );
    }
}
