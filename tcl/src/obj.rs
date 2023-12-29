//! # An overview of Tcl values, aka `Obj`s.
//!
//! Tcl's dual-ported values provide a general-purpose mechanism for storing and
//! exchanging Tcl values. They largely replace the use of strings in Tcl. For
//! example, they are used to store variable values, command arguments, command
//! results, and scripts. Tcl values behave like strings but also hold an internal
//! representation that can be manipulated more efficiently. For example, a Tcl list
//! is now represented as a value that holds the list's string representation as well
//! as an array of pointers to the values for each list element. Dual-ported values
//! avoid most runtime type conversions. They also improve the speed of many
//! operations since an appropriate representation is immediately available. The
//! interpreter itself uses Tcl values to cache the instruction bytecodes resulting
//! from compiling scripts.
//!
//! The two representations are a cache of each other and are computed lazily. That
//! is, each representation is only computed when necessary, it is computed from the
//! other representation, and, once computed, it is saved. In addition, a change in
//! one representation invalidates the other one. As an example, a Tcl program doing
//! integer calculations can operate directly on a variable's internal machine
//! integer representation without having to constantly convert between integers and
//! strings. Only when it needs a string representing the variable's value, say to
//! print it, will the program regenerate the string representation from the integer.
//! Although values contain an internal representation, their semantics are defined
//! in terms of strings: an up-to-date string can always be obtained, and any change
//! to the value will be reflected in that string when the value's string
//! representation is fetched.
//!
//! Values are allocated on the heap and are referenced using a smart pointer `Obj`.
//! Values are shared as much as possible. This significantly reduces storage
//! requirements because some values such as long lists are very large. Also, most
//! Tcl values are only read and never modified. This is especially true for
//! procedure arguments, which can be shared between the caller and the called
//! procedure. Assignment and argument binding is done by simply assigning a pointer
//! to the value. Reference counting is used to determine when it is safe to reclaim
//! a value's storage.
//!
//! Tcl values are typed. A value's internal representation is controlled by its
//! type. Several types are predefined in the Tcl core including integer, double,
//! list, and bytecode. Extension writers can extend the set of types by defining
//! their own Tcl_ObjType structs. This crate provides a registered type `Tcl<T>`.

use crate::{
    CodeToResult,
    error::{
        DeError,
        DeKind,
    },
};

use std::{
    borrow::Cow,
    collections::HashMap,
    convert::{TryFrom, TryInto},
    ffi::CString,
    fmt::{self, Debug},
    hash::Hash,
    mem,
    ops::Range,
    os::raw::{c_char, c_double, c_int, c_long, c_longlong, c_void},
    path::PathBuf,
    ptr::{NonNull, null_mut},
    slice,
};

use mutf8::mstr;

use tuplex::{HomoTuple, IntoHomoTuple, MapHomoTuple, Len};

/// A smart pointer that points to a referece-counted, heap-allocated Tcl value.
#[repr( transparent )]
pub struct Obj( pub(crate) NonNull<clib::Tcl_Obj> );

impl Debug for Obj {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        let obj = unsafe{ self.0.as_ref() };
        f.debug_struct("Obj")
         .field( "@", &(obj as *const _) )
         .field( "refCount", &obj.refCount )
         .field( "string rep", &self.to_string() )
         .finish()
    }
}

impl Clone for Obj {
    fn clone( &self ) -> Obj {
        unsafe{ incr_ref( self.as_ptr() )}
        Obj( self.0 )
    }
}

impl Default for Obj {
    fn default() -> Obj { Obj::new() }
}

impl Drop for Obj {
    fn drop( &mut self ) {
        unsafe{ decr_ref( self.as_ptr() ); }
    }
}

impl Obj {
    /// Creates a new, empty `Obj` on heap, with reference count 1.
    pub fn new() -> Obj {
        crate::init();

        let tcl_obj = unsafe{ clib::Tcl_NewObj() };
        unsafe{ incr_ref( tcl_obj ); }
        Obj( NonNull::new( tcl_obj ).expect( "Tcl_NewObj should return non null ptr" ))
    }

