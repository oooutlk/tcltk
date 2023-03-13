//! Serialize `Obj` into Tcl string.

use crate::error::SerError;

type Result<T> = std::result::Result<T, SerError>;

use clib::{
    Tcl_DString,
    Tcl_DStringInit,
    Tcl_DStringAppendElement,
    Tcl_DStringStartSublist,
    Tcl_DStringEndSublist,
    Tcl_DStringFree,
};

use mutf8::mstr;

use serde::{ser, Serialize};

use std::{
    ffi::CString,
    fmt::{self, Display},
    mem::MaybeUninit,
    ops::AddAssign,
    os::raw::{c_char, c_int},
    pin::Pin,
    slice,
};

/// A Wrapper of `Tcl_DString` structure, which represents a dynamic string in Tcl.
#[repr( transparent )]
pub struct DString( Pin<Box<Tcl_DString>> );

fn dstring_to_string( dstring: &DString ) -> String {
    let data = dstring.0.string as *const _ as *const u8;
    let len = dstring.0.length as usize;
    let m = unsafe{ mstr::from_mutf8_unchecked( slice::from_raw_parts( data, len ))};
    m.to_utf8().into_owned()
}

impl From<DString> for String {
    fn from( dstring: DString ) -> Self {
        dstring_to_string( &dstring )
    }
}

impl Display for DString {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        dstring_to_string( &self ).fmt( f )
    }
}

impl Default for DString {
    fn default() -> Self { DString::new() }
}

impl DString {
    /// Creates a new, empty `DString`.
    pub fn new() -> Self {
        unsafe {
            let pin_box = Box::pin( MaybeUninit::<Tcl_DString>::zeroed().assume_init() );
            let mut dstring = DString( pin_box );
            Tcl_DStringInit( dstring.as_mut_ptr() );
            dstring
        }
    }

    pub fn into_c_str( self ) -> (c_int, *mut c_char) {
        let tcl_dstring = &*self.0;
        let len = tcl_dstring.length as u32;
        let s = unsafe{ clib::Tcl_Alloc( len + 1 )};
        unsafe{ std::ptr::copy_nonoverlapping( tcl_dstring.string, s, len as usize + 1 ); }
        (len as c_int, s)
    }

    fn as_mut_ptr( &mut self ) -> *mut Tcl_DString { unsafe{ self.0.as_mut().get_unchecked_mut() as *mut _ }}

    /// To append a list element that is itself a sublist, first call `start_list()`, then call `+=` operator for each of the elements in the sublist, then call `end_list()` to end the sublist.
    ///
    /// `start_list()` appends a space character if needed, followed by an open brace.
    ///
    /// Lists can be nested to any depth.
    pub fn start_list( &mut self ) {
        unsafe{ Tcl_DStringStartSublist( self.as_mut_ptr() ); }
    }

    /// To append a list element that is itself a sublist, first call `start_list()`, then call `+=` operator for each of the elements in the sublist, then call `end_list()` to end the sublist.
    ///
    /// `end_list()` appends a close brace.
    ///
    /// Lists can be nested to any depth.
    pub fn end_list( &mut self ) {
        unsafe{ Tcl_DStringEndSublist( self.as_mut_ptr() ); }
    }
}

impl Drop for DString {
    fn drop( &mut self ) {
        unsafe {
            Tcl_DStringFree( self.as_mut_ptr() );
        }
    }
}

impl<R:AsRef<str>> AddAssign<R> for DString {
    fn add_assign( &mut self, rhs: R ) {
        let cstring = CString::new( rhs.as_ref() ).expect("C-style &str to append DString");
        unsafe{ Tcl_DStringAppendElement( self.as_mut_ptr(), cstring.as_ptr() ); }
    }
}

impl AddAssign<&Self> for DString {
    fn add_assign( &mut self, rhs: &Self ) {
        unsafe{ Tcl_DStringAppendElement( self.as_mut_ptr(),  rhs.0.string ); }
    }
}

/// A structure for serializing `Obj` into Tcl string.
pub struct Serializer {
    output: DString,
}

impl Serializer {
    fn new() -> Self {
        Serializer{ output: DString::new() }
    }
}

/// Serialize the given data structure as a Tcl string.
///
/// # Errors
///
/// Serialization will never fail.
pub fn to_string<T:Serialize>( value: &T ) -> Result<String> {
    let mut serializer = Serializer::new();
    value.serialize( &mut serializer )?;
    Ok( serializer.output.to_string() )
}

