#![doc( hidden )]

use crate::{
    CodeToResult,
    Obj,
    error::{
        MutateSharedDict,
        NotDict,
    },
    obj::incr_ref,
};

use std::{
    os::raw::c_int,
    ptr::{self, NonNull, null_mut},
};

use enumx::export::*;
use enumx::predefined::*;
use cex::*;

impl Obj {
    /// Creates a new, empty dictionary value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    ///
    /// let dict = Obj::new_dict();
    /// assert_eq!( dict.to_string(), "" );
    /// ```
    pub fn new_dict() -> Obj {
        super::init();

        let tcl_obj = unsafe{ clib::Tcl_NewDictObj() };
        unsafe{ incr_ref( tcl_obj ); }
        Obj( NonNull::new( tcl_obj ).expect( "Tcl_NewObj should return non null ptr" ))
    }

    /// Looks up the given key within the given dictionary and returns the value associated with that key,
    /// or `None` if the key has no mapping within the dictionary.
    /// An error of `NotDict` occurs if it cannot be converted to a dictionary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use tcl::*;
    ///
    /// let mut map = HashMap::new();
    /// map.insert( "C"   , 1972 );
    /// map.insert( "C++" , 1983 );
    /// map.insert( "Rust", 2006 );
    ///
    /// let map = Obj::from( map );
    /// assert!( map.dict_get("Cpp").unwrap().is_none() );
    /// assert_eq!( map.dict_get("Rust").unwrap().unwrap().as_i32(), 2006 );
    /// ```
    pub fn dict_get( &self, key: impl Into<Obj> ) -> Result<Option<Obj>, NotDict> {
        let key: Obj = key.into();
        let mut value = ptr::null_mut::<clib::Tcl_Obj>();

        unsafe {
            clib::Tcl_DictObjGet(
                null_mut(),
                self.as_ptr(),
                key.as_ptr(),
                &mut value
        )}
        .unit_result()
        .map( |_| if value.is_null() {
            None
        } else {
            Some( unsafe{ Obj::from_raw( value )})
        })
        .map_err( |_| NotDict( self.clone() ))
    }

    /// Updates the given dictionary so that the given key maps to the given value;
    /// any key may exist at most once in any particular dictionary.
    /// The dictionary must not be shared, or an error of `MutateSharedDict` will occur.
    /// An error of `NotDict` occurs if it cannot be converted to a dictionary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use tcl::*;
    ///
    /// let dict = Obj::new_dict();
    /// assert_eq!( dict.to_string(), "" );
    ///
    /// dict.dict_put( "Rust", 2006 );
    /// assert_eq!( dict.to_string(), "Rust 2006" );
    /// ```
    #[cex]
    pub fn dict_put( &self, key: impl Into<Obj>, value: impl Into<Obj> ) -> Result!( () throws MutateSharedDict, NotDict ) {
        if self.is_shared() {
            throw!( MutateSharedDict( self.clone() ));
        }

        let key  : Obj = key  .into();
        let value: Obj = value.into();

        ret!(
            unsafe {
                clib::Tcl_DictObjPut(
                    null_mut(),
                    self.as_ptr(),
                    key.as_ptr(),
                    value.as_ptr(),
            )}
            .unit_result()
            .map_err( |_| NotDict( self.clone() ))
        );
    }

    /// Updates the given dictionary so that the given key has no mapping to any value.
    /// The dictionary must not be shared, or an error of `MutateSharedDict` will occur.
    /// It is not an error if the key did not previously exist.
    /// An error of `NotDict` occurs if it cannot be converted to a dictionary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use tcl::*;
    ///
    /// let mut map = HashMap::new();
    /// map.insert( "C++" , 1983 );
    /// map.insert( "Rust", 2006 );
    ///
    /// let map = Obj::from( map );
    /// map.dict_remove("Cpp").unwrap();
    /// map.dict_remove("C++").unwrap();
    /// assert_eq!( map.to_string(), "Rust 2006" );
    /// ```
    #[cex]
    pub fn dict_remove( &self, key: impl Into<Obj> ) -> Result!( () throws MutateSharedDict, NotDict ) {
        if self.is_shared() {
            throw!( MutateSharedDict( self.clone() ));
        }

        let key: Obj = key.into();

        ret!(
            unsafe {
                clib::Tcl_DictObjRemove(
                    null_mut(),
                    self.as_ptr(),
                    key.as_ptr(),
            )}
            .unit_result()
            .map_err( |_| NotDict( self.clone() ))
        );
    }

