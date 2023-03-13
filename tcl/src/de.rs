//! Deserialize Tcl values to Rust values.

use serde::de::{
    self, DeserializeOwned, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use crate::{
    DictIter, Obj,
    error::{DeError, DeKind},
};

use std::{
    convert::TryFrom,
    os::raw::c_int,
};

type Result<T, E=DeError> = std::result::Result<T,E>;

/// A structure for deserializing Tcl `Obj`s into Rust values.
pub struct Deserializer {
    stack : Vec<Obj>,
}

/// Deserialize an instance of type `T` from a Tcl obj.
///
/// # Errors
///
/// This conversion can fail for various reasons. See `DeError` definition in error.rs for details.
pub fn from_obj<T:DeserializeOwned>( obj: Obj ) -> Result<T> {
    T::deserialize( &mut Deserializer::from_obj( obj ))
}

impl Deserializer {
    fn from_obj( obj: Obj ) -> Self {
        Deserializer {
            stack : vec![ obj ]
        }
    }

    fn push( &mut self, obj: Obj ) { self.stack.push( obj ); }

    fn pop( &mut self ) -> Obj { self.stack.pop().unwrap() }

    fn peek( &self ) -> &Obj { self.stack.last().unwrap() }

    fn peek_len( &self ) -> Result<c_int> { Ok( self.peek().list_length()? )}

    fn parse_bool( &mut self ) -> Result<bool> { bool::try_from( self.pop() )}
    fn parse_u8(   &mut self ) -> Result<u8>   {   u8::try_from( self.pop() )}
    fn parse_u16(  &mut self ) -> Result<u16>  {  u16::try_from( self.pop() )}
    fn parse_u32(  &mut self ) -> Result<u32>  {  u32::try_from( self.pop() )}
    fn parse_u64(  &mut self ) -> Result<u64>  {  u64::try_from( self.pop() )}
    fn parse_i8(   &mut self ) -> Result<i8>   {   i8::try_from( self.pop() )}
    fn parse_i16(  &mut self ) -> Result<i16>  {  i16::try_from( self.pop() )}
    fn parse_i32(  &mut self ) -> Result<i32>  {  i32::try_from( self.pop() )}
    fn parse_i64(  &mut self ) -> Result<i64>  {  i64::try_from( self.pop() )}
    fn parse_f32(  &mut self ) -> Result<f32>  {  f32::try_from( self.pop() )}
    fn parse_f64(  &mut self ) -> Result<f64>  {  f64::try_from( self.pop() )}

    fn parse_str( &mut self ) -> &'static str {
        Box::leak( self.pop().to_string().into_boxed_str() )
    }
}

