//! All error types returned by this crate are defined in this mod.

use crate::{
    Obj,
    incr_ref,
    decr_ref,
};

use enumx::def_impls;
use enumx::predefined::*;
use cex::{
    crate_error,
    impl_std_error,
};

use serde::{de, ser};

use std::{
    any::type_name,
    ffi::CStr,
    fmt::{
        self, Debug, Display,
    },
    marker::PhantomData,
    ptr,
};

/// Kind of `DeError`.
#[derive( Debug )]
pub enum DeKind {
    Custom,
    DictNoKey( Obj ),
    DictBadKey,
    DictBadVal,
    List{ err_idx: usize },
    ListLen{ expected: usize, got: usize },
    NotBool,
    NotChar,
    NotDict,
    NotEnum,
    NotI8,
    NotI16,
    NotI32,
    NotI64,
    NotISize,
    NotF32,
    NotF64,
    NotList,
    NotU8,
    NotU16,
    NotU32,
    NotU64,
    NotUnit,
    NotUSize,
    String,
}

/// Deserialize error, with full backtrace to the start of deserialization.
pub struct DeError( pub Vec<(DeKind, Obj)> );

impl Debug for DeError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "error: when deserializing" )?;
        for (idx, obj) in &self.0 {
            write!( formatter, "\n{:?} => {}", idx, obj.to_string() )?;
        }
        writeln!( formatter )
    }
}

impl DeError {
    pub fn new( de_kind: DeKind, obj: Obj ) -> Self {
        DeError( vec![( de_kind, obj )])
    }

    pub fn chain( de_kind: DeKind, obj: Obj, error: Self ) -> Self {
        let mut derror = DeError( Vec::with_capacity( error.0.len() + 1 ));
        derror.0.extend( error.0.into_iter() );
        derror.0.push(( de_kind, obj ));
        derror
    }
}

macro_rules! de_error_from {
    ($($error:ident),*) => {$(
        impl From<$error> for DeError {
            fn from( error: $error ) -> Self {
                DeError::new( DeKind::$error, error.0 )
            }
        }
    )*};
}

de_error_from!( NotDict, NotList );

/// Serialization error, which never happens.
#[derive( Debug )]
pub enum SerError {}

impl ser::Error for SerError {
    fn custom<T: Display>( _msg: T ) -> Self {
        unreachable!()
    }
}

impl Display for SerError {
    fn fmt( &self, _formatter: &mut fmt::Formatter ) -> fmt::Result {
        unreachable!()
    }
}

impl std::error::Error for SerError {}

impl de::Error for DeError {
    fn custom<T: Display>( msg: T ) -> Self {
        DeError::new( DeKind::Custom, msg.to_string().into() )
    }
}

impl_std_error!{ DeError }

/// `Tcl_Init()` fails to find and source initialization script.
#[derive( Debug )]
pub struct TclInitError( pub InterpError );
impl_std_error!{ TclInitError }

/// `Tcl_CreateInterp()` returns null pointer.
#[derive( Debug )]
pub struct NullInterp;
impl_std_error!{ NullInterp }

/// Fails to cast a Tcl obj to expected type.
#[derive( Debug )]
pub struct MismatchedObjType;
impl_std_error!{ MismatchedObjType }

/// Fails to take the value of an obj because the value is borrowed.
#[derive( Debug )]
pub struct MoveBorrowedValue( pub Obj );
impl_std_error!{ MoveBorrowedValue }

/// Fails to take the value of an obj because the obj is shared.
#[derive( Debug )]
pub struct MoveSharedObj( pub Obj );
impl_std_error!{ MoveSharedObj }

/// Fails to retrieve data because the internal data pointer is null.
#[derive( Debug )]
pub struct NullDataPtr( pub Obj );
impl_std_error!{ NullDataPtr }

/// Fails to get repeatly sequence of `T` in some list.
#[derive( Debug )]
pub struct NotSeqOf<T> {
    pub obj  : Obj,
    pub mark : PhantomData<T>,
}

impl_std_error!{ NotSeqOf<T> }

