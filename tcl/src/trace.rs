use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    Obj,
    error::{
        DeError,
        DeKind,
        InterpError,
    },
    interp::*,
};

use structx::*;

impl Interp {
    pub fn trace_add_command_rename( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "command", name, "rename", command ))
    }

    pub fn trace_add_command_delete( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "command", name, "delete", command ))
    }

    pub fn trace_add_execution_enter( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "execution", name, "enter", command ))
    }

    pub fn trace_add_execution_leave( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "execution", name, "leave", command ))
    }

    pub fn trace_add_execution_enter_step( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "execution", name, "enterstep", command ))
    }

    pub fn trace_add_execution_leave_step( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "execution", name, "leavestep", command ))
    }

    pub fn trace_add_variable_array( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "variable", name, "array", command ))
    }

    pub fn trace_add_variable_read( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "variable", name, "read", command ))
    }

    pub fn trace_add_variable_write( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "variable", name, "write", command ))
    }

    pub fn trace_add_variable_unset( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "add", "variable", name, "unset", command ))
    }

    pub fn trace_remove_command_rename( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "command", name, "rename", command ))
    }

    pub fn trace_remove_command_delete( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "command", name, "delete", command ))
    }

    pub fn trace_remove_execution_enter( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "execution", name, "enter", command ))
    }

    pub fn trace_remove_execution_leave( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "execution", name, "leave", command ))
    }

    pub fn trace_remove_execution_enter_step( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "execution", name, "enterstep", command ))
    }

    pub fn trace_remove_execution_leave_step( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "execution", name, "leavestep", command ))
    }

    pub fn trace_remove_variable_array( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "variable", name, "array", command ))
    }

    pub fn trace_remove_variable_read( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "variable", name, "read", command ))
    }

    pub fn trace_remove_variable_write( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "variable", name, "write", command ))
    }

    pub fn trace_remove_variable_unset( &self, name: impl Into<Obj>, command: impl Into<Obj> ) -> Result<()> {
        self.run(( "trace", "remove", "variable", name, "unset", command ))
    }

    #[cex]
    pub fn trace_info_command( &self, name: impl Into<Obj> ) -> Result!( Vec<Structx!{ op: Obj, command: Obj }>
        throws DeError, InterpError )
    {
        let elems = self.eval(( "trace", "info", "command", name ))?
            .get_elements()
            .map_err( |err| DeError::new( DeKind::NotList, err.0 ))?;

        let mut info = vec![];
        for elem in elems {
            let e = elem.clone();
            let mut op_and_command = elem.get_elements()
                .map_err( |err| DeError::new( DeKind::NotList, err.0 ))?;
            let op = match op_and_command.next() {
                Some( op ) => op,
                None => throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 0 }, e )),
            };
            let command = match op_and_command.next() {
                Some( command ) => command,
                None => throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 1 }, e )),
            };

            let remaining = op_and_command.count();
            if remaining != 0 {
                throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 2+remaining }, e ));
            }
            info.push( structx!{ op, command });
        }

        Ok( info )
    }

    #[cex]
    pub fn trace_info_execution( &self, name: impl Into<Obj> ) -> Result!( Vec<Structx!{ op: Obj, command: Obj }>
        throws DeError, InterpError )
    {
        let elems = self.eval(( "trace", "info", "execution", name ))?
            .get_elements()
            .map_err( |err| DeError::new( DeKind::NotList, err.0 ))?;

        let mut info = vec![];
        for elem in elems {
            let e = elem.clone();
            let mut op_and_command = elem.get_elements()
                .map_err( |err| DeError::new( DeKind::NotList, err.0 ))?;
            let op = match op_and_command.next() {
                Some( op ) => op,
                None => throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 0 }, e )),
            };
            let command = match op_and_command.next() {
                Some( command ) => command,
                None => throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 1 }, e )),
            };

            let remaining = op_and_command.count();
            if remaining != 0 {
                throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 2+remaining }, e ));
            }
            info.push( structx!{ op, command });
        }

        Ok( info )
    }

    #[cex]
    pub fn trace_info_variable( &self, name: impl Into<Obj> ) -> Result!( Vec<Structx!{ op: Obj, command: Obj }>
        throws DeError, InterpError )
    {
        let elems = self.eval(( "trace", "info", "variable", name ))?
            .get_elements()
            .map_err( |err| DeError::new( DeKind::NotList, err.0 ))?;

        let mut info = vec![];
        for elem in elems {
            let e = elem.clone();
            let mut op_and_command = elem.get_elements()
                .map_err( |err| DeError::new( DeKind::NotList, err.0 ))?;
            let op = match op_and_command.next() {
                Some( op ) => op,
                None => throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 0 }, e )),
            };
            let command = match op_and_command.next() {
                Some( command ) => command,
                None => throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 1 }, e )),
            };

            let remaining = op_and_command.count();
            if remaining != 0 {
                throw!( DeError::new( DeKind::ListLen{ expected: 2, got: 2+remaining }, e ));
            }
            info.push( structx!{ op, command });
        }

        Ok( info )
    }
}
