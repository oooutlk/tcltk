//! Provides an extention to Tcl values: `Tcl<T>`.

use serde::{
    Serialize,
    de::DeserializeOwned,
};

use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    Obj,
    UnwrapOrAbort,
    error::{
        DeError,
        DeKind,
        MismatchedObjType,
        MoveBorrowedValue,
        MoveSharedObj,
        NullDataPtr,
    },
};

use std::{
    any::TypeId,
    cell::RefCell,
    convert::TryFrom,
    ffi::{CStr, CString},
    hash::{Hash, Hasher},
    mem,
    ops::Deref,
    os::raw::{c_void, c_int},
    ptr::{self, NonNull, null_mut},
};

/// Registers a new Tcl value type.
pub trait Register {
    fn register();
}

/// A wrapper for storing a value of type `T` in Tcl `Obj`.
///
/// In a `#[proc] fn`, arguments of type `&T`/`&mut T` are implemented in (mutably) borrowing `Tcl<T>`.
#[derive( Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord )]
pub struct Tcl<T:Clone>( RefCell<T> );

impl<T:Clone+Hash> Hash for Tcl<T> {
    fn hash<H: Hasher>( &self, state: &mut H ) {
        self.0.borrow().hash( state );
    }
}

impl<T:Clone> Deref for Tcl<T> {
    type Target = RefCell<T>;
    fn deref( &self ) -> &Self::Target { &self.0 }
}

impl<T> TryFrom<Obj> for Tcl<T>
    where T: 'static + Serialize + DeserializeOwned + Clone
{
    type Error = Throws!( DeError, MoveBorrowedValue, MoveSharedObj, NullDataPtr );

    #[cex]
    fn try_from( obj: Obj ) -> Result!( Self throws DeError, MoveBorrowedValue, MoveSharedObj, NullDataPtr ) {
        if obj.is_shared() {
            throw!( MoveSharedObj( obj ));
        }

        match NonNull::new( obj.value_ptr() as *mut Tcl<T> ) {
            Some( ptr ) => {
                if unsafe{ ptr.as_ref() }.0.try_borrow_mut().is_err() {
                    throw!( MoveBorrowedValue( obj ));
                }

                let type_ptr = register_or_get_tcl_obj_type::<Self>();
                if obj.type_ptr() == type_ptr {
                    unsafe{ (*obj.as_ptr()).internalRep.twoPtrValue.ptr1 = null_mut(); }
                    Ok( unsafe{ *Box::from_raw( ptr.as_ptr() )})
                } else {
                    match crate::from_obj::<T>( obj.clone() ) {
                        Ok( value ) => {
                            unsafe{ *obj.as_ptr() }.typePtr = type_ptr;
                            Ok( Tcl( RefCell::new( value )))
                        },
                        Err( err ) => throw!( err ),
                    }
                }
            },
            None => throw!( NullDataPtr( obj )),
        }
    }
}

impl<T> Tcl<T>
    where T: 'static + Serialize + DeserializeOwned + Clone
{
    #[doc( hidden )]
    #[cex]
    pub fn ptr_from_obj( obj: Obj ) -> Result!( NonNull<Tcl<T>> throws DeError, NullDataPtr ) {
        let type_ptr = register_or_get_tcl_obj_type::<Self>();
        if obj.type_ptr() == type_ptr {
            match NonNull::new( obj.value_ptr() as *mut Tcl<T> ) {
                Some( ptr ) => Ok( ptr ),
                None        => throw!( NullDataPtr( obj.clone() )),
            }
        } else {
            let inner: T = crate::from_obj( obj.clone() )?;
            // store inner value in obj and convert its type
            unsafe {
                if let Some( free ) = (*obj.type_ptr()).freeIntRepProc {
                    free( obj.as_ptr() )
                }

                let non_null = Tcl::<T>::non_null_ptr_from( inner );
                (*obj.0.as_ptr()).typePtr = type_ptr;
                (*obj.0.as_ptr()).internalRep.twoPtrValue.ptr1 = non_null.as_ptr() as *mut c_void;
                Ok( non_null )
            }
        }
    }

    fn non_null_ptr_from( value: T ) -> NonNull<Tcl<T>> {
        Self::non_null_ptr_from_ref_cell( RefCell::new( value ))
    }

    fn non_null_ptr_from_ref_cell( ref_cell: RefCell<T> ) -> NonNull<Tcl<T>> {
        unsafe{ NonNull::new_unchecked( Box::into_raw( Box::new( Tcl( ref_cell ))))}
    }
}