    /// Constructs an `Obj` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The raw pointer should be provided by Tcl's callback, or previously obtained by [`Obj::as_ptr()`].
    pub unsafe fn from_raw( tcl_obj: *mut clib::Tcl_Obj ) -> Obj {
        crate::init();

        if tcl_obj.is_null() {
            panic!( "Tcl_NewObj should return non null ptr" );
        } else {
            incr_ref( tcl_obj );
            Obj( NonNull::new_unchecked( tcl_obj ))
        }
    }

    /// Provides a raw pointer to the obj.
    pub fn as_ptr( &self ) -> *mut clib::Tcl_Obj {
        self.0.as_ptr()
    }

    /// Consumes the obj, returning the raw pointer.
    /// To avoid a memory leak the pointer must be converted back to an `Obj` using `Obj::from_raw`.
    pub fn into_raw( self ) -> *mut clib::Tcl_Obj {
        let ptr = self.0.as_ptr();
        mem::forget( self );
        ptr
    }

    /// Return an obj's string representation. If the value's MUTF string representation
    /// is invalid, the string representation is regenerated from the value's internal
    /// representation.
    pub fn get_string( &self ) -> String {
        let mut len: c_int = 0;
        unsafe {
            let data = clib::Tcl_GetStringFromObj( self.as_ptr(), &mut len as *mut _ ) as *const u8;
            let slice = slice::from_raw_parts( data, len as usize );
            let m = mstr::from_mutf8_unchecked( slice );
            m.to_utf8().into_owned()
        }
    }

    /// Marks an obj's string representation invalid and to free any storage associated with the old string representation.
    pub fn invalidate_string_rep( &self ) {
        unsafe{ clib::Tcl_InvalidateStringRep( self.0.as_ptr() ); }
    }

    /// Test if `tcl::Obj` has no value.
    /// A newly created `Obj` is guaranteed to be dummy.
    /// An empty string is considered to be dummy.
    ///
    /// # Example
    ///
    /// ```
    /// assert!( tcl::Obj::new().is_empty() );
    /// assert!( tcl::Obj::from("").is_empty() );
    /// assert!( !tcl::Obj::from("hello").is_empty() );
    /// ```
    pub fn is_empty( &self ) -> bool {
        unsafe {
            let tcl_obj = &*(self.as_ptr() as *const clib::Tcl_Obj);
            let mut len: c_int = tcl_obj.length;

            if tcl_obj.bytes.is_null() {
                let _ = clib::Tcl_GetStringFromObj( self.as_ptr(), &mut len as *mut _ ) as *const u8;
            }

            len == 0
        }
    }

    /// Checks if the reference count of this obj is greater than 1.
    pub fn is_shared( &self ) -> bool {
        unsafe{ self.0.as_ref().refCount > 1 }
    }

    /// Clones the underlying value of this obj.
    pub fn clone_value( &self ) -> Option<Obj> {
        unsafe{ *self.type_ptr() }
            .dupIntRepProc
            .map( |dup_int_rep_proc| {
                let obj = Obj::new();
                unsafe{ dup_int_rep_proc( self.as_ptr(), obj.as_ptr() ); }
                obj
            })
    }

    pub(crate) fn type_ptr( &self ) -> *const clib::Tcl_ObjType {
        unsafe{ self.0.as_ref().typePtr }
    }

    pub(crate) fn value_ptr( &self ) -> *mut c_void {
        unsafe{ self.0.as_ref().internalRep.twoPtrValue.ptr1 }
    }

    pub(crate) fn free_internal_rep( &self ) {
        NonNull::new( self.type_ptr() as *mut clib::Tcl_ObjType )
            .map( |ptr| unsafe {
                ptr.as_ref().freeIntRepProc
                    .map( |free| free( self.as_ptr() ))
            });
    }
}

impl fmt::Display for Obj {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        f.write_str( &self.get_string() )
    }
}

impl From<bool> for Obj {
    fn from( b: bool ) -> Obj {
        crate::init();
        unsafe {
            Obj::from_raw( clib::Tcl_NewIntObj( if b {1} else {0} ))
        }
    }
}

impl TryFrom<Obj> for bool {
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        let mut value: c_int = 0;
        unsafe {
            clib::Tcl_GetBooleanFromObj( null_mut(), obj.as_ptr(), &mut value )
                .unit_result()
                .map( |_| value == 1 )
                .map_err( |_| DeError::new( DeKind::NotBool, obj ))
        }
    }
}

