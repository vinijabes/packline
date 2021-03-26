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
    fn serialize(&self);

    type Error;
}

pub trait DeserializableSchema: SizedSchema {
    fn deserialize() -> Result<Self::Item, Self::Error>
    where
        Self: Sized;

    type Item;
    type Error;
}

pub trait Schema: SerializableSchema + DeserializableSchema {}

impl SerializableSchema for i8 {
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i8 {
    #[inline(always)]
    fn deserialize() -> Result<i8, std::convert::Infallible> {
        Ok(10i8)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i16 {
    #[inline(always)]
    fn deserialize() -> Result<i16, std::convert::Infallible> {
        Ok(10i16)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i32 {
    #[inline(always)]
    fn deserialize() -> Result<i32, std::convert::Infallible> {
        Ok(10i32)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for i64 {
    #[inline(always)]
    fn deserialize() -> Result<i64, std::convert::Infallible> {
        Ok(10i64)
    }

    type Item = i64;
    type Error = std::convert::Infallible;
}

impl SizedSchema for i64 {
    fn size(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl SerializableSchema for u16 {
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u16 {
    #[inline(always)]
    fn deserialize() -> Result<u16, std::convert::Infallible> {
        Ok(10u16)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u32 {
    #[inline(always)]
    fn deserialize() -> Result<u32, std::convert::Infallible> {
        Ok(10u32)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for u64 {
    #[inline(always)]
    fn deserialize() -> Result<u64, std::convert::Infallible> {
        Ok(10u64)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for f32 {
    #[inline(always)]
    fn deserialize() -> Result<f32, std::convert::Infallible> {
        Ok(10f32)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for f64 {
    #[inline(always)]
    fn deserialize() -> Result<f64, std::convert::Infallible> {
        Ok(10f64)
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
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl DeserializableSchema for String {
    #[inline(always)]
    fn deserialize() -> Result<String, std::convert::Infallible> {
        Ok(String::new())
    }

    type Item = String;
    type Error = std::convert::Infallible;
}

impl SizedSchema for String {
    fn size(&self) -> usize {
        std::mem::size_of::<char>() * self.chars().count()
    }
}

impl<T: SerializableSchema> SerializableSchema for Vec<T> {
    fn serialize(&self) {}

    type Error = std::convert::Infallible;
}

impl<T: DeserializableSchema> DeserializableSchema for Vec<T> {
    #[inline(always)]
    fn deserialize() -> Result<Vec<T>, std::convert::Infallible> {
        Ok(Vec::new())
    }

    type Item = Vec<T>;
    type Error = std::convert::Infallible;
}

impl<T: SizedSchema> SizedSchema for Vec<T> {
    fn size(&self) -> usize {
        self.iter().map(SizedSchema::size).sum()
    }
}