/// Serialize the given data structure as a Tcl string,
/// returning the length and address of a MUTF-8, C-style string.
///
/// # Errors
///
/// Serialization will never fail.
pub fn to_c_str<T:Serialize>( value: &T ) -> Result<(c_int, *mut c_char)> {
    let mut serializer = Serializer::new();
    value.serialize( &mut serializer )?;
    Ok( serializer.output.into_c_str() )
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool( self, v: bool ) -> Result<()> { self.output += if v { "true" } else { "false" }; Ok(()) }
    fn serialize_i8(   self, v: i8   ) -> Result<()> { self.serialize_i64(i64::from( v )) }
    fn serialize_i16(  self, v: i16  ) -> Result<()> { self.serialize_i64( i64::from(v) )}
    fn serialize_i32(  self, v: i32  ) -> Result<()> { self.serialize_i64(i64::from( v ))}
    fn serialize_i64(  self, v: i64  ) -> Result<()> { self.output += &v.to_string(); Ok(()) }
    fn serialize_u8(   self, v: u8   ) -> Result<()> { self.serialize_u64(u64::from( v ))}
    fn serialize_u16(  self, v: u16  ) -> Result<()> { self.serialize_u64(u64::from( v ))}
    fn serialize_u32(  self, v: u32  ) -> Result<()> { self.serialize_u64(u64::from( v ))}
    fn serialize_u64(  self, v: u64  ) -> Result<()> { self.output += &v .to_string(); Ok(()) }
    fn serialize_f32(  self, v: f32  ) -> Result<()> { self.serialize_f64(f64::from( v ))}
    fn serialize_f64(  self, v: f64  ) -> Result<()> { self.output += &v .to_string(); Ok(()) }
    fn serialize_char( self, v: char ) -> Result<()> { self.serialize_str( &v.to_string() )}
    fn serialize_str(  self, v: &str ) -> Result<()> { self.output += v; Ok(()) }

    fn serialize_bytes( self, v: &[u8] ) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut tuple = self.serialize_seq( None )?;
        for byte in v {
            tuple.serialize_element( byte )?;
        }
        tuple.end()
    }

    fn serialize_none( self ) -> Result<()> {
        self.output += "";
        Ok(())
    }

    fn serialize_some<T>( self, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        self.output += "Some";

        let mut sub = Serializer::new();
        value.serialize( &mut sub )?;
        self.output += &sub.output;

        Ok(())
    }

    fn serialize_unit( self ) -> Result<()> {
        self.output += "";
        Ok(())
    }

    fn serialize_unit_struct( self, _name: &'static str ) -> Result<()> { self.serialize_unit() }

    fn serialize_unit_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str ) -> Result<()> {
        self.serialize_str( variant )
    }

    fn serialize_newtype_struct<T>( self, _name: &'static str, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        value.serialize( self )
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to Tcl value in externally tagged form as `{ NAME { VALUE } }`.
    fn serialize_newtype_variant<T>( self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        variant.serialize( &mut *self )?;

        self.output.start_list();
        value.serialize( &mut *self )?;
        self.output.end_list();

        Ok(())
    }

    fn serialize_seq( self, _len: Option<usize> ) -> Result<Self::SerializeSeq> {
        Ok( self )
    }

    fn serialize_tuple( self, _len: usize ) -> Result<Self::SerializeTuple> {
        self.serialize_seq( None )
    }

    // Tuple structs look just like sequences.
    fn serialize_tuple_struct( self, _name: &'static str, _len: usize ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq( None )
    }

    // Tuple variants are represented in JSON as `{ NAME {DATA...} }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize ) -> Result<Self::SerializeTupleVariant> {
        variant.serialize( &mut *self )?;
        self.output.start_list();
        Ok( self )
    }

    // Maps are represented as `{ K V K V ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok( self )
    }

    // Structs look just like maps.
    fn serialize_struct( self, _name: &'static str, _len: usize ) -> Result<Self::SerializeStruct> {
        Ok( self )
    }

    // Struct variants are represented as `{ NAME { K V ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize ) -> Result<Self::SerializeStructVariant> {
        variant.serialize(&mut *self)?;
        self.output.start_list();
        Ok( self )
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>( &mut self, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        value.serialize( &mut **self )
    }

    // Close the sequence.
    fn end( self ) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T>( &mut self, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        value.serialize( &mut **self )
    }

    fn end( self ) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T>( &mut self, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        value.serialize( &mut **self )
    }

    fn end( self ) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T>( &mut self, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        value.serialize( &mut **self )
    }

    fn end( self ) -> Result<()> {
        self.output.end_list();
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T>( &mut self, key: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        key.serialize( &mut **self )
    }

    fn serialize_value<T>( &mut self, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        value.serialize( &mut **self )
    }

    fn end( self ) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        key.serialize( &mut **self )?;
        value.serialize( &mut **self )
    }

    fn end( self ) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T>( &mut self, key: &'static str, value: &T ) -> Result<()>
        where T: ?Sized + Serialize,
    {
        key.serialize( &mut **self )?;
        value.serialize( &mut **self )
    }

    fn end( self ) -> Result<()> {
        self.output.end_list();
        Ok(())
    }
}

#[cfg( test )]
mod tests {
    use super::*;

    #[test]
    fn serde_works() {
        #[derive( serde::Serialize )]
        enum Value {
            Text( String ),
            Bin( Vec<u8> ),
            Code( i32 ),
        }
        let v = vec![ Value::Text( "hello, world!".to_owned() ), Value::Bin( vec![ 3, 7, 2, 1 ] ), Value::Code(42) ];

        println!( "{}", super::to_string( &v ).unwrap() );
    }

    #[test]
    fn tuple_struct_serde_works() {
        #[derive( serde::Serialize )]
        struct Value( Option<String>, Option<Vec<u8>>, Option<i32> );

        let v = Value( Some( "hello, world!".to_owned() ), Some( vec![ 3, 7, 2, 1 ]), None );

        println!( "{}", to_string( &v ).unwrap() );
    }

    #[test]
    fn ser_unit() {
        assert_eq!( to_string( &() ).unwrap(), "{}" );
    }

    #[test]
    fn ser_struct() {
        #[derive( Debug, serde::Serialize )]
        struct Struct{ a: i32, b: bool, c: f64 }
        let v = Struct{ a: 1, b: false, c: 3.14 };
        assert_eq!( to_string( &v ).unwrap(), "a 1 b false c 3.14" );
    }
}
