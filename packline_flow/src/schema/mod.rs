use bytes::{BufMut, BytesMut};
use std::convert::TryInto;

pub enum Types {
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    VarInt,
    VarLong,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    String,
    Bytes,
}

pub trait SizedSchema {
    fn size(&self) -> usize;
}

pub trait SerializableSchema: SizedSchema {
    fn serialize(&self, encoder: &mut BytesMut);

    type Error;
}

pub trait DeserializableSchema: SizedSchema {
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<Self::Item, Self::Error>
    where
        Self: Sized;

    type Item;
    type Error;
}

pub trait Schema: SerializableSchema + DeserializableSchema {}

impl SerializableSchema for i8 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_i8(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i8 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<i8, std::convert::Infallible> {
        Ok(i8::from_be_bytes(
            decoder.next(1).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = i8;
    type Error = std::convert::Infallible;
}

impl SizedSchema for i8 {
    fn size(&self) -> usize {
        std::mem::size_of::<i8>()
    }
}

impl SerializableSchema for i16 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_i16(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i16 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<i16, std::convert::Infallible> {
        Ok(i16::from_be_bytes(
            decoder.next(2).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = i16;
    type Error = std::convert::Infallible;
}

impl SizedSchema for i16 {
    fn size(&self) -> usize {
        std::mem::size_of::<i16>()
    }
}

impl SerializableSchema for i32 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_i32(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i32 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<i32, std::convert::Infallible> {
        Ok(i32::from_be_bytes(
            decoder.next(4).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = i32;
    type Error = std::convert::Infallible;
}

impl SizedSchema for i32 {
    fn size(&self) -> usize {
        std::mem::size_of::<i32>()
    }
}

impl SerializableSchema for i64 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_i64(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i64 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<i64, std::convert::Infallible> {
        Ok(i64::from_be_bytes(
            decoder.next(8).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = i64;
    type Error = std::convert::Infallible;
}

impl SizedSchema for i64 {
    fn size(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl SerializableSchema for u8 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_u8(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u8 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<u8, std::convert::Infallible> {
        Ok(u8::from_be_bytes(
            decoder.next(1).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = u8;
    type Error = std::convert::Infallible;
}

impl SizedSchema for u8 {
    fn size(&self) -> usize {
        std::mem::size_of::<u8>()
    }
}

impl SerializableSchema for u16 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_u16(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u16 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<u16, std::convert::Infallible> {
        Ok(u16::from_be_bytes(
            decoder.next(2).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = u16;
    type Error = std::convert::Infallible;
}

impl SizedSchema for u16 {
    fn size(&self) -> usize {
        std::mem::size_of::<u16>()
    }
}

impl SerializableSchema for u32 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_u32(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u32 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<u32, std::convert::Infallible> {
        Ok(u32::from_be_bytes(
            decoder.next(4).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = u32;
    type Error = std::convert::Infallible;
}

impl SizedSchema for u32 {
    fn size(&self) -> usize {
        std::mem::size_of::<u32>()
    }
}

impl SerializableSchema for u64 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_u64(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u64 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<u64, std::convert::Infallible> {
        Ok(u64::from_be_bytes(
            decoder.next(8).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = u64;
    type Error = std::convert::Infallible;
}

impl SizedSchema for u64 {
    fn size(&self) -> usize {
        std::mem::size_of::<u64>()
    }
}

impl SerializableSchema for f32 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_f32(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for f32 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<f32, std::convert::Infallible> {
        Ok(f32::from_be_bytes(
            decoder.next(4).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = f32;
    type Error = std::convert::Infallible;
}

impl SizedSchema for f32 {
    fn size(&self) -> usize {
        std::mem::size_of::<f32>()
    }
}

impl SerializableSchema for f64 {
    fn serialize(&self, encoder: &mut BytesMut) {
        encoder.put_f64(*self);
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for f64 {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<f64, std::convert::Infallible> {
        Ok(f64::from_be_bytes(
            decoder.next(8).try_into().expect("slice with incorrect length"),
        ))
    }

    type Item = f64;
    type Error = std::convert::Infallible;
}

impl SizedSchema for f64 {
    fn size(&self) -> usize {
        std::mem::size_of::<f64>()
    }
}

impl SerializableSchema for String {
    fn serialize(&self, encoder: &mut BytesMut) {
        (self.len() as i64).serialize(encoder);
        encoder.put(self.as_bytes());
    }

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for String {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<String, std::convert::Infallible> {
        let len = i64::deserialize(decoder).unwrap() as usize;
        let mut buf = vec![0u8; len];

        buf.copy_from_slice(decoder.next(len));

        Ok(String::from_utf8(buf).unwrap())
    }

    type Item = String;
    type Error = std::convert::Infallible;
}

impl SizedSchema for String {
    fn size(&self) -> usize {
        std::mem::size_of::<i64>() + self.len()
    }
}

impl<T: SerializableSchema> SerializableSchema for Vec<T> {
    fn serialize(&self, encoder: &mut BytesMut) {
        (self.len() as i64).serialize(encoder);
        for i in self {
            i.serialize(encoder);
        }
    }

    type Error = std::convert::Infallible;
}

impl<T: DeserializableSchema<Item = T, Error = std::convert::Infallible>> DeserializableSchema for Vec<T> {
    #[inline(always)]
    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<Vec<T>, std::convert::Infallible> {
        let len = i64::deserialize(decoder).unwrap() as usize;
        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            result.push(T::deserialize(decoder).unwrap());
        }

        Ok(result)
    }

    type Item = Vec<T>;
    type Error = std::convert::Infallible;
}

impl<T: SizedSchema> SizedSchema for Vec<T>
where
    T: SizedSchema,
{
    fn size(&self) -> usize {
        self.iter().map(SizedSchema::size).sum::<usize>() + std::mem::size_of::<i64>()
    }
}
