//! High-level bindings to Tcl 8.6
//!
//! The crate tcl is bindings to Tcl programming language, aiming at providing safe and easy to use API.
//!
//! # Quickstart
//!
//! ## `std::Convert` between Rust values and Tcl objects.
//!
//! ```rust
//! use std::convert::TryFrom;
//! use tcl::*;
//!
//! let obj = Obj::from( 0 );
//! assert_eq!( obj.to_string(), "0" );
//! assert_eq!( i32::try_from( obj )?, 0 );
//!
//! let obj = Obj::from( 1 );
//! assert_eq!( obj.as_i32(), 1 );
//! assert_eq!( obj.as_f64(), 1.0 );
//! assert_eq!( obj.as_bool(), true );
//!
//! let obj = Obj::from(( false, 42, "answer".to_owned() ));
//! assert_eq!( obj.to_string(), "0 42 answer" );
//! assert_eq!( <(bool,i32,String)>::try_from( obj )?,
//!     (false, 42, "answer".to_owned() )
//! );
//!
//! let v = vec![ "alpha".to_owned(), "beta".to_owned(), "gamma".to_owned() ];
//! let obj: Obj = v.clone().into();
//! assert_eq!( obj.to_string(), "alpha beta gamma" );
//! assert_eq!( Vec::<String>::try_from( obj )?, v );
//!
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert( "alpha".to_owned(), 1 );
//! map.insert( "beta" .to_owned(), 2 );
//! map.insert( "gamma".to_owned(), 3 );
//!
//! let obj: Obj = map.clone().into();
//! assert_eq!( HashMap::<String, i32>::try_from( obj )?, map );
//!
//! # Ok::<(),TclError>(())
//! ```
//!
//! ## User-defined types `deserialize`d / `try_from` Tcl objects.
//!
//! ```rust
//! use tcl::*;
//!
//! #[derive( Clone, PartialEq, Debug, serde::Deserialize )]
//! #[derive( TryFromDe )]
//! struct Struct{ a: i32, b: bool, c: f64 }
//!
//! let obj = Obj::from( "a 1 b false c 3.14" );
//! let v: Struct = from_obj( obj.clone() )?;
//! assert_eq!( v, Struct{ a: 1, b: false, c: 3.14 });
//!
//! let v: Struct = obj.clone().try_into()?;
//! assert_eq!( v, Struct{ a: 1, b: false, c: 3.14 });
//!
//! # Ok::<(),TclError>(())
//! ```
//!
//! ## Use `Tcl<T>` to store Rust values in Tcl `Obj`s, an vice-vesa.
//!
//! ```rust
//! use std::convert::TryFrom;
//! use tcl::*;
//!
//! let obj = Tcl::new_obj( vec![ 1, 1, 2, 3, 5, 8 ]);
//! let tcl_obj = Tcl::<Vec<i32>>::try_from( obj )?;
//! assert_eq!( tcl_obj.into_inner(), vec![ 1, 1, 2, 3, 5, 8 ]);
//!
//! # Ok::<(),TclError>(())
//! ```
//!
//! ## Run Tcl scripts
//!
//! ```rust
//! use tcl::*;
//!
//! let interpreter = Interpreter::new()?;
//! let a = 3;
//! let b = 7;
//! let c = interpreter.eval(( "expr", a, "*", b ))?;
//! assert_eq!( a*b, c.as_i32() );
//!
//! # Ok::<(),TclError>(())
//! ```
//!
//! ## Register Rust functions as tcl commands, the unsafe way
//!
//! ```rust
//! use tcl::*;
//!
//! #[proc] fn mul( a: i32, b: i32 ) -> TclResult<i32> { Ok( a * b )}
//!
//! let interpreter = Interpreter::new()?;
//! unsafe { // it's safe for `#[proc] fn`.
//!     interpreter.def_proc( "mul", mul );
//! }
//! let c = interpreter.eval( "mul 3 7" )?;
//! assert_eq!( c.as_i32(), 21 );
//!
//! # Ok::<(),TclError>(())
//! ```
//!
//! ## Register Rust functions as tcl commands, the safe way
//!
//! ```rust
//! use tcl::*;
//!
//! let interpreter = Interpreter::new()?;
//!
//! let cmd = tclfn!( &interpreter, /*cmd: "mul", args: "",*/
//!     fn mul( a: i32, b: i32 ) -> TclResult<i32> { Ok( a * b )}
//! );
//!
//! let c = interpreter.eval( "mul 3 7" )?;
//! assert_eq!( c.as_i32(), 21 );
//!
//! # Ok::<(),TclError>(())
//! ```
//!
//! ## Register Rust closures as tcl commands
//!
//! ```rust
//! use tcl::*;
//!
//! let offset = 0;
//! let interpreter = Interpreter::new()?;
//!
//! let cmd = tclosure!( &interpreter, /*cmd: "mul", args: "",*/
//!     move |a: i32, b: i32| -> TclResult<i32> { Ok( a * b + offset )}
//! );
//!
//! let a = 3;
//! let b = 7;
//! let c = interpreter.eval(( "eval", cmd, a, b ))?;
//! assert_eq!( c.as_i32(), 21 );
//!
//! # Ok::<(),TclError>(())
//! ```