impl Obj {
    /// Returns a bool value of this obj.
    /// The values 0, "0", false, "false"( case insensitive ) is considered as false.
    /// The values 1, "1", true, "true"( case insensitive ) is considered as true.
    /// Other values will panic.
    pub fn as_bool( &self ) -> bool {
        let mut value: c_int = 0;
        unsafe {
            clib::Tcl_GetBooleanFromObj( null_mut(), self.as_ptr(), &mut value )
                .unit_result()
                .map( |_| value == 1 )
                .unwrap()
        }
    }
}

macro_rules! obj_from_int {
    ($tcl_api:ident $ty:ty => $c_ty:ty) => {
        impl From<$ty> for Obj {
            fn from( i: $ty ) -> Obj {
                crate::init();
                unsafe {
                    Obj::from_raw( clib::$tcl_api( i as $c_ty ))
                }
            }
        }
    };
}

obj_from_int!( Tcl_NewIntObj      i8   => c_int      );
obj_from_int!( Tcl_NewIntObj      u8   => c_int      );
obj_from_int!( Tcl_NewIntObj     i16   => c_int      );
obj_from_int!( Tcl_NewIntObj     u16   => c_int      );
obj_from_int!( Tcl_NewIntObj     i32   => c_int      );
obj_from_int!( Tcl_NewWideIntObj u32   => c_longlong );
obj_from_int!( Tcl_NewWideIntObj i64   => c_longlong );
obj_from_int!( Tcl_NewWideIntObj isize => c_longlong );

macro_rules! int_from_obj {
    ($tcl_api:ident $c_ty:ty => $ty:ident $as_method:ident $error:ident ) => {
        impl TryFrom<Obj> for $ty {
            type Error = DeError;

            fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
                let mut value: $c_ty = 0;
                if unsafe{ clib::$tcl_api( null_mut(), obj.as_ptr(), &mut value )}.is_ok() {
                    if (std::$ty::MIN as $c_ty ..= std::$ty::MAX as $c_ty).contains( &value ) {
                        return Ok( value as $ty );
                    }
                }
                Err( DeError::new( DeKind::$error, obj ))
            }
        }

        impl Obj {
            pub fn $as_method( &self ) -> $ty {
                let mut value: $c_ty = 0;
                if unsafe{ clib::$tcl_api( null_mut(), self.as_ptr(), &mut value )}.is_ok() {
                    if (std::$ty::MIN as $c_ty ..= std::$ty::MAX as $c_ty).contains( &value ) {
                        return value as $ty;
                    }
                }
                panic!( "\"{}\" cannot be converted to {}", self.to_string(), stringify!( $ty ));
            }
        }
    };
}

int_from_obj!( Tcl_GetIntFromObj     c_int      =>  i8   as_i8    NotI8    );
int_from_obj!( Tcl_GetLongFromObj    c_long     => i16   as_i16   NotI16   );
int_from_obj!( Tcl_GetWideIntFromObj c_longlong => i32   as_i32   NotI32   );
int_from_obj!( Tcl_GetWideIntFromObj c_longlong => i64   as_i64   NotI64   );
int_from_obj!( Tcl_GetWideIntFromObj c_longlong => isize as_isize NotISize );

macro_rules! uint_from_obj {
    ($tcl_api:ident $c_ty:ty => $ty:ident $as_method:ident $error:ident ) => {
        impl TryFrom<Obj> for $ty {
            type Error = DeError;

            fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
                let mut value: $c_ty = 0;
                if unsafe{ clib::$tcl_api( null_mut(), obj.as_ptr(), &mut value )}.is_ok() {
                    if (0..=std::$ty::MAX as $c_ty).contains( &value ) {
                        return Ok( value as $ty );
                    }
                }
                Err( DeError::new( DeKind::$error, obj ))
            }
        }

        impl Obj {
            pub fn $as_method( &self ) -> $ty {
                let mut value: $c_ty = 0;
                if unsafe{ clib::$tcl_api( null_mut(), self.as_ptr(), &mut value )}.is_ok() {
                    if (0..=std::$ty::MAX as $c_ty).contains( &value ) {
                        return value as $ty;
                    }
                }
                panic!( "\"{}\" cannot be converted to {}", self.to_string(), stringify!( $ty ));
            }
        }
    };
}

