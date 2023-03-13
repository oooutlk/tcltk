use crate::{
    interp::{
        Interp,
        Result,
    },
};

impl Interp {
    pub fn update( &self ) -> Result<()> {
        self.run( "update" )
    }

    pub fn update_idletasks( &self ) -> Result<()> {
        self.run(( "update", "idletasks" ))
    }
}
