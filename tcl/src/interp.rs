//! Tcl interpreter.

use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    incr_ref,
    error::{
        NullInterp,
        TclInitError,
        InterpError,
    },
    obj::Obj,
};

use std::{
    ffi::CString,
    mem,
    ops::Deref,
    os::raw::{c_double, c_int, c_longlong},
    path::Path,
    ptr::{self, NonNull},
};

pub(crate) type Result<T, E=InterpError> = std::result::Result<T,E>;

/// Converts integral error code returned by Tcl's C API to Rust's `Result` type.
pub trait CodeToResult where Self: Into<c_int> {
    /// Converts `TCL_ERROR` to `Err(InterpError)`, which stores returned Tcl `Obj` and options.
    fn code_to_result( self, interp: &Interp ) -> Result<()> {
        let tcl_error = clib::TCL_ERROR as c_int;
        if self.into() == tcl_error {
            Err( interp.error() )
        } else {
            Ok(())
        }
    }

    /// Converts `TCL_OK` to `Ok(())`, otherwise `Err(())`.
    fn unit_result( self ) -> Result<(),()> {
        if self.into() == clib::TCL_OK as c_int {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Test if it is `TCL_OK`.
    fn is_ok( self ) -> bool {
        self.into() == clib::TCL_OK as c_int
    }
}

impl CodeToResult for c_int {}

/// Tcl interpeter which has ownership.
pub struct Interpreter( Interp );

impl Interpreter {
    /// Creates a Tcl interpeter.
    #[cex]
    pub fn new() -> Result!( Self throws NullInterp, TclInitError ) {
        crate::init();

        let tcl_interp = match NonNull::new( unsafe{ clib::Tcl_CreateInterp() }) {
            Some( tcl_interp ) => Ok( tcl_interp ),
            None => Err( NullInterp ),
        };

        let this = Interpreter( Interp( tcl_interp? ));
        unsafe{ clib::Tcl_Init( (this.0).0.as_ptr() )}
            .code_to_result( &this )
            .map_err( TclInitError )?;
        Ok( this )
    }

    /// Consumes the interpreter, returning a wrapped raw nonnull pointer.
    pub fn into_raw_non_null( self ) -> NonNull<clib::Tcl_Interp> {
        let raw_non_null = (self.0).0;
        mem::forget( self );
        raw_non_null
    }
}

impl Deref for Interpreter {
    type Target = Interp;
    fn deref( &self ) -> &Interp { &self.0 }
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        unsafe{ clib::Tcl_DeleteInterp( (self.0).0.as_ptr() )}
    }
}

/// Tcl interpreter which does not have ownership.
#[derive( Clone, Debug )]
pub struct Interp( NonNull<clib::Tcl_Interp> );

/// API for registering Rust functions as Tcl commands.
pub type ObjCmdProc = extern "C" fn( clib::ClientData, *mut clib::Tcl_Interp, c_int, *const *mut clib::Tcl_Obj ) -> c_int;

impl Interp {
    /// Constructs a non-owning interpreter from a raw pointer.
    ///
    /// # Safety
    ///
    /// The raw pointer should be provided by Tcl's callback, or previously obtained by [`Interp::as_ptr()`].
    pub unsafe fn from_raw( raw: *mut clib::Tcl_Interp ) -> Result<Self, NullInterp> {
        match NonNull::new( raw ) {
            Some( interp ) => Ok( Interp( interp )),
            None           => Err( NullInterp ),
        }
    }

    /// Obtains a raw pointer, required in Tcl's C API.
    pub fn as_ptr( &self ) -> *mut clib::Tcl_Interp {
        self.0.as_ptr()
    }

    /// Returns the result of interpreter as a value.
    pub fn result( &self ) -> Obj {
        unsafe{ Obj::from_raw( clib::Tcl_GetObjResult( self.0.as_ptr() ))}
    }

    /// Returns the result and options of interpreter as an error.
    pub fn error( &self ) -> InterpError {
        let options = unsafe {
            Obj::from_raw( clib::Tcl_GetReturnOptions( self.0.as_ptr(), clib::TCL_ERROR as c_int ))
        };

        let obj = self.result();
        InterpError{ obj, options }
    }