uint_from_obj!( Tcl_GetIntFromObj     c_int      =>  u8 as_u8  NotU8  );
uint_from_obj!( Tcl_GetLongFromObj    c_long     => u16 as_u16 NotU16 );
uint_from_obj!( Tcl_GetWideIntFromObj c_longlong => u32 as_u32 NotU32 );

#[cfg( target_pointer_width = "32" )]
uint_from_obj!( Tcl_GetWideIntFromObj c_longlong => usize as_usize NotUSize );

#[cfg( target_pointer_width = "64" )]
impl TryFrom<Obj> for usize {
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<usize, DeError> {
        u64::try_from( obj ).map( |v| v as usize )
    }
}

#[cfg( target_pointer_width = "64" )]
impl Obj {
    /// Returns a usize value of this obj.
    /// Values that is not a usize will panic.
    pub fn as_usize( &self ) -> usize {
        usize::try_from( self.clone() ).unwrap()
    }
}

#[cfg( target_pointer_width = "64" )]
impl Obj {
    /// Returns a u64 value of this obj.
    /// Values that is not a u64 will panic.
    pub fn as_u64( &self ) -> u64 {
        u64::try_from( self.clone() ).unwrap()
    }
}

#[cfg( any( target_pointer_width = "32", windows ))]
obj_from_int!( Tcl_NewWideIntObj usize => c_longlong );

#[cfg( all( target_pointer_width = "64", not(windows) ))]
impl From<usize> for Obj {
    fn from( v: usize ) -> Obj {
        crate::init();
        Obj::from( v as u64 )
    }
}

obj_from_int!( Tcl_NewDoubleObj f64 => c_double );

macro_rules! float_from_obj {
    ( $ty:ty, $error:ident, $as_method:ident ) => {
        impl TryFrom<Obj> for $ty {
            type Error = DeError;

            fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
                let mut value: c_double = 0.0;
                unsafe {
                    clib::Tcl_GetDoubleFromObj( null_mut(), obj.as_ptr(), &mut value )
                }
                .unit_result()
                .map( |_| value as $ty )
                .map_err( |_| DeError::new( DeKind::$error, obj ))
            }
        }

        impl Obj {
            pub fn $as_method( &self ) -> $ty {
                let mut value: c_double = 0.0;
                unsafe {
                    clib::Tcl_GetDoubleFromObj( null_mut(), self.as_ptr(), &mut value )
                }
                .unit_result()
                .map( |_| value as $ty )
                .unwrap()
            }
        }
    };
}

float_from_obj!( f32, NotF32, as_f32 );
float_from_obj!( f64, NotF64, as_f64 );

impl From<char> for Obj {
    fn from( c: char ) -> Obj {
        crate::init();
        c.to_string().into()
    }
}

impl TryFrom<Obj> for char {
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        let s = obj.to_string();
        let mut chars = s.chars();
        if let Some( ch ) = chars.next() {
            if chars.next().is_none() {
                return Ok( ch );
            }
        }
        Err( DeError::new( DeKind::NotChar, obj.clone() ))
    }
}

impl<'a> From<&'a str> for Obj {
    fn from( s: &'a str ) -> Obj {
        crate::init();
        let cow = mstr::from_utf8( s.as_bytes() );
        unsafe{ Obj::from_raw( clib::Tcl_NewStringObj( cow.as_ptr() as *const c_char, cow.len() as c_int ))}
    }
}

impl From<String> for Obj {
    fn from( s: String ) -> Obj {
        crate::init();
        let cow = mstr::from_utf8( s.as_bytes() );
        unsafe{ Obj::from_raw( clib::Tcl_NewStringObj( cow.as_ptr() as *const c_char, cow.len() as c_int ))}
    }
}

impl TryFrom<Obj> for String {
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        Ok( obj.to_string() )
    }
}

impl From<CString> for Obj {
    fn from( s: CString ) -> Obj {
        crate::init();
        let cow = mstr::from_utf8( s.as_bytes() );
        unsafe{ Obj::from_raw( clib::Tcl_NewStringObj( cow.as_ptr() as *const c_char, cow.len() as c_int ))}
    }
}

