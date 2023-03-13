#![doc( hidden )]

use crate::{
    CodeToResult,
    Obj,
    error::NotList,
};

use std::{
    os::raw::c_int,
    mem,
    ptr::null_mut,
    slice,
};

type Result<T, E=NotList> = std::result::Result<T,E>;

impl Obj {
    /// Creates a new value as a Tcl list obj to hold `objs`.
    pub fn new_list( objs: impl Iterator<Item=Obj> ) -> Obj {
        let objs: Box<[*mut clib::Tcl_Obj]> = objs.map( Obj::into_raw ).collect();
        let objc = objs.len() as c_int;
        let objv = objs.as_ptr() as *const *mut clib::Tcl_Obj;
        let list = unsafe{ clib::Tcl_NewListObj( objc, objv )};
        mem::forget( objs );
        unsafe{ Obj::from_raw( list )}
    }

    /// Creates a new, empty Tcl list obj with the specified capacity.
    pub fn new_list_with_capacity( cap: usize ) -> Obj {
        unsafe{ Obj::from_raw( clib::Tcl_NewListObj( cap as c_int, null_mut() ))}
    }

    /// Sets the obj to hold a Tcl list composed of `objs`.
    pub fn set_list( self, objs: impl Iterator<Item=Obj> ) -> Self {
        let objs: Box<[*mut clib::Tcl_Obj]> = objs.map( Obj::into_raw ).collect();
        let objc = objs.len() as c_int;
        let objv = objs.as_ptr() as *const *mut clib::Tcl_Obj;
        unsafe{ clib::Tcl_SetListObj( self.as_ptr(), objc, objv ); }
        mem::forget( objs );
        self
    }

    /// Append a list `to_append` to this list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let list = Obj::from(( "The", "answer" ));
    /// list.list_append(( "is", 42 ));
    /// assert_eq!( list.to_string(), "The answer is 42" );
    /// ```
    pub fn list_append( &self, to_append: impl Into<Obj> ) -> Result<()> {
        let to_append = to_append.into();
        unsafe{ clib::Tcl_ListObjAppendList( null_mut(), self.as_ptr(), to_append.as_ptr() )}
        .unit_result()
        .map_err( |_| NotList( self.clone() ))
    }

    /// Append an element `elem` to this list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let list = Obj::from(( "The", "answer" ));
    /// list.list_append_element(( "is", 42 ));
    /// assert_eq!( list.to_string(), "The answer {is 42}" );
    /// ```
    pub fn list_append_element( &self, elem: impl Into<Obj> ) -> Result<()> {
        let elem = elem.into();
        unsafe{ clib::Tcl_ListObjAppendElement( null_mut(), self.as_ptr(), elem.as_ptr() )}
        .unit_result()
        .map_err( |_| NotList( self.clone() ))
    }

    /// Converts the list to an iterator that iterates over all its elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let list = Obj::from(( "The", "answer", "is", 42 ));
    /// let mut elems = list.get_elements().unwrap();
    /// assert_eq!( elems.next().unwrap().to_string(), "The" );
    /// assert_eq!( elems.next().unwrap().to_string(), "answer" );
    /// assert_eq!( elems.next().unwrap().to_string(), "is" );
    /// assert_eq!( elems.next().unwrap().as_i32(), 42 );
    /// ```
    pub fn get_elements( self ) -> Result<impl Iterator<Item=Obj>> {
        let mut objc = 0;
        let mut objv = null_mut();
        let objs = {
            unsafe{ clib::Tcl_ListObjGetElements( null_mut(), self.as_ptr(), &mut objc, &mut objv )}
            .unit_result()
            .map_err( |_| NotList( self ))?;

            unsafe {
                slice::from_raw_parts( objv, objc as usize )
                    .iter()
                    .map( |tcl_obj| Obj::from_raw( *tcl_obj ))
                    .collect::<Vec<_>>()
            }
        };
        Ok( objs.into_iter() )
    }

    /// Returns the amount of list elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let list = Obj::from(( "The", "answer", "is", 42 ));
    /// assert_eq!( list.list_length().unwrap(), 4 );
    /// ```
    pub fn list_length( &self ) -> Result<c_int> {
        let mut len = 0;
        unsafe{ clib::Tcl_ListObjLength( null_mut(), self.as_ptr(), &mut len )}
        .unit_result()
        .map_err( |_| NotList( self.clone() ))?;
        Ok( len )
    }

    /// Returns the nth obj in the list.
    /// Note that the index is 0-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let list = Obj::from(( "The", "answer", "is", 42 ));
    /// assert_eq!( list.list_index(1).unwrap().unwrap().to_string(), "answer" );
    /// assert!( list.list_index(4).unwrap().is_none() );
    /// ```
    pub fn list_index( &self, index: c_int ) -> Result<Option<Obj>> {
        let mut obj_ptr = null_mut();
        unsafe{ clib::Tcl_ListObjIndex( null_mut(), self.as_ptr(), index, &mut obj_ptr )}
        .unit_result()
        .map_err( |_| NotList( self.clone() ))?;

        if obj_ptr.is_null() {
            Ok( None )
        } else {
            Ok( Some( unsafe{ Obj::from_raw( obj_ptr )}))
        }
    }

    /// Replace from the `first` to `first`+`count` objs in the list with `objs`.
    ///
    /// If the argument `first` is zero or negative, it refers to the first element;
    /// if it is greater than or equal to the number of elements in the list, then no elements are deleted;
    /// the new elements are appended to the list.
    ///
    /// If count is zero or negative then no elements are deleted;
    /// the new elements are simply inserted before the one designated by first.
    ///
    /// Note that the index is 0-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// let list = Obj::from(( "The", "answer", "is", 42 ));
    /// let objs = vec![ "to", "life,", "the", "universe,", "and", "everything:" ].into_iter().map( Obj::from );
    /// list.list_replace( 2, 1, objs ).unwrap();
    /// assert_eq!( list.to_string(), "The answer to life, the universe, and everything: 42" );
    /// ```
    pub fn list_replace( &self, first: c_int, count: c_int, objs: impl Iterator<Item=Obj> ) -> Result<()> {
        let objs: Box<[*mut clib::Tcl_Obj]> = objs.map( Obj::into_raw ).collect();
        let objc = objs.len() as c_int;
        let objv = objs.as_ptr() as *const *mut clib::Tcl_Obj;

        let result = unsafe{ clib::Tcl_ListObjReplace( null_mut(), self.as_ptr(), first, count, objc, objv )}
            .unit_result()
            .map_err( |_| NotList( self.clone() ));

        mem::forget( objs );
        result
    }
}