    fn obj_from_ptr( &self, obj_ptr: *mut clib::Tcl_Obj ) -> Result<Obj> {
        if obj_ptr.is_null() {
            Err( self.error() )
        } else {
            Ok( unsafe{ Obj::from_raw( obj_ptr )})
        }
    }

    /// Executes the `code` until either an error occurs or the end of the script is reached.
    /// Returns a value stored in obj on success.
    pub fn eval( &self, code: impl Into<Obj> ) -> Result<Obj> {
        self.eval_with_flags( code, 0 )
    }

    /// Executes the `code` until either an error occurs or the end of the script is reached.
    /// Additional options can be specified using flags `TCL_EVAL_GLOBAL` and `TCL_EVAL_DIRECT`.
    pub fn eval_with_flags( &self, code: impl Into<Obj>, flags: c_int ) -> Result<Obj> {
        let code = code.into();
        #[cfg( debug_assertions )] println!( "{}", code );

        unsafe {
            clib::Tcl_EvalObjEx( self.0.as_ptr(), code.as_ptr(), flags ).code_to_result( self )?;
        }

        Ok( self.result() )
    }

    /// Executes the `code` until either an error occurs or the end of the script is reached.
    /// Returns `()` on success.
    pub fn run( &self, code: impl Into<Obj> ) -> Result<()> {
        self.eval_with_flags( code, 0 )?;
        Ok(())
    }

    /// Reads the value of variable named `var` defined in the interpreter.
    pub fn get( &self, var: impl Into<Obj> ) -> Result<Obj> {
        let var = var.into();
        let flags = ( clib::TCL_LEAVE_ERR_MSG ) as c_int;
        let ptr = unsafe{ clib::Tcl_ObjGetVar2( self.0.as_ptr(), var.as_ptr(), ptr::null_mut(), flags )};
        self.obj_from_ptr( ptr )
    }

    /// Reads the value of `elem` of array `arr` defined in the interpreter.
    pub fn arr_get( &self, arr: impl Into<Obj>, elem: impl Into<Obj> ) -> Result<Obj> {
        let ( arr, elem ) = ( arr.into(), elem.into() );
        let flags = ( clib::TCL_LEAVE_ERR_MSG ) as c_int;
        let ptr = unsafe{ clib::Tcl_ObjGetVar2( self.0.as_ptr(), arr.as_ptr(), elem.as_ptr(), flags )};
        self.obj_from_ptr( ptr )
    }