impl<'a,B> From<Cow<'a,B>> for Obj
    where B: 'a + ToOwned + ?Sized
        , &'a B: Into<Obj>
        , <B as ToOwned>::Owned: Into<Obj>
{
    fn from( cow: Cow<'a,B> ) -> Obj {
        match cow {
            Cow::Borrowed( borrowed ) => borrowed.into(),
            Cow::Owned( owned ) => owned.into(),
        }
    }
}

impl From<Obj> for PathBuf {
    fn from( obj: Obj ) -> Self {
        PathBuf::from( obj.to_string() )
    }
}

impl From<PathBuf> for Obj {
    fn from( s: PathBuf ) -> Obj {
        s.display().to_string().into()
    }
}

impl<T> From<Option<T>> for Obj
    where T: Into<Obj>
{
    fn from( option: Option<T> ) -> Obj {
        crate::init();
        match option {
            Some( value ) => (value,).into(),
            None => Obj::new(),
        }
    }
}

impl<T,O> From<&[T]> for Obj
    where T : ToOwned<Owned=O>
        , O : Into<Obj>
{
    fn from( v: &[T] ) -> Obj {
        crate::init();
        let v: Box<[*mut clib::Tcl_Obj]> = v.iter().map( |e| e.to_owned().into().into_raw() ).collect();
        let list = unsafe{ clib::Tcl_NewListObj( v.len() as c_int, v.as_ptr() as *const *mut clib::Tcl_Obj )};
        mem::forget( v );
        unsafe{ Obj::from_raw( list )}
    }
}

impl<T> From<Vec<T>> for Obj
    where T: Into<Obj>
{
    fn from( v: Vec<T> ) -> Obj {
        crate::init();
        let v: Box<[*mut clib::Tcl_Obj]> = v.into_iter().map( |e| e.into().into_raw() ).collect();
        let list = unsafe{ clib::Tcl_NewListObj( v.len() as c_int, v.as_ptr() as *const *mut clib::Tcl_Obj )};
        mem::forget( v );
        unsafe{ Obj::from_raw( list )}
    }
}

impl<T> TryFrom<Obj> for Vec<T>
    where Obj: TryInto<T,Error=DeError>
{
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        let list = obj.clone();
        list.get_elements()
            .map_err( |_| DeError::new( DeKind::NotList, obj.clone() ))?
            .enumerate()
            .try_fold( Vec::new(), |mut acc, (nth,elem)| {
                match elem.try_into() {
                    Ok( elem ) => acc.push( elem ),
                    Err( err ) => return Err( DeError::chain( DeKind::List{ err_idx: nth }, obj.clone(), err )),
                }
                Ok( acc )
            })
    }
}

impl<K,V> From<HashMap<K,V>> for Obj
    where K: Into<Obj>
        , V: Into<Obj>
{
    fn from( hash_map: HashMap<K,V> ) -> Obj {
        crate::init();
        let dict = Obj::new_dict();
        for (k,v) in hash_map.into_iter() {
            dict.dict_put( k, v ).unwrap();
        }
        dict
    }
}

impl<K,V> TryFrom<Obj> for HashMap<K,V>
    where Obj : TryInto<K,Error=DeError>
        , Obj : TryInto<V,Error=DeError>
        , K   : Hash + Eq
{
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        let dict = obj.clone();
        dict.dict_iter()?
            .try_fold( HashMap::new(), |mut acc, (key, val)| {
                let key = key.try_into().map_err( |e| DeError::chain( DeKind::DictBadKey, obj.clone(), e ))?;
                let val = val.try_into().map_err( |e| DeError::chain( DeKind::DictBadVal, obj.clone(), e ))?;
                acc.insert( key, val );
                Ok( acc )
            })
    }
}

impl<T> From<Range<T>> for Obj
    where T: Into<Obj>
{
    fn from( range: Range<T> ) -> Obj {
        crate::init();
        (range.start, range.end).into()
    }
}

impl<T> TryFrom<Obj> for Range<T>
    where Obj: TryInto<T,Error=DeError>
{
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
        <(T,T)>::try_from( obj ).map( |v| Range{ start: v.0, end: v.1 })
    }
}