impl<T> NotSeqOf<T> {
    pub fn new( obj: Obj ) -> Self {
        NotSeqOf{ obj, mark: PhantomData }
    }
}

/// Type erased version of `NotSeqOf`.
#[derive( Debug )]
pub struct NotSeq {
    pub obj : Obj,
    pub ty  : &'static str,
}

impl_std_error!{ NotSeq }

impl<T> From<NotSeqOf<T>> for NotSeq {
    fn from( not_seq_of: NotSeqOf<T> ) -> Self {
        NotSeq{ obj: not_seq_of.obj, ty: type_name::<T>() }
    }
}

/// Fails on list operations when converting `Obj` to a Tcl list.
#[derive( Debug )]
pub struct NotList( pub Obj );
impl_std_error!{ NotList }

/// Fails on dict operations when converting `Obj` to a Tcl dict.
#[derive( Debug )]
pub struct NotDict( pub Obj );
impl_std_error!{ NotDict }

/// Mutating shared dict obj is not allowed.
#[derive( Debug )]
pub struct MutateSharedDict( pub Obj );
impl_std_error!{ MutateSharedDict }

/// Errors returned by Tcl interpreter.
pub struct InterpError {
    /// The returned value of en error.
    pub obj     : Obj,
    /// The returned options of en error.
    pub options : Obj,
}

impl InterpError {
    fn get_value( &self, name: &[u8] ) -> Obj {
        unsafe {
            let name = CStr::from_bytes_with_nul_unchecked( name );
            let key = clib::Tcl_NewStringObj( name.as_ptr(), -1 );
            let mut value = ptr::null_mut::<clib::Tcl_Obj>();
            incr_ref( key );
            clib::Tcl_DictObjGet( ptr::null_mut(), self.options.as_ptr(), key, &mut value );
            let value = Obj::from_raw( value );
            decr_ref( key );
            value
        }
    }

    /// Returns the value of `-errorinfo` option.
    pub fn info( &self ) -> String { self.get_value( b"-errorinfo\0" ).get_string() }

    /// Returns the value of `-errorcode` option.
    pub fn code( &self ) -> Obj { self.get_value( b"-errorcode\0" )}
}

impl Debug for InterpError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "{}", self.info() )
    }
}

impl Display for InterpError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        formatter.write_str( &self.obj.get_string() )
    }
}

impl std::error::Error for InterpError {}

crate_error!{
    #[derive( Debug )]
    pub enum TclError {
        DeError          ,
        TclInitError     ,
        NullInterp       ,
        MismatchedObjType,
        MoveBorrowedValue,
        MoveSharedObj    ,
        NullDataPtr      ,
        NotList          ,
        NotDict          ,
        NotSeq           ,
        MutateSharedDict ,
        InterpError      ,
    }
}

impl_std_error!{ TclError }

#[cfg( not( any( feature="cex_log", feature="cex_env_log" )))]
impl<T> From<NotSeqOf<T>> for TclError {
    fn from( e: NotSeqOf<T> ) -> Self { TclError::NotSeq( NotSeq::from( e ))}
}

#[cfg( any( feature="cex_log", feature="cex_env_log" ))]
impl<T> From<cex::Log<NotSeqOf<T>>> for TclError {
    fn from( e: cex::Log<NotSeqOf<T>> ) -> Self {
        TclError::NotSeq( cex::Log{ error: NotSeq::from( e.error ), agent: e.agent })
    }
}

#[doc( hidden )]
pub trait IntoTclError {
    fn into_tcl_error( self ) -> TclError;
}

impl<E: IntoTclError> From<E> for TclError {
    fn from( e: E ) -> Self { e.into_tcl_error() }
}

def_impls! {
    impl IntoTclError for Enum![1..=4]
        where _Variants!(): Into<TclError>
    {
        fn into_tcl_error( self ) -> TclError {
            _match!(
                _variant!().into()
            )
        }
    }
}

/// The crate result type for users who do not want to use checked exceptions( `#[cex]` ).
pub type TclResult<T> = Result<T, TclError>;
