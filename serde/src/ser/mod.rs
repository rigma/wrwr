use super::error::{Error, Result};
use std::io::Write;

/// A serializer which is marshalling a [Serde] data structure into a
/// marshalled binary packet suited for network communication.
///
/// Be advice that the produced packets should be read thanks to a data
/// wire. For instance, this serializer does not include the length of
/// a string into the serialization.
#[derive(Clone, Debug)]
pub struct Serializer {
    writer: Vec<u8>,
}

impl Serializer {
    /// Instanciantes a new instance of the serializer.
    pub fn new() -> Self {
        Self {
            writer: Vec::new(),
        }
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ser> serde::ser::Serializer for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'ser>;
    type SerializeTuple = Compound<'ser>;
    type SerializeTupleStruct = Compound<'ser>;
    type SerializeTupleVariant = Compound<'ser>;
    type SerializeMap = Compound<'ser>;
    type SerializeStruct = Compound<'ser>;
    type SerializeStructVariant = Compound<'ser>;

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str
    ) -> Result<Self::Ok> {
        self.serialize_u32(variant_index)
    }

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        self.writer
            .write(if value { &[1] } else { &[0] })
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        self.writer
            .write(&[value])
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        self.writer
            .write(&[value as u8])
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        let bytes = value.to_ne_bytes();

        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_char(self, character: char) -> Result<Self::Ok> {
        let mut bytes = [0; 4];
        character.encode_utf8(&mut bytes);
        
        self.writer
            .write(&bytes)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn serialize_str(self, string: &str) -> Result<Self::Ok> {
        self.writer
            .write_all(string.as_bytes())
            .map_err(Into::into)
    }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<Self::Ok> {
        self.writer
            .write_all(&bytes)
            .map_err(Into::into)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Compound::new(self, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(Compound::new(self, Some(len)))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        Ok(Compound::new(self, Some(len)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str, 
        len: usize
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_u32(variant_index)?;
        Ok(Compound::new(self, Some(len)))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Compound::new(self, len))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(Compound::new(self, Some(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_u32(variant_index)?;
        Ok(Compound::new(self, Some(len)))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T
    ) -> Result<()>
    where
        T: serde::Serialize + ?Sized
    {
        self.serialize_u32(variant_index)?;
        value.serialize(&mut *self)?;
        Ok(())
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Compound<'ser> {
    serializer: &'ser mut Serializer,
    element_count: usize,
    max_count: Option<usize>,
}

impl<'ser> Compound<'ser> {
    pub fn new(serializer: &'ser mut Serializer, max_count: Option<usize>) -> Self {
        Self {
            serializer,
            element_count: 0,
            max_count,
        }
    }
}

impl<'ser> serde::ser::SerializeSeq for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }

        value.serialize(&mut *self.serializer)?;
        self.element_count += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser> serde::ser::SerializeTuple for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }

        value.serialize(&mut *self.serializer)?;
        self.element_count += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser> serde::ser::SerializeTupleStruct for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }
    
        value.serialize(&mut *self.serializer)?;
        self.element_count += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser> serde::ser::SerializeTupleVariant for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }

        value.serialize(&mut *self.serializer)?;
        self.element_count += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser> serde::ser::SerializeMap for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<Self::Ok>
    where
        K: serde::Serialize + ?Sized,
        V: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }

        self.serialize_key(key)?;
        self.serialize_value(value)?;
        self.element_count += 1;
        Ok(())
    }

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        key.serialize(&mut *self.serializer)
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        value.serialize(&mut *self.serializer)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser> serde::ser::SerializeStruct for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }

        value.serialize(&mut *self.serializer)?;
        self.element_count += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser> serde::ser::SerializeStructVariant for Compound<'ser> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize + ?Sized,
    {
        if let Some(max_count) = &self.max_count {
            if *max_count < self.element_count + 1 {
                return Err(Error::SizeLimit(*max_count));
            }
        }

        value.serialize(&mut *self.serializer)?;
        self.element_count += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn it_serializes_an_unit() {
        let mut serializer = Serializer::new();
        let result = Serialize::serialize(&(), &mut serializer);

        assert!(result.is_ok());
        assert_eq!(Vec::<u8>::new(), serializer.writer);
    }

    #[test]
    fn it_serializes_an_unit_strict() {
        #[derive(Serialize)]
        struct Unit;

        let mut serializer = Serializer::new();
        let result = Serialize::serialize(&Unit, &mut serializer);

        assert!(result.is_ok());
        assert_eq!(Vec::<u8>::new(), serializer.writer);
    }

    #[test]
    fn it_serializes_an_unit_variant() {
        #[derive(Serialize)]
        enum Variant {
            Unit,
        }

        let mut serializer = Serializer::new();
        let result = Serialize::serialize(&Variant::Unit, &mut serializer);

        assert!(result.is_ok());
        assert_eq!(vec![0, 0, 0, 0], serializer.writer);
    }

    #[test]
    fn it_serializes_a_boolean() {
        let mut serializer = Serializer::new();
        
        assert!(Serialize::serialize(&false, &mut serializer).is_ok());
        assert!(Serialize::serialize(&true, &mut serializer).is_ok());

        assert_eq!(vec![0, 1], serializer.writer);
    }

    #[test]
    fn it_serializes_an_u8() {
        let mut serializer = Serializer::new();

        assert!(Serialize::serialize(&0xffu8, &mut serializer).is_ok());
        assert_eq!(vec![0xffu8], serializer.writer);
    }

    #[test]
    fn it_serializes_an_u16() {
        let mut serializer = Serializer::new();

        assert!(Serialize::serialize(&0xffffu16, &mut serializer).is_ok());
        assert_eq!(vec![0xffu8; 2], serializer.writer);
    }

    #[test]
    fn it_serializes_an_u32() {
        let mut serializer = Serializer::new();

        assert!(Serialize::serialize(&0xffffffffu32, &mut serializer).is_ok());
        assert_eq!(vec![0xffu8; 4], serializer.writer);
    }

    #[test]
    fn it_serializes_an_u64() {
        let mut serializer = Serializer::new();

        assert!(Serialize::serialize(&0xffffffffffffffffu64, &mut serializer).is_ok());
        assert_eq!(vec![0xffu8; 8], serializer.writer);
    }

    #[test]
    fn it_serializes_an_u128() {
        let mut serializer = Serializer::new();

        assert!(Serialize::serialize(&0xffffffffffffffffffffffffffffffffu128, &mut serializer).is_ok());
        assert_eq!(vec![0xffu8; 16], serializer.writer);
    }

    #[test]
    fn it_serializes_an_i8() {
        let mut serializer = Serializer::new();
        let n = 0x0fi8;

        assert!(Serialize::serialize(&n, &mut serializer).is_ok());
        assert_eq!(Vec::from(&n.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_an_i16() {
        let mut serializer = Serializer::new();
        let n = 0x0fffi16;

        assert!(Serialize::serialize(&n, &mut serializer).is_ok());
        assert_eq!(Vec::from(&n.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_an_i32() {
        let mut serializer = Serializer::new();
        let n = 0x0fffffffi32;

        assert!(Serialize::serialize(&n, &mut serializer).is_ok());
        assert_eq!(Vec::from(&n.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_an_i64() {
        let mut serializer = Serializer::new();
        let n = 0x0fffffffffffffffi64;

        assert!(Serialize::serialize(&n, &mut serializer).is_ok());
        assert_eq!(Vec::from(&n.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_an_i128() {
        let mut serializer = Serializer::new();
        let n = 0x0fffffffffffffffffffffffffffffffi128;

        assert!(Serialize::serialize(&n, &mut serializer).is_ok());
        assert_eq!(Vec::from(&n.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_a_f32() {
        use std::f32;

        let mut serializer = Serializer::new();
        assert!(Serialize::serialize(&f32::consts::PI, &mut serializer).is_ok());
        assert_eq!(Vec::from(&f32::consts::PI.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_a_f64() {
        use std::f64;

        let mut serializer = Serializer::new();
        assert!(Serialize::serialize(&f64::consts::E, &mut serializer).is_ok());
        assert_eq!(Vec::from(&f64::consts::E.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_a_char() {
        let mut serializer = Serializer::new();
        let mut bytes = [0u8; 4];

        let c = 'ñ';
        c.encode_utf8(&mut bytes);

        assert!(Serialize::serialize(&c, &mut serializer).is_ok());
        assert_eq!(Vec::from(&bytes[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_a_str() {
        let mut serializer = Serializer::new();
        let string = "Hello, world!";

        assert!(Serialize::serialize(&string, &mut serializer).is_ok());
        assert_eq!(Vec::from(string.as_bytes()), serializer.writer);
    }

    #[test]
    fn it_serializes_a_byte_array() {
        let mut serializer = Serializer::new();
        let bytes = [1u8, 2, 3, 4];

        assert!(Serialize::serialize(&bytes, &mut serializer).is_ok());
        assert_eq!(Vec::from(&bytes[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_an_option() {
        let mut serializer = Serializer::new();
        let n = 0x0fffi16;

        let option: Option<i16> = None;
        assert!(Serialize::serialize(&option, &mut serializer).is_ok());

        let option = Some(n);
        assert!(Serialize::serialize(&option, &mut serializer).is_ok());

        assert_eq!(Vec::from(&n.to_ne_bytes()[..]), serializer.writer);
    }

    #[test]
    fn it_serializes_a_sequence() {
        let mut serializer = Serializer::new();
        let v = vec![1u8, 2, 3, 4];

        assert!(Serialize::serialize(&v, &mut serializer).is_ok());
        assert_eq!(v, serializer.writer);
    }

    #[test]
    fn it_serializes_a_tuple() {
        let mut serializer = Serializer::new();
        let tuple = ('ß', std::f64::consts::PI, 2048, 11u8);

        assert!(Serialize::serialize(&tuple, &mut serializer).is_ok());

        let mut expected = Vec::new();

        let mut bytes = [0u8; 4];
        'ß'.encode_utf8(&mut bytes);

        expected.append(&mut Vec::from(&bytes[..]));
        expected.append(&mut Vec::from(&std::f64::consts::PI.to_ne_bytes()[..]));
        expected.append(&mut Vec::from(&2048i32.to_ne_bytes()[..]));
        expected.push(11u8);

        assert_eq!(expected, serializer.writer);
    }

    #[test]
    fn it_serializes_a_map() {
        use std::collections::{BTreeMap, HashMap};

        let mut serializer = Serializer::new();
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("hello".to_string(), 42);

        assert!(Serialize::serialize(&map, &mut serializer).is_ok());

        let mut expected = Vec::new();
        expected.append(&mut Vec::from(&"hello".as_bytes()[..]));
        expected.append(&mut Vec::from(&42i32.to_ne_bytes()[..]));

        assert_eq!(expected, serializer.writer);

        let mut serializer = Serializer::new();
        let mut map: BTreeMap<String, i32> = BTreeMap::new();
        map.insert("hello".to_string(), 42);

        assert!(Serialize::serialize(&map, &mut serializer).is_ok());

        let mut expected = Vec::new();
        expected.append(&mut Vec::from(&"hello".as_bytes()[..]));
        expected.append(&mut Vec::from(&42i32.to_ne_bytes()[..]));

        assert_eq!(expected, serializer.writer);
    }
}