    /// Sets the value of variable named `lhs` defined in the interpreter to be `rhs`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// let x = Obj::from("{");
    /// interpreter.set( "x", "{" );
    /// assert_eq!( interpreter.get("x").unwrap().to_string(), "{" );
    /// ```
    pub fn set( &self, lhs: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( lhs, rhs ) = ( lhs.into(), rhs.into() );
        let flags = ( clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe {
            let obj_ptr = clib::Tcl_ObjSetVar2( self.0.as_ptr(), lhs.as_ptr(), ptr::null_mut(), rhs.as_ptr(), flags );
            assert!( !obj_ptr.is_null() );
            incr_ref( obj_ptr );
            Obj::from_raw( obj_ptr )
        }
    }

    /// Sets the value of variable named `lhs` defined in the interpreter to be `rhs`.
    /// The value will be converted to a list element.
    /// See <https://www.tcl.tk/man/tcl/TclLib/SetVar.htm#M10> for more.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// interpreter.set_list_elem( "x", "{" );
    /// assert_eq!( interpreter.get("x").unwrap().to_string(), "\\{" );
    /// ```
    pub fn set_list_elem( &self, lhs: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( lhs, rhs ) = ( lhs.into(), rhs.into() );
        let flags = ( clib::TCL_LIST_ELEMENT | clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe {
            let obj_ptr = clib::Tcl_ObjSetVar2( self.0.as_ptr(), lhs.as_ptr(), ptr::null_mut(), rhs.as_ptr(), flags );
            assert!( !obj_ptr.is_null() );
            incr_ref( obj_ptr );
            Obj::from_raw( obj_ptr )
        }
    }

    /// Sets the value of `elem` of array `arr` defined in the interpreter, to be `rhs`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interp = Interpreter::new().unwrap();
    /// interp.arr_set( "color", "red"  , 0xff0000 );
    /// interp.arr_set( "color", "green", 0x00ff00 );
    /// interp.arr_set( "color", "blue" , 0x0000ff );
    /// assert_eq!( interp.arr_get( "color", "red"   ).unwrap().as_u32(), 0xff0000 );
    /// assert_eq!( interp.arr_get( "color", "green" ).unwrap().as_u32(), 0x00ff00 );
    /// assert_eq!( interp.arr_get( "color", "blue"  ).unwrap().as_u32(), 0x0000ff );
    ///
    /// tclfn!( &interp, fn rust_fn() -> TclResult<()> {
    ///     let interp = tcl_interp!();
    ///     assert_eq!( interp.arr_get( "color", "red"   ).unwrap().as_u32(), 0xff0000 );
    ///     assert_eq!( interp.arr_get( "color", "green" ).unwrap().as_u32(), 0x00ff00 );
    ///     assert_eq!( interp.arr_get( "color", "blue"  ).unwrap().as_u32(), 0x0000ff );
    ///     Ok(())
    /// });
    ///
    /// interp.run( "rust_fn" ).unwrap();
    /// ```
    pub fn arr_set( &self, arr: impl Into<Obj>, elem: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( arr, elem, rhs ) = ( arr.into(), elem.into(), rhs.into() );
        let flags = ( clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe {
            let obj_ptr = clib::Tcl_ObjSetVar2( self.0.as_ptr(), arr.as_ptr(), elem.as_ptr(), rhs.as_ptr(), flags );
            assert!( !obj_ptr.is_null() );
            incr_ref( obj_ptr );
            Obj::from_raw( obj_ptr )
        }
    }

    /// Sets the value of `elem` of array `arr` defined in the interpreter, to be `rhs`.
    /// The value will be converted to a list element.
    /// See <https://www.tcl.tk/man/tcl/TclLib/SetVar.htm#M10> for more.
    pub fn arr_set_list_elem( &self, arr: impl Into<Obj>, elem: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( arr, elem, rhs ) = ( arr.into(), elem.into(), rhs.into() );
        let flags = ( clib::TCL_LIST_ELEMENT | clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe {
            let obj_ptr = clib::Tcl_ObjSetVar2( self.0.as_ptr(), arr.as_ptr(), elem.as_ptr(), rhs.as_ptr(), flags );
            assert!( !obj_ptr.is_null() );
            incr_ref( obj_ptr );
            Obj::from_raw( obj_ptr )
        }
    }

    /// Append value `rhs` to the variable `lhs` defined in the interpreter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// interpreter.append( "x", 0 );
    /// interpreter.append( "x", 0 );
    /// assert_eq!( interpreter.get("x").unwrap().to_string(), "00" );
    /// ```
    pub fn append( &self, lhs: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( lhs, rhs ) = ( lhs.into(), rhs.into() );
        let flags = ( clib::TCL_APPEND_VALUE | clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe{ Obj::from_raw(
            clib::Tcl_ObjSetVar2( self.0.as_ptr(), lhs.as_ptr(), ptr::null_mut(), rhs.as_ptr(), flags ))}
    }

    /// Append value `rhs` to the variable `lhs` defined in the interpreter.
    /// The value will be converted to a list element.
    /// See <https://www.tcl.tk/man/tcl/TclLib/SetVar.htm#M10> for more.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// interpreter.append_list_elem( "x", 0 );
    /// interpreter.append_list_elem( "x", 0 );
    /// assert_eq!( interpreter.get("x").unwrap().to_string(), "0 0" );
    /// ```
    pub fn append_list_elem( &self, lhs: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( lhs, rhs ) = ( lhs.into(), rhs.into() );
        let flags = ( clib::TCL_APPEND_VALUE | clib::TCL_LIST_ELEMENT | clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe{ Obj::from_raw(
            clib::Tcl_ObjSetVar2( self.0.as_ptr(), lhs.as_ptr(), ptr::null_mut(), rhs.as_ptr(), flags ))}
    }

    /// Append value `rhs` to the variable `elem` of array `arr`, defined in the interpreter.
    pub fn arr_append( &self, arr: impl Into<Obj>, elem: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( arr, elem, rhs ) = ( arr.into(), elem.into(), rhs.into() );
        let flags = ( clib::TCL_APPEND_VALUE | clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe{ Obj::from_raw(
            clib::Tcl_ObjSetVar2( self.0.as_ptr(), arr.as_ptr(), elem.as_ptr(), rhs.as_ptr(), flags ))}
    }

    /// Append value `rhs` to the variable `elem` of array `arr`, defined in the interpreter.
    /// The value will be converted to a list element.
    /// See <https://www.tcl.tk/man/tcl/TclLib/SetVar.htm#M10> for more.
    pub fn arr_append_list_elem( &self, arr: impl Into<Obj>, elem: impl Into<Obj>, rhs: impl Into<Obj> ) -> Obj {
        let ( arr, elem, rhs ) = ( arr.into(), elem.into(), rhs.into() );
        let flags = ( clib::TCL_APPEND_VALUE | clib::TCL_LIST_ELEMENT | clib::TCL_LEAVE_ERR_MSG ) as c_int;
        unsafe{ Obj::from_raw(
            clib::Tcl_ObjSetVar2( self.0.as_ptr(), arr.as_ptr(), elem.as_ptr(), rhs.as_ptr(), flags ))}
    }

    /// This command removes one variable named `var_name`. If a name refers to an
    /// element of an array then that element is removed without affecting the rest of
    /// the array. If a name consists of an array name with no parenthesized index, then
    /// the entire array is deleted. An error can occur when the named variable does not
    /// exist, or the name refers to an array element but the variable is a scalar, or
    /// the name refers to a variable in a non-existent namespace.
    pub fn unset( &self, var_name: &str ) -> Result<()> {
        let flags = ( clib::TCL_LEAVE_ERR_MSG ) as c_int;
        if let Ok( var_name ) = CString::new( var_name ) {
            unsafe {
                clib::Tcl_UnsetVar( self.0.as_ptr(), var_name.as_ptr(), flags )
                    .code_to_result( self )
            }
        } else {
            Ok(())
        }
    }

    /// This command removes one variable named `var_name`. If a name refers to an
    /// element of an array then that element is removed without affecting the rest of
    /// the array. If a name consists of an array name with no parenthesized index, then
    /// the entire array is deleted.
    pub fn unset_nocomplain( &self, var_name: &str ) {
        let flags: c_int = 0;
        if let Ok( var_name ) = CString::new( var_name ) {
            unsafe {
                clib::Tcl_UnsetVar( self.0.as_ptr(), var_name.as_ptr(), flags );
            }
        }
    }

    /// This command removes an element named `elem_name` of an array named `arr_name`.
    /// An error can occur when the element does not exist, or the name refers to an
    /// array element but the variable is a scalar, or the name refers to the array in a
    /// non-existent namespace.
    pub fn arr_unset( &self, arr_name: &str, elem_name: &str ) -> Result<()> {
        let flags = ( clib::TCL_LEAVE_ERR_MSG ) as c_int;
        if let Ok( arr_name ) = CString::new( arr_name ) {
            if let Ok( elem_name ) = CString::new( elem_name ) {
                return unsafe {
                    clib::Tcl_UnsetVar2( self.0.as_ptr(), arr_name.as_ptr(), elem_name.as_ptr(), flags )
                        .code_to_result( self )
                };
            }
        }
        Ok(())
    }

    /// This command removes an element named `elem_name` of an array named `arr_name`.
    pub fn arr_unset_nocomplain( &self, arr_name: &str, elem_name: &str ) {
        let flags: c_int = 0;
        if let Ok( arr_name ) = CString::new( arr_name ) {
            if let Ok( elem_name ) = CString::new( elem_name ) {
                unsafe {
                    clib::Tcl_UnsetVar2( self.0.as_ptr(), arr_name.as_ptr(), elem_name.as_ptr(), flags );
                }
            }
        }
    }

    /// Converts `val` into a boolean value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// assert_eq!( interpreter.boolean( false   ).unwrap(), false );
    /// assert_eq!( interpreter.boolean( true    ).unwrap(), true  );
    /// assert_eq!( interpreter.boolean( 0       ).unwrap(), false );
    /// assert_eq!( interpreter.boolean( 1       ).unwrap(), true  );
    /// assert_eq!( interpreter.boolean( 2       ).unwrap(), true  );
    /// assert_eq!( interpreter.boolean( "0"     ).unwrap(), false );
    /// assert_eq!( interpreter.boolean( "1"     ).unwrap(), true  );
    /// assert_eq!( interpreter.boolean( "false" ).unwrap(), false );
    /// assert_eq!( interpreter.boolean( "true"  ).unwrap(), true  );
    /// assert_eq!( interpreter.boolean( "FaLsE" ).unwrap(), false );
    /// assert_eq!( interpreter.boolean( "tRuE"  ).unwrap(), true  );
    /// assert!(    interpreter.boolean( "Trueman" ).is_err() );
    /// ```
    pub fn boolean( &self, val: impl Into<Obj> ) -> Result<bool> {
        let mut value: c_int = 0;
        unsafe {
            clib::Tcl_GetBooleanFromObj( self.0.as_ptr(), val.into().as_ptr(), &mut value as *mut _ )
                .code_to_result( self )
                .map( |_| value != 0 )
        }
    }

    /// Gets boolean value of variable `var` defined in the intepreter.
    pub fn get_boolean( &self, var: impl Into<Obj> ) -> Result<bool> {
        let val = self.get( var.into() )?;
        self.boolean( val )
    }

    /// Converts `val` into a `c_int` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// assert_eq!( interpreter.int( 0      ).unwrap(), 0 );
    /// assert_eq!( interpreter.int( "0"    ).unwrap(), 0 );
    /// assert_eq!( interpreter.int( false  ).unwrap(), 0 );
    /// assert_eq!( interpreter.int( true   ).unwrap(), 1 );
    /// assert!(    interpreter.int( "zero" ).is_err() );
    /// ```
    pub fn int( &self, val: impl Into<Obj> ) -> Result<c_int> {
        let mut value: c_int = 0;
        unsafe {
            clib::Tcl_GetIntFromObj( self.0.as_ptr(), val.into().as_ptr(), &mut value as *mut _ )
                .code_to_result( self )
                .map( |_| value )
        }
    }

    /// Gets `c_int` value of variable `var` defined in the intepreter.
    pub fn get_int( &self, var: impl Into<Obj> ) -> Result<c_int> {
        let val = self.get( var.into() )?;
        self.int( val )
    }

    /// Sets the variable `lhs`'s value to be `rhs`, which is a `c_int`.
    pub fn set_int( &self, lhs: impl Into<Obj>, rhs: c_int ) -> Obj {
        self.set( lhs, Obj::from( rhs ))
    }

    /// Converts `val` into a `c_longlong` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// assert_eq!( interpreter.longlong( 0      ).unwrap(), 0 );
    /// assert_eq!( interpreter.longlong( "0"    ).unwrap(), 0 );
    /// assert_eq!( interpreter.longlong( false  ).unwrap(), 0 );
    /// assert_eq!( interpreter.longlong( true   ).unwrap(), 1 );
    /// assert!(    interpreter.longlong( "zero" ).is_err() );
    /// ```
    pub fn longlong( &self, val: impl Into<Obj> ) -> Result<c_longlong> {
        let mut value: c_longlong = 0;
        unsafe {
            clib::Tcl_GetWideIntFromObj( self.0.as_ptr(), val.into().as_ptr(), &mut value as *mut _ )
                .code_to_result( self )
                .map( |_| value )
        }
    }

    /// Gets `c_longlong` value of variable `var` defined in the intepreter.
    pub fn get_longlong( &self, var: impl Into<Obj> ) -> Result<c_longlong> {
        let val = self.get( var.into() )?;
        self.longlong( val )
    }

    /// Sets the variable `lhs`'s value to be `rhs`, which is a `c_longlong`.
    pub fn set_longlong( &self, lhs: impl Into<Obj>, rhs: c_longlong ) -> Obj {
        self.set( lhs, Obj::from( rhs ))
    }

    /// Converts `val` into a `c_double` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let interpreter = Interpreter::new().unwrap();
    /// assert_eq!( interpreter.double( 0.0    ).unwrap(), 0.0 );
    /// assert_eq!( interpreter.double( "0.0"  ).unwrap(), 0.0 );
    /// assert_eq!( interpreter.double( false  ).unwrap(), 0.0 );
    /// assert_eq!( interpreter.double( true   ).unwrap(), 1.0 );
    /// assert!(    interpreter.double( "zero" ).is_err() );
    /// ```
    pub fn double( &self, val: impl Into<Obj> ) -> Result<c_double> {
        let mut value: c_double = 0.0;
        unsafe {
            let (arg0,arg1) = (self.0.as_ptr(), val.into());
            clib::Tcl_GetDoubleFromObj( arg0, arg1.as_ptr(), &mut value as *mut _ )
                .code_to_result( self )
                .map( |_| value )
        }
    }

    /// Gets `c_double` value of variable `var` defined in the intepreter.
    pub fn get_double( &self, var: impl Into<Obj> ) -> Result<c_double> {
        let var = var.into();
        let val = self.get( var )?;
        self.double( val )
    }

    /// Sets the variable `lhs`'s value to be `rhs`, which is a `c_double`.
    pub fn set_double( &self, lhs: impl Into<Obj>, rhs: c_double ) -> Obj {
        self.set( lhs, Obj::from( rhs ))
    }

    /// Registers Rust function `proc` as a Tcl proc, with optional client data and destructor.
    ///
    /// # Safety
    ///
    /// According to <https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-panics>,
    /// a `panic!` across an FFI boundary is undefined behavior.
    ///
    /// Any user provided `proc` should not `panic!`.
    /// However, a `#[proc] fn` uses abort instead of panic, which is safe to register.
    pub unsafe fn def_proc_with_client_data( &self, name: &str, proc: ObjCmdProc, data: clib::ClientData, deleter: clib::Tcl_CmdDeleteProc ) {
        let name = CString::new( name ).expect("Tcl proc name should be CString.");
        clib::Tcl_CreateObjCommand( self.as_ptr(), name.as_c_str().as_ptr(), Some( proc ), data, deleter );
    }

    /// Registers Rust function `proc` as a Tcl proc, without client data nor destructor.
    ///
    /// # Safety
    ///
    /// According to <https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-panics>,
    /// a `panic!` across an FFI boundary is undefined behavior.
    ///
    /// Any user provided `proc` should not `panic!`.
    /// However, a `#[proc] fn` uses abort instead of panic, which is safe to register.
    pub unsafe fn def_proc( &self, name: &str, proc: ObjCmdProc ) {
        self.def_proc_with_client_data( name, proc, ptr::null_mut(), None );
    }

    /// This command is typically invoked by Tcl code that wishes to use a particular
    /// version of a particular package. The arguments indicate which package is wanted,
    /// and the command ensures that a suitable version of the package is loaded into
    /// the interpreter. If the command succeeds, it returns the version number that is
    /// loaded; otherwise it generates an error.
    ///
    /// Note: `path` will be appended to `auto_path`.
    pub fn package_load( &self, name: &str, path: impl AsRef<Path> )
        -> Result<Obj>
    {
        self.run(( "lappend", "auto_path", path.as_ref().to_string_lossy() ))?;
        self.eval(( "package", "require", name ))
    }

    /// This is equivalent to calling "package provide" with the specified package name and
    /// version.
    pub fn package_provide(&self, name: &str, version: &str) -> c_int {
        let name = CString::new(name).expect("Tcl package name should be CString.");
        let version = CString::new(version).expect("Tcl package version should be CString.");
        unsafe {
            clib::Tcl_PkgProvide(
                self.as_ptr(),
                name.as_c_str().as_ptr(),
                version.as_c_str().as_ptr(),
            )
        }
    }

    /// This command takes the contents of the specified file or resource and passes it
    /// to the Tcl interpreter as a text script. The return value from source is the
    /// return value of the last command executed in the script. If an error occurs in
    /// evaluating the contents of the script then the source command will return that
    /// error. If a return command is invoked from within the script then the remainder
    /// of the file will be skipped and the source command will return normally with the
    /// result from the return command.
    ///
    /// The end-of-file character for files is “\32” (^Z) for all platforms. The source
    /// command will read files up to this character. This restriction does not exist
    /// for the read or gets commands, allowing for files containing code and data
    /// segments (scripted documents). If you require a “^Z” in code for string
    /// comparison, you can use “\032” or “\u001a”, which will be safely substituted by
    /// the Tcl interpreter into “^Z”.
    ///
    /// A leading BOM (Byte order mark) contained in the file is ignored for unicode
    /// encodings (utf-8, unicode).
    pub fn source( &self, path: impl AsRef<Path> ) -> Result<Obj> {
        self.eval(( "source", path.as_ref().to_string_lossy() ))
    }
}

#[cfg(test)]
mod tests {
    use crate::TclResult;
    use super::*;

    #[test]
    fn test_to_string() {
        let x = Obj::from( "{x}" );

        assert_eq!( x.to_string(), "{x}" );
    }

    #[test]
    fn eval() {
        let interp = Interpreter::new().unwrap();

        assert_eq!( interp.eval( "expr {2 + 2}" ).unwrap().to_string(), "4" );
    }

    #[test]
    fn set_var() -> Result<()> {
        let interp = Interpreter::new().unwrap();

        let set = Obj::from( "set" );
        let x = Obj::from( "x" );
        interp.eval(( set, x, 5 )).ok();
        assert_eq!( interp.eval("return $x").unwrap().to_string(), "5" );
        assert_eq!( interp.get("x")?.to_string(), "5" );
        Ok(())
    }

    #[test]
    fn proc() {
        use clib::{Tcl_Interp, Tcl_Obj, ClientData};

        extern "C" fn rust_fn( _data: ClientData, tcl_interp: *mut Tcl_Interp, objc: c_int, objv: *const *mut Tcl_Obj,) -> c_int {
            match std::panic::catch_unwind( || {
                let interp = unsafe{ Interp::from_raw( tcl_interp ).unwrap() };
                let objs: &[*mut Tcl_Obj] = unsafe{ std::slice::from_raw_parts( objv, objc as usize )};
                let result = interp.eval( format!( "puts \"{}\"", objs.iter().fold( String::new(), |acc,obj| acc + &unsafe{ Obj::from_raw( *obj )}.to_string() + " " )));
                if result.is_ok() {
                    clib::TCL_OK as c_int
                } else {
                    clib::TCL_ERROR as c_int
                }
            }) {
                Ok( code ) => code,
                Err(_) => clib::TCL_ERROR as c_int, // abort is better.
            }
        }

        let interp = Interpreter::new().unwrap();

        unsafe{ interp.def_proc( "rust_fn", rust_fn )};
        interp.run( "rust_fn hello world" ).ok();
    }

    #[test]
    fn test_ref_type() -> TclResult<()> {
        #[derive( Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize )]
        struct Struct{ a: i32, b: bool, c: f64 }

        impl Drop for Struct{ fn drop( &mut self ) { println!( "Struct drop: {:?}", self as *mut _ ); }}

        use crate as tcl;
        pub(crate) use tcl::{
            Obj,
        };

        let interpreter = Interpreter::new().unwrap();

        #[tcl::proc]
        fn rust_fn( ref_struct: &mut Struct, i: i32 ) -> TclResult<bool> {
            println!( "ref_struct: {:?}, i: {}", ref_struct, i );
            println!( "ref_struct at {:?}", ref_struct as *const _ );
            ref_struct.b = true;
            tcl_invalidate_str_rep!( ref_struct );
            return Ok( ref_struct.b );
        }

        unsafe{ interpreter.def_proc( "rust_fn", rust_fn )};

        let x = tcl::Tcl::new_obj( Struct{ a:1, b:false, c:3.14 });
        interpreter.set( "x", x );
        interpreter.eval( "rust_fn x 42" )?;
        let result = interpreter.get("x")?.to_string();
        eprintln!( "cmp result" );
        assert_eq!( result, "a 1 b true c 3.14" );
        Ok(())
    }

    #[test]
    fn error() {
        let interp = Interpreter::new().unwrap();
        if let Err( err ) = interp.eval(( "badcommand", "or filename" )) {
            println!( "err:{:?}", err );
        }
    }

    #[test]
    fn package_provide() {
        let interp = Interpreter::new().unwrap();
        interp.package_provide("mypackage", "1.2.3");
        assert_eq!(
            interp
                .eval("package require mypackage")
                .unwrap()
                .get_string(),
            "1.2.3".to_string()
        );
    }
}
