//! serde Serializer / Deserializer for msgpack.
//!
//! Provides [`to_bytes`] and [`from_value`] / [`from_bytes`] for converting between
//! Rust types (via serde) and the msgpack wire format used by the FFI bridge.
//!
//! This module bridges `serde` with the existing [`crate::msgpack`] encoder/decoder.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use serde::de::{self, Deserializer as _, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::ser::{self, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple, SerializeTupleStruct};
use serde::{Deserialize, Serialize};

use crate::msgpack::{self, Value};

// ─── Error ───────────────────────────────────────────────────────────────

/// Serialization / deserialization error.
#[derive(Debug, Clone)]
pub struct Error(pub String);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl ser::Error for Error {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

// ─── Serialize → Value ───────────────────────────────────────────────────

/// Serialize a value to a msgpack [`Value`].
pub fn to_value<T: Serialize>(val: &T) -> Result<Value, Error> {
    val.serialize(ValueSerializer)
}

/// Serialize a value to msgpack bytes.
pub fn to_bytes<T: Serialize>(val: &T) -> Result<Vec<u8>, Error> {
    let v = to_value(val)?;
    Ok(msgpack::encode_value(&v))
}

struct ValueSerializer;

impl ser::Serializer for ValueSerializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = SeqSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = MapSerializer;
    type SerializeStructVariant = MapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Value, Error> {
        Ok(Value::Bool(v))
    }
    fn serialize_i8(self, v: i8) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_i16(self, v: i16) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_i32(self, v: i32) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_i64(self, v: i64) -> Result<Value, Error> {
        Ok(Value::Integer(v))
    }
    fn serialize_u8(self, v: u8) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_u16(self, v: u16) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_u32(self, v: u32) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_u64(self, v: u64) -> Result<Value, Error> {
        Ok(Value::Integer(v as i64))
    }
    fn serialize_f32(self, v: f32) -> Result<Value, Error> {
        Ok(Value::Float(v as f64))
    }
    fn serialize_f64(self, v: f64) -> Result<Value, Error> {
        Ok(Value::Float(v))
    }
    fn serialize_char(self, v: char) -> Result<Value, Error> {
        let mut buf = [0u8; 4];
        let s = v.encode_utf8(&mut buf);
        Ok(Value::String(String::from(s)))
    }
    fn serialize_str(self, v: &str) -> Result<Value, Error> {
        Ok(Value::String(String::from(v)))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Value, Error> {
        Ok(Value::Binary(v.to_vec()))
    }
    fn serialize_none(self) -> Result<Value, Error> {
        Ok(Value::Nil)
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Value, Error> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Value, Error> {
        Ok(Value::Nil)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, Error> {
        Ok(Value::Nil)
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Value, Error> {
        Ok(Value::String(String::from(variant)))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value, Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, Error> {
        use alloc::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert(String::from(variant), value.serialize(ValueSerializer)?);
        Ok(Value::Map(map))
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<SeqSerializer, Error> {
        Ok(SeqSerializer(Vec::with_capacity(len.unwrap_or(0))))
    }
    fn serialize_tuple(self, len: usize) -> Result<SeqSerializer, Error> {
        Ok(SeqSerializer(Vec::with_capacity(len)))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<SeqSerializer, Error> {
        Ok(SeqSerializer(Vec::with_capacity(len)))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<SeqSerializer, Error> {
        Ok(SeqSerializer(Vec::with_capacity(len)))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<MapSerializer, Error> {
        Ok(MapSerializer {
            map: alloc::collections::BTreeMap::new(),
            key: None,
        })
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<MapSerializer, Error> {
        Ok(MapSerializer {
            map: alloc::collections::BTreeMap::new(),
            key: None,
        })
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<MapSerializer, Error> {
        Ok(MapSerializer {
            map: alloc::collections::BTreeMap::new(),
            key: None,
        })
    }
}

struct SeqSerializer(Vec<Value>);

impl SerializeSeq for SeqSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.0.push(value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(self.0))
    }
}

impl SerializeTuple for SeqSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.0.push(value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(self.0))
    }
}

impl SerializeTupleStruct for SeqSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.0.push(value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(self.0))
    }
}

impl ser::SerializeTupleVariant for SeqSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.0.push(value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(self.0))
    }
}

struct MapSerializer {
    map: alloc::collections::BTreeMap<String, Value>,
    key: Option<String>,
}

impl SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Error> {
        let v = key.serialize(ValueSerializer)?;
        match v {
            Value::String(s) => {
                self.key = Some(s);
                Ok(())
            }
            _ => Err(Error("map key must be a string".to_string())),
        }
    }
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        let k = self.key.take().ok_or_else(|| Error("missing key".to_string()))?;
        self.map.insert(k, value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.map))
    }
}