/// The following items will be seen in `tcl_derive`'s proc macros.
/// Let's reexport them from `crate clib`,
/// otherwise the `crate tcl`'s users will depend on `clib` too.
pub mod reexport_clib {
    pub use clib::{
        ClientData,
        TCL_ERROR,
        TCL_OK,
        Tcl_Interp,
        Tcl_InvalidateStringRep,
        Tcl_Obj,
        Tcl_SetObjResult,
        Tcl_WrongNumArgs,
    };
}

pub use tcl_derive::{
    TryFromDe,
    proc,
    tclfn,
    tclosure,
};

use std::{
    env,
    ffi::CString,
    sync::Once,
};

macro_rules! tcl_panic {
    ( $msg:expr ) => {{
        let s = CString::new( $msg ).unwrap_or_default();
        #[allow( unused_unsafe )]
        unsafe{ clib::Tcl_Panic( s.as_c_str().as_ptr() ); }
        unreachable!()
    }};
}

/// Aborts the program instead of `panic!()` in FFI callbacks.
pub trait UnwrapOrAbort {
    type Inner;

    /// Returns the value on success, otherwise aborts the program with `message`.
    fn unwrap_or_abort( self, message: &str ) -> Self::Inner;
}

impl<T> UnwrapOrAbort for Option<T> {
    type Inner = T;

    #[allow( unreachable_code )]
    fn unwrap_or_abort( self, message: &str ) -> Self::Inner {
        self.unwrap_or_else( || tcl_panic!( format!( "unwrap_or_abort on None value: {}", message ).as_str() ))
    }
}

impl<T,E> UnwrapOrAbort for Result<T,E>
    where E: std::fmt::Debug
{
    type Inner = T;

    #[allow( unreachable_code )]
    fn unwrap_or_abort( self, message: &str ) -> Self::Inner {
        self.unwrap_or_else( |err| tcl_panic!( format!( "unwrap_or_abort on Err: {:?}\n{}", err, message ).as_str() ))
    }
}

mod after;

pub mod interp;
pub use interp::{CodeToResult, Interpreter, Interp, ObjCmdProc};

pub mod obj;
pub use obj::{Obj, incr_ref, decr_ref};

pub mod ext;
pub use ext::Tcl;

pub mod ser;
pub use ser::{Serializer, to_c_str, to_string};

mod de;
pub use de::from_obj;

pub mod error;
pub use error::{
    IntoTclError,
    TclError,
    TclResult,
};

pub mod dict;
pub use dict::DictIter;

pub mod list;

mod trace;

mod update;

static INIT: Once = Once::new();

pub(crate) fn init() {
    INIT.call_once( || {
        let arg0 = CString::new( env::args().next().unwrap() ).unwrap();
        unsafe {
            clib::Tcl_FindExecutable( arg0.as_ptr() );
            if clib::Tcl_GetNameOfExecutable().is_null() {
                panic!( "failed to initialize Tcl" );
            }
        }
    });
}