impl<T> Register for Tcl<T>
    where T: 'static + Serialize + DeserializeOwned + Clone
{
    fn register() {
        let c_string = type_id_to_c_string::<Self>();
        let name = c_string.as_ptr();
        mem::forget( c_string );

        unsafe {
            clib::Tcl_RegisterObjType( Box::into_raw( Box::new( clib::Tcl_ObjType{
                name            ,
                freeIntRepProc  : Some( Tcl::<T>::free_internal_rep ),
                dupIntRepProc   : Some( Tcl::<T>::dup_internal_rep  ),
                updateStringProc: Some( Tcl::<T>::update_string     ),
                setFromAnyProc  : Some( Tcl::<T>::set_from_any      ),
            })));
        }
    }
}

impl<T> Tcl<T>
    where T: 'static + Serialize + DeserializeOwned + Clone
{
    /// Creates a boxed value of type `T`.
    pub fn new( value: T ) -> Box<Self> { Box::new( Tcl( RefCell::new( value )))}

    /// Creates a value of erased type `T`, stored in a Tcl `Obj`.
    pub fn new_obj( value: T ) -> Obj {
        crate::init();

        let type_ptr = register_or_get_tcl_obj_type::<Self>();
        let mut obj = Obj::new();
        unsafe {
            obj.0.as_mut().typePtr = type_ptr;
            obj.0.as_mut().internalRep.twoPtrValue.ptr1 = Box::into_raw( Self::new( value )) as *mut c_void;
        }
        obj
    }

    /// Consumes the `Tcl`, returning the wrapped value.
    pub fn into_inner( self ) -> T {
        self.0.into_inner()
    }

    #[doc( hidden )]
    pub fn box_into_c_void_ptr( self ) -> *mut c_void { Box::into_raw( Box::new( self )) as *mut c_void }

    #[cex]
    unsafe fn non_null_ptr_cast_from( obj: *mut clib::Tcl_Obj ) -> Result!( NonNull<Self> throws MismatchedObjType, NullDataPtr ) {
        if register_or_get_tcl_obj_type::<Self>() as *const _ == (*obj).typePtr {
            match NonNull::new( (*obj).internalRep.twoPtrValue.ptr1 as *mut Tcl<T> ) {
                Some( ptr ) => Ok( ptr ),
                None => throw!( NullDataPtr( Obj::from_raw( obj ))),
            }
        } else {
            throw!( MismatchedObjType )
        }
    }

    unsafe extern "C" fn free_internal_rep( obj_ptr: *mut clib::Tcl_Obj ) {
        let ptr = (*obj_ptr).internalRep.twoPtrValue.ptr1 as *mut Tcl<T>;
        if !ptr.is_null() {
            if (*ptr).0.try_borrow_mut().is_err() {
                Result::<(),_>::Err( MoveBorrowedValue( Obj::from_raw( obj_ptr ))).unwrap_or_abort("Trying to free borrowed value.");
            }

            ptr::drop_in_place( Tcl::<T>::non_null_ptr_cast_from( obj_ptr )
                .unwrap_or_abort("Bad cast of tcl::Tcl in free_internal_rep.")
                .as_ptr() );
            (*obj_ptr).internalRep.twoPtrValue.ptr1 = null_mut();
            (*obj_ptr).typePtr = null_mut();
        }
    }

    unsafe extern "C" fn dup_internal_rep( src: *mut clib::Tcl_Obj, dup: *mut clib::Tcl_Obj ) {
        (*dup).typePtr = (*src).typePtr;
        let value_ptr = (*src).internalRep.twoPtrValue.ptr1;
        (*dup).internalRep.twoPtrValue.ptr1 = Tcl::<T>::box_into_c_void_ptr( (*( value_ptr as *mut Tcl<T> )).clone() );
    }

    unsafe extern "C" fn update_string( obj_ptr: *mut clib::Tcl_Obj ) {
        let value_ptr = Tcl::<T>::non_null_ptr_cast_from( obj_ptr )
            .unwrap_or_abort("Bad cast of tcl::Tcl in update_string.");
        let (len,c_str) = crate::to_c_str( &value_ptr.as_ref().0 )
            .unwrap_or_abort("Failed in converting from tcl::Tcl to Tcl DString in update_string.");
        if (*obj_ptr).internalRep.twoPtrValue.ptr2 == (*obj_ptr).bytes as *mut _ {
            clib::Tcl_InvalidateStringRep( obj_ptr );
        }
        (*obj_ptr).length = len;
        (*obj_ptr).bytes = c_str as *mut _;
        (*obj_ptr).internalRep.twoPtrValue.ptr2 = c_str as *mut _;
    }

    unsafe extern "C" fn set_from_any( interp: *mut clib::Tcl_Interp, obj_ptr: *mut clib::Tcl_Obj ) -> c_int {
        let obj = Obj::from_raw( obj_ptr );
        let value = match crate::from_obj::<T>( obj.clone() ) {
            Ok( value ) => Tcl::new( value ),
            Err( _ ) => {
                clib::Tcl_SetObjResult( interp, Obj::from( "set_from_any() failed" ).into_raw() );
                let obj = Obj::from(( "TCL_RS", "WRAPPED_RS_VALUE", "SET_FROM_ANY", "DESER" ));
                clib::Tcl_SetObjErrorCode( interp, obj.into_raw() );
                return clib::TCL_ERROR as c_int;
            },
        };

        obj.free_internal_rep();
        let type_id = type_id_to_c_string::<Tcl<T>>();
        (*obj_ptr).typePtr = clib::Tcl_GetObjType( type_id.as_ptr() );
        (*obj_ptr).internalRep.twoPtrValue.ptr1 = value.box_into_c_void_ptr();

        clib::TCL_OK as c_int
    }
}