impl SerializeStruct for MapSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.map
            .insert(String::from(key), value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.map))
    }
}

impl ser::SerializeStructVariant for MapSerializer {
    type Ok = Value;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.map
            .insert(String::from(key), value.serialize(ValueSerializer)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.map))
    }
}

// ─── Deserialize ← Value ────────────────────────────────────────────────

/// Deserialize from a msgpack [`Value`].
pub fn from_value<'de, T: Deserialize<'de>>(value: Value) -> Result<T, Error> {
    T::deserialize(ValueDeserializer(value))
}

/// Deserialize from raw msgpack bytes.
pub fn from_bytes<'de, T: Deserialize<'de>>(data: &[u8]) -> Result<T, Error> {
    if data.is_empty() {
        return T::deserialize(ValueDeserializer(Value::Nil));
    }
    let (value, _) = Value::decode(data).unwrap_or((Value::Nil, 0));
    from_value(value)
}

struct ValueDeserializer(Value);

impl<'de> de::Deserializer<'de> for ValueDeserializer {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Nil => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Integer(i) => visitor.visit_i64(i),
            Value::Float(f) => visitor.visit_f64(f),
            Value::String(s) => visitor.visit_string(s),
            Value::Binary(b) => visitor.visit_byte_buf(b),
            Value::Array(arr) => {
                let len = arr.len();
                visitor.visit_seq(SeqDeserializer {
                    iter: arr.into_iter(),
                    len,
                })
            }
            Value::Map(map) => {
                let len = map.len();
                visitor.visit_map(MapDeserializer {
                    iter: map.into_iter(),
                    value: None,
                    len,
                })
            }
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Bool(b) => visitor.visit_bool(b),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_i8(i as i8),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_i16(i as i16),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_i32(i as i32),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_i64(i),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_u8(i as u8),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_u16(i as u16),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_u32(i as u32),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Integer(i) => visitor.visit_u64(i as u64),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Float(f) => visitor.visit_f32(f as f32),
            Value::Integer(i) => visitor.visit_f32(i as f32),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Float(f) => visitor.visit_f64(f),
            Value::Integer(i) => visitor.visit_f64(i as f64),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::String(ref s) if s.len() == 1 => {
                visitor.visit_char(s.chars().next().unwrap())
            }
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::String(s) => visitor.visit_string(s),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_str(visitor)
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Binary(b) => visitor.visit_byte_buf(b),
            _ => self.deserialize_any(visitor),
        }
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_bytes(visitor)
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Nil => visitor.visit_none(),
            other => visitor.visit_some(ValueDeserializer(other)),
        }
    }
    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_unit()
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_unit()
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Array(arr) => {
                let len = arr.len();
                visitor.visit_seq(SeqDeserializer {
                    iter: arr.into_iter(),
                    len,
                })
            }
            _ => Err(Error("expected array".to_string())),
        }
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Map(map) => {
                let len = map.len();
                visitor.visit_map(MapDeserializer {
                    iter: map.into_iter(),
                    value: None,
                    len,
                })
            }
            _ => Err(Error("expected map".to_string())),
        }
    }
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_map(visitor)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        match self.0 {
            Value::String(s) => visitor.visit_enum(EnumDeserializer::Unit(s)),
            Value::Map(map) => {
                if let Some((key, val)) = map.into_iter().next() {
                    visitor.visit_enum(EnumDeserializer::Map(key, val))
                } else {
                    Err(Error("expected non-empty map for enum".to_string()))
                }
            }
            _ => Err(Error("expected string or map for enum".to_string())),
        }
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_str(visitor)
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_unit()
    }
}

