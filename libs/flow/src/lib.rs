pub use flow_derive::*;

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

pub trait Schema {
    fn deserialize() -> Self;
    fn serialize();

    fn size(&self) -> usize;
}

impl Schema for i8 {
    #[inline(always)]
    fn deserialize() -> Self {
        10i8
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<i8>()
    }
}

impl Schema for i16 {
    #[inline(always)]
    fn deserialize() -> Self {
        10i16
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<i16>()
    }
}

impl Schema for i32 {
    #[inline(always)]
    fn deserialize() -> Self {
        10i32
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<i32>()
    }
}

impl Schema for i64 {
    #[inline(always)]
    fn deserialize() -> Self {
        10i64
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl Schema for u16 {
    #[inline(always)]
    fn deserialize() -> Self {
        10u16
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<u16>()
    }
}

impl Schema for u32 {
    #[inline(always)]
    fn deserialize() -> Self {
        10u32
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<u32>()
    }
}

impl Schema for u64 {
    #[inline(always)]
    fn deserialize() -> Self {
        10u64
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<u64>()
    }
}

impl Schema for f32 {
    #[inline(always)]
    fn deserialize() -> Self {
        10f32
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<f32>()
    }
}

impl Schema for f64 {
    #[inline(always)]
    fn deserialize() -> Self {
        10f64
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<f64>()
    }
}

impl Schema for String {
    #[inline(always)]
    fn deserialize() -> Self {
        String::new()
    }
    fn serialize() {}

    fn size(&self) -> usize {
        std::mem::size_of::<char>() * self.chars().count()
    }
}

impl<T: Schema> Schema for Vec<T> {
    #[inline(always)]
    fn deserialize() -> Self {
        Vec::new()
    }
    fn serialize() {}

    fn size(&self) -> usize {
        self.iter().map(Schema::size).sum()
    }
}