impl<'a,'de:'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = DeError;

    fn deserialize_any     <V:Visitor<'de>>( self, _visitor:V ) -> Result<V::Value> { unimplemented!() }
    fn deserialize_bool    <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_bool( self.parse_bool()? )}
    fn deserialize_i8      <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_i8(   self.parse_i8()  ? )}
    fn deserialize_i16     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_i16(  self.parse_i16() ? )}
    fn deserialize_i32     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_i32(  self.parse_i32() ? )}
    fn deserialize_i64     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_i64(  self.parse_i64() ? )}
    fn deserialize_u8      <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_u8(   self.parse_u8()  ? )}
    fn deserialize_u16     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_u16(  self.parse_u16() ? )}
    fn deserialize_u32     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_u32(  self.parse_u32() ? )}
    fn deserialize_u64     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_u64(  self.parse_u64() ? )}
    fn deserialize_f32     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_f32(  self.parse_f32() ? )}
    fn deserialize_f64     <V:Visitor<'de>>( self,  visitor:V ) -> Result<V::Value> { visitor.visit_f64(  self.parse_f64() ? )}
    fn deserialize_bytes   <V:Visitor<'de>>( self, _visitor:V ) -> Result<V::Value> { unimplemented!() }
    fn deserialize_byte_buf<V:Visitor<'de>>( self, _visitor:V ) -> Result<V::Value> { unimplemented!() }

    fn deserialize_char<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        let obj = self.pop();
        let s = obj.to_string();
        if s.len() == 1 {
            visitor.visit_char( s.chars().next().unwrap() )
        } else {
            Err( DeError::new( DeKind::NotChar, obj ))
        }
    }

    fn deserialize_str<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        visitor.visit_borrowed_str( self.parse_str() )
    }

    fn deserialize_string<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        visitor.visit_string( self.pop().to_string() )
    }

    fn deserialize_option<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        if self.peek_len()? == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some( self )
        }
    }

    fn deserialize_unit<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        if self.peek_len()? == 0 {
            visitor.visit_unit()
        } else {
            Err( DeError::new( DeKind::NotUnit, self.pop() ))
        }
    }

    fn deserialize_unit_struct<V:Visitor<'de>>( self, _name: &'static str, visitor: V ) -> Result<V::Value> {
        self.deserialize_unit( visitor )
    }

    fn deserialize_newtype_struct<V:Visitor<'de>>( self, _name: &'static str, visitor: V ) -> Result<V::Value> {
        visitor.visit_newtype_struct( self )
    }

    fn deserialize_seq<V:Visitor<'de>>( mut self, visitor: V ) -> Result<V::Value> {
        visitor.visit_seq( ListAccess::new( &mut self ))
    }

    fn deserialize_tuple<V:Visitor<'de>>( mut self, _len: usize, visitor: V ) -> Result<V::Value> {
        visitor.visit_seq( ListAccess::new( &mut self ))
    }

    fn deserialize_tuple_struct<V:Visitor<'de>>( self, _name: &'static str, len: usize, visitor: V ) -> Result<V::Value> {
        self.deserialize_tuple( len, visitor )
    }

    fn deserialize_map<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        visitor.visit_map( DictAccess::new( self )? )
    }

    fn deserialize_struct<V:Visitor<'de>>( self, _name: &'static str, _fields: &'static [&'static str], visitor: V ) -> Result<V::Value> {
        visitor.visit_map( DictAccess::new( self )? )
    }

    fn deserialize_enum<V:Visitor<'de>>( self, _name: &'static str, _variants: &'static [&'static str], visitor: V ) -> Result<V::Value> {
        match self.peek_len()? {
            0 => {
                self.pop();
                visitor.visit_enum( "\"\"".into_deserializer() )
            },
            1..=2 => {
                Ok( visitor.visit_enum( Enum::new( self ))? )
            },
            _ => Err( DeError::new( DeKind::NotEnum, self.pop() )),
        }

    }

    fn deserialize_identifier<V:Visitor<'de>>( self, visitor: V ) -> Result<V::Value> {
        self.deserialize_str( visitor )
    }

    fn deserialize_ignored_any<V:Visitor<'de>>( self, _visitor: V ) -> Result<V::Value> {
        unimplemented!();
    }
}

struct ListAccess<'a> {
    de    : &'a mut Deserializer,
    list  : Obj,
    index : c_int,
}

impl<'a> ListAccess<'a> {
    fn new( de: &'a mut Deserializer ) -> Self {
        let list = de.peek().clone();
        ListAccess{ de, list, index: 0 }
    }
}

impl<'a, 'de:'a> SeqAccess<'de> for ListAccess<'a> {
    type Error = DeError;

    fn next_element_seed<T:DeserializeSeed<'de>>( &mut self, seed: T ) -> Result<Option<T::Value>> {
        self.list
            .list_index( self.index )
            .map_err( From::from )
            .and_then( |elem| match elem {
                None => Ok( None ),
                Some( elem ) => {
                    self.de.push( elem );
                    self.index += 1;
                    let de_seed = DeserializeSeed::deserialize( seed, &mut *self.de )
                        .map_err( |e| DeError::chain(
                            DeKind::List{ err_idx: self.index as usize },
                            self.list.clone(),
                            e
                        ))?;
                    Ok( Some( de_seed ))
                },
            })
    }
}

struct DictAccess<'a> {
    de        : &'a mut Deserializer,
    dict      : Obj,
    dict_iter : DictIter,
    value     : Option<Obj>,
}

impl<'a> DictAccess<'a> {
    fn new( de: &'a mut Deserializer ) -> Result<Self, DeError> {
        let dict      = de.peek().clone();
        let dict_iter = de.peek().clone().dict_iter()?;
        Ok( DictAccess {
            de       ,
            dict     ,
            dict_iter,
            value    : None,
        })
    }
}