fn type_id_to_c_string<T:'static>() -> CString {
    CString::from(
        CStr::from_bytes_with_nul( format!( "{:?}\0", TypeId::of::<T>() ).as_bytes() )
        .expect("TypeId should not contain any interrior nul.")
    )
}

#[doc( hidden )]
pub fn register_or_get_tcl_obj_type<T:'static+Register>() -> *mut clib::Tcl_ObjType {
    let type_id = type_id_to_c_string::<T>();
    let mut type_ptr = unsafe{ clib::Tcl_GetObjType( type_id.as_ptr() )};
    if type_ptr.is_null() {
        <T as Register>::register();
        type_ptr = unsafe{ clib::Tcl_GetObjType( type_id.as_ptr() )};
    }
    type_ptr as *mut _
}

impl From<u64> for Obj {
    fn from( v: u64 ) -> Obj {
        let mut obj = Obj::new();
        unsafe {
            let obj_ptr = obj.0.as_mut();
            (*obj_ptr).typePtr = register_or_get_tcl_obj_type::<u64>();
            (*obj_ptr).internalRep.twoPtrValue.ptr1 = v as *mut c_void;
            u64_update_string( obj_ptr );
        }
        obj
    }
}

impl TryFrom<Obj> for u64 {
    type Error = DeError;

    fn try_from( obj: Obj ) -> Result<u64, DeError> {
        let type_ptr = register_or_get_tcl_obj_type::<Self>() as *const clib::Tcl_ObjType;
        unsafe {
            if (*obj.as_ptr()).typePtr == type_ptr {
                Ok( (*obj.as_ptr()).internalRep.twoPtrValue.ptr1 as u64 )
            } else {
                match obj.to_string().parse::<u64>() {
                    Ok( value ) => {
                        (*obj.as_ptr()).typePtr = type_ptr;
                        Ok( value )
                    },
                    Err( _ ) => Err( DeError::new( DeKind::NotU64, obj.clone() )),
                }
            }
        }
    }
}

impl Register for u64 {
    fn register() {
        let c_string = type_id_to_c_string::<Self>();
        let name = c_string.as_ptr();
        mem::forget( c_string );

        unsafe {
            clib::Tcl_RegisterObjType( Box::into_raw( Box::new( clib::Tcl_ObjType{
                name            ,
                freeIntRepProc  : None,
                dupIntRepProc   : Some( u64_dup_internal_rep ),
                updateStringProc: Some( u64_update_string    ),
                setFromAnyProc  : Some( u64_set_from_any     ),
            })));
        }
    }
}

unsafe extern "C" fn u64_dup_internal_rep( src: *mut clib::Tcl_Obj, dup: *mut clib::Tcl_Obj ) {
    (*dup).typePtr = (*src).typePtr;
    (*dup).internalRep.twoPtrValue.ptr1 = (*src).internalRep.twoPtrValue.ptr1;
}

unsafe extern "C" fn u64_update_string( obj_ptr: *mut clib::Tcl_Obj ) {
    let s = ( (*obj_ptr).internalRep.twoPtrValue.ptr1 as u64 ).to_string();
    let result = clib::Tcl_Alloc( s.len() as u32 + 1 );
    std::ptr::copy_nonoverlapping( s.as_ptr(), result as *mut u8, s.len() + 1 );

    (*obj_ptr).length =  s.len() as c_int;
    (*obj_ptr).bytes = result;
}

unsafe extern "C" fn u64_set_from_any( _interp: *mut clib::Tcl_Interp, obj_ptr: *mut clib::Tcl_Obj ) -> c_int {
    let data = (*obj_ptr).bytes as *const i8 as *const u8;
    let len = (*obj_ptr).length as usize;
    if let Ok(s) = std::str::from_utf8( std::slice::from_raw_parts( data, len )) {
        if let Ok( value ) = s.parse::<u64>() {
            let obj = Obj::from_raw( obj_ptr );
            obj.free_internal_rep();
            (*obj_ptr).internalRep.twoPtrValue.ptr1 = value as *mut c_void;
            return clib::TCL_OK as c_int;
        }
    }

    clib::TCL_ERROR as c_int
}

#[cfg( test )]
mod tests {
    #[test]
    fn test_u64() {
        let obj = crate::Obj::from( 42_u64 );
        assert_eq!( obj.get_string(), "42" );
    }
}