impl<Tup,HTup,MTup> From<Tup> for Obj
    where Tup : IntoHomoTuple<Obj, Output=HTup>
        , HTup: MapHomoTuple<Obj, *mut clib::Tcl_Obj, Output=MTup>
        , MTup: HomoTuple<*mut clib::Tcl_Obj>
{
    fn from( tuple: Tup ) -> Obj {
        crate::init();
        let array = tuple
            .into_homo_tuple()
            .map_homo_tuple( |obj| obj.into_raw() )
            .into_array();

        let objc = array.len() as c_int;
        if objc == 0 {
            Obj::new()
        } else {
            let objv = &array as *const _ as *const *mut clib::Tcl_Obj;
            unsafe {
                Obj::from_raw( clib::Tcl_NewListObj( objc, objv ))
            }
        }
    }
}

macro_rules! tuple_from_obj {
    ($($degree:expr => ($($t:ident $n:expr)* ))* ) => {$(
        impl<$($t,)*> TryFrom<Obj> for ($($t,)*)
            where $( Obj: TryInto<$t,Error=DeError> ),*
        {
            type Error = DeError;

            #[allow( non_snake_case )]
            fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
                let list = obj.clone();
                let elems = list.get_elements()?.collect::<Vec<Obj>>();
                let elem_cnt = elems.len();
                if elem_cnt != $degree {
                    return Err( DeError::new( DeKind::ListLen{ expected: $degree, got: elem_cnt }, obj.clone() ));
                }
                let mut elems = elems.into_iter();
                $(
                    let $t = match elems.next().unwrap().try_into() {
                        Ok( elem ) => elem,
                        Err( err ) => return Err( DeError::chain( DeKind::List{ err_idx: $n }, obj.clone(), err )),
                    };
                )*
                Ok(( $($t,)* ))
            }
        }
    )*};
}

tuple_from_obj!{
     1 => (T0 0)
     2 => (T0 0 T1 1)
     3 => (T0 0 T1 1 T2 2)
     4 => (T0 0 T1 1 T2 2 T3 3)
     5 => (T0 0 T1 1 T2 2 T3 3 T4 4)
     6 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5)
     7 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6)
     8 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7)
     9 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8)
    10 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9)
    11 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10)
    12 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11)
    13 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12)
    14 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13)
    15 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14)
    16 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15)
    17 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16)
    18 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17)
    19 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18)
    20 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19)
    21 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20)
    22 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21)
    23 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22)
    24 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23)
    25 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24)
    26 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25)
    27 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25 T26 26)
    28 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25 T26 26 T27 27)
    29 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25 T26 26 T27 27 T28 28)
    30 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25 T26 26 T27 27 T28 28 T29 29)
    31 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25 T26 26 T27 27 T28 28 T29 29 T30 30)
    32 => (T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 T13 13 T14 14 T15 15 T16 16 T17 17 T18 18 T19 19 T20 20 T21 21 T22 22 T23 23 T24 24 T25 25 T26 26 T27 27 T28 28 T29 29 T30 30 T31 31)
}

/// Increase the reference count of the Tcl obj which the `tcl_obj` points to.
///
/// # Safety
///
/// The `tcl_obj` pointer should be valid Tcl obj.
pub unsafe fn incr_ref( tcl_obj: *mut clib::Tcl_Obj ) {
    (*tcl_obj).refCount += 1;
    if (*tcl_obj).refCount == 1 {
        let mut len: c_int = 0;
        let _s = {
            let data = clib::Tcl_GetStringFromObj( tcl_obj, &mut len ) as *const u8;
            let slice = slice::from_raw_parts( data, len as usize );
            let m = mstr::from_mutf8_unchecked( slice );
            //let m = unsafe{ mstr::from_mutf8_unchecked( slice::from_raw_parts( data, len as usize ))};
            m.to_utf8().into_owned()
        };
    }
}

/// Decrease the reference count of the Tcl obj which the `tcl_obj` points to.
///
/// # Safety
///
/// The `tcl_obj` pointer should be valid Tcl obj.
pub unsafe fn decr_ref( tcl_obj: *mut clib::Tcl_Obj ) {
    (*tcl_obj).refCount -= 1;
    if (*tcl_obj).refCount <= 0 {

        let mut len: c_int = 0;
        let _s = {
            let data = clib::Tcl_GetStringFromObj( tcl_obj, &mut len ) as *const u8;
            let slice = slice::from_raw_parts( data, len as usize );
            let m = mstr::from_mutf8_unchecked( slice );
            m.to_utf8().into_owned()
        };

        clib::TclFreeObj( tcl_obj );
    }
}