impl<'a, 'de:'a> MapAccess<'de> for DictAccess<'a> {
    type Error = DeError;

    fn next_key_seed<K:DeserializeSeed<'de>>( &mut self, seed: K ) -> Result<Option<K::Value>> {
        match self.dict_iter.next() {
            None => Ok( None ),
            Some(( key, value )) => {
                self.value = Some( value );
                self.de.push( key );
                let de_seed = seed
                    .deserialize( &mut *self.de )
                    .map( Some )
                    .map_err( |e| DeError::chain( DeKind::DictBadKey, self.dict.clone(), e ))?;
                Ok( de_seed )
            },
        }
    }

    fn next_value_seed<V:DeserializeSeed<'de>>( &mut self, seed: V ) -> Result<V::Value> {
        self.de.push( self.value.take().unwrap() );
        let de_seed = seed
            .deserialize( &mut *self.de )
            .map_err( |e| DeError::chain( DeKind::DictBadVal, self.dict.clone(), e ))?;
        Ok( de_seed )
    }
}

struct Enum<'a> {
    de: &'a mut Deserializer,
}

impl<'a> Enum<'a> {
    fn new( de: &'a mut Deserializer ) -> Self { Enum{ de }}
}

impl<'a, 'de:'a> EnumAccess<'de> for Enum<'a> {
    type Error = DeError;
    type Variant = Self;

    fn variant_seed<V:DeserializeSeed<'de>>( self, seed: V ) -> Result<( V::Value, Self::Variant )> {
        let val = seed.deserialize( &mut *self.de )?;
        Ok(( val, self ))
    }
}

impl<'a, 'de:'a> VariantAccess<'de> for Enum<'a> {
    type Error = DeError;

    fn unit_variant( self ) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T:DeserializeSeed<'de>>( self, seed: T ) -> Result<T::Value> {
        seed.deserialize( self.de )
    }

    fn tuple_variant<V:Visitor<'de>>( self, len: usize, visitor: V ) -> Result<V::Value> {
        de::Deserializer::deserialize_tuple( self.de, len, visitor )
    }

    fn struct_variant<V:Visitor<'de>>( self, fields: &'static [&'static str], visitor: V ) -> Result<V::Value> {
        de::Deserializer::deserialize_tuple( self.de, fields.len(), visitor )
    }
}

#[cfg( test )]
mod tests {
    use std::ptr::null_mut;
    use super::*;

    #[test]
    fn de_unit() {
        let obj = Obj::new();
        let result: () = from_obj( obj ).unwrap();
        assert_eq!( result, () );
    }

    #[test]
    fn de_struct() {
        let obj = Obj::from( "a 1 b false c 3.14" );
        #[derive( PartialEq, Debug, serde::Deserialize )]
        struct Struct{ a: i32, b: bool, c: f64 }
        let result: Struct = from_obj( obj.clone() ).unwrap();
        assert_eq!( result, Struct{ a: 1, b: false, c: 3.14 });
        println!( "Obj:{}", obj.get_string() );
    }

    #[test]
    fn de_vec() {
        let obj = Obj::from( ".c.feet .c.meters .c.calc .c.flbl .c.islbl .c.mlbl" );
        //let result: Vec<&'static str> = from_obj( InterpObj( &mut interpreter, &mut obj )).unwrap();
        let result: Vec<String> = from_obj( obj ).unwrap();
        assert_eq!( result, vec![ ".c.feet".to_owned(), ".c.meters".to_owned(), ".c.calc".to_owned(), ".c.flbl".to_owned(), ".c.islbl".to_owned(), ".c.mlbl".to_owned(), ] );
        //assert_eq!( result, vec![ ".c.feet", ".c.meters", ".c.calc", ".c.flbl", ".c.islbl", ".c.mlbl", ] );
    }

    #[test]
    fn test_get_int() {
        let mut value: c_int = 0;
        let obj = Obj::from( "20" );
        unsafe{ clib::Tcl_GetIntFromObj( null_mut(), obj.as_ptr(), &mut value ); }
    }
}