    /// Updates the given variable with the number of key/value pairs currently in the given dictionary.
    /// An error of `NotDict` occurs if it cannot be converted to a dictionary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use tcl::*;
    ///
    /// let mut map = HashMap::new();
    /// map.insert( "C++" , 1983 );
    /// map.insert( "Rust", 2006 );
    /// let map = Obj::from( map );
    /// assert_eq!( map.dict_size().unwrap(), 2 );
    ///
    /// let obj = vec![ "C++", "1983", "Rust", "2006" ];
    /// assert_eq!( map.dict_size().unwrap(), 2 );
    /// ```
    pub fn dict_size( &self ) -> Result<c_int, NotDict> {
        let mut size: c_int = 0;
        unsafe {
            clib::Tcl_DictObjSize(
                null_mut(),
                self.as_ptr(),
                &mut size as *mut c_int,
        )}
        .unit_result()
        .map( |_| size )
        .map_err( |_| NotDict( self.clone() ))
    }

    ///  Creates an iterator from a dictionary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use tcl::*;
    ///
    /// let mut map = HashMap::new();
    /// map.insert( "C"   , 1972 );
    /// map.insert( "C++" , 1983 );
    /// map.insert( "Rust", 2006 );
    /// let map = Obj::from( map );
    /// let mut list = map
    ///     .dict_iter()
    ///     .unwrap()
    ///     .map( |(k,v)| (k.to_string(), v.as_i32() ))
    ///     .collect::<Vec<_>>();
    /// list.sort();
    /// assert_eq!( list, vec![ ("C".to_owned(),1972), ("C++".to_owned(),1983), ("Rust".to_owned(),2006) ]);
    /// ```
    pub fn dict_iter( self ) -> Result<DictIter, NotDict> {
        let remains = self.dict_size()? as usize;

        Ok( DictIter {
            dict    : self,
            search  : None,
            remains ,
        })
    }
}

/// An iterator that iterates each key-value pair.
pub struct DictIter {
    dict    : Obj,
    search  : Option<clib::Tcl_DictSearch>,
    remains : usize,
}

impl Iterator for DictIter {
    type Item = (Obj, Obj);

    fn next( &mut self ) -> Option<Self::Item> {
        let mut key   = ptr::null_mut::<clib::Tcl_Obj>();
        let mut value = ptr::null_mut::<clib::Tcl_Obj>();
        let mut done: c_int = 0;

        let search = match self.search.take() {
            Some( mut search ) => unsafe {
                clib::Tcl_DictObjNext( &mut search, &mut key, &mut value, &mut done );
                search
            },
            None => unsafe {
                let mut new_search = std::mem::MaybeUninit::<clib::Tcl_DictSearch>::zeroed().assume_init();

                clib::Tcl_DictObjFirst(
                    null_mut(),
                    self.dict.as_ptr(),
                    &mut new_search,
                    &mut key,
                    &mut value,
                    &mut done
                )
                .unit_result()
                .ok()?;

                new_search
            },
        };

        self.search = Some( search );

        if done == 0 {
            self.remains -= 1;
            unsafe{ Some(( Obj::from_raw( key ), Obj::from_raw( value )))}
        } else {
            None
        }
    }

    fn size_hint( &self ) -> (usize, Option<usize>) {
        (self.remains, Some( self.remains ))
    }
}

#[cfg( test )]
mod tests {
    use super::*;

    #[test]
    fn iteration() {
        let dict = Obj::new_dict();
        dict.dict_put( "hello", "world" ).unwrap();
        dict.dict_put( "answer", "42" ).unwrap();
        assert_eq!(
            dict.dict_iter()
                .unwrap()
                .map( |(k,v)| (k.get_string(), v.get_string() ))
                .collect::<Vec<_>>(),
            vec![
                ("hello".to_owned(), "world".to_owned() ),
                ("answer".to_owned(), "42".to_owned() )]
        );
    }
}