// ─── Seq Deserializer ────────────────────────────────────────────────────

struct SeqDeserializer {
    iter: alloc::vec::IntoIter<Value>,
    len: usize,
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = Error;
    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Error> {
        match self.iter.next() {
            Some(v) => seed.deserialize(ValueDeserializer(v)).map(Some),
            None => Ok(None),
        }
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

// ─── Map Deserializer ────────────────────────────────────────────────────

struct MapDeserializer {
    iter: alloc::collections::btree_map::IntoIter<String, Value>,
    value: Option<Value>,
    len: usize,
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = Error;
    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Error> {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                seed.deserialize(ValueDeserializer(Value::String(k))).map(Some)
            }
            None => Ok(None),
        }
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Error> {
        let v = self.value.take().ok_or_else(|| Error("missing value".to_string()))?;
        seed.deserialize(ValueDeserializer(v))
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

// ─── Enum Deserializer ───────────────────────────────────────────────────

enum EnumDeserializer {
    Unit(String),
    Map(String, Value),
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V: DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, VariantDeserializer), Error> {
        match self {
            EnumDeserializer::Unit(s) => {
                let val = seed.deserialize(ValueDeserializer(Value::String(s)))?;
                Ok((val, VariantDeserializer(None)))
            }
            EnumDeserializer::Map(key, val) => {
                let k = seed.deserialize(ValueDeserializer(Value::String(key)))?;
                Ok((k, VariantDeserializer(Some(val))))
            }
        }
    }
}

struct VariantDeserializer(Option<Value>);

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        Ok(())
    }
    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Error> {
        match self.0 {
            Some(v) => seed.deserialize(ValueDeserializer(v)),
            None => Err(Error("expected data for newtype variant".to_string())),
        }
    }
    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Some(v) => ValueDeserializer(v).deserialize_seq(visitor),
            None => Err(Error("expected array for tuple variant".to_string())),
        }
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        match self.0 {
            Some(v) => ValueDeserializer(v).deserialize_map(visitor),
            None => Err(Error("expected map for struct variant".to_string())),
        }
    }
}

// ─── Value ↔ serde impls ────────────────────────────────────────────────

impl Serialize for Value {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Nil => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Integer(i) => serializer.serialize_i64(*i),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::String(s) => serializer.serialize_str(s),
            Value::Binary(b) => serializer.serialize_bytes(b),
            Value::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Value::Map(map) => {
                let mut m = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    m.serialize_entry(k, v)?;
                }
                m.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("any msgpack value")
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Nil)
    }
    fn visit_bool<E>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Value, E> {
        Ok(Value::Integer(v))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Value, E> {
        Ok(Value::Integer(v as i64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Value, E> {
        Ok(Value::Float(v))
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(String::from(v)))
    }
    fn visit_string<E>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }
    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Value, E> {
        Ok(Value::Binary(v.to_vec()))
    }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Value, E> {
        Ok(Value::Binary(v))
    }
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Nil)
    }
    fn visit_some<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        Deserialize::deserialize(deserializer)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(item) = seq.next_element()? {
            v.push(item);
        }
        Ok(Value::Array(v))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut m = alloc::collections::BTreeMap::new();
        while let Some((k, v)) = map.next_entry::<String, Value>()? {
            m.insert(k, v);
        }
        Ok(Value::Map(m))
    }
}
