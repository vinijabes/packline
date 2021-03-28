use bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::messages::Message;
use crate::{SerializableSchema, SizedSchema};

impl Encoder<Message> for super::FlowCodec {
    type Error = std::io::Error;

    fn encode(&mut self, input: Message, output: &mut BytesMut) -> Result<(), Self::Error> {
        output.reserve(input.size());
        input.serialize(output);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FlowSerializable, FlowSized};

    mod flow {
        pub use crate::codec;
        pub use crate::flow::*;
    }

    #[test]
    fn test_encode_i8_to_bytes() {
        const SIZE: usize = 1;

        let mut buf = BytesMut::with_capacity(SIZE);
        42i8.serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_i16_to_bytes() {
        const SIZE: usize = 2;

        let mut buf = BytesMut::with_capacity(SIZE);
        ((42i16 << 8) + 42i16).serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_i32_to_bytes() {
        const SIZE: usize = 4;

        let mut buf = BytesMut::with_capacity(SIZE);
        ((42i32 << 24) + (42i32 << 16) + (42i32 << 8) + 42i32).serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(4, buf.capacity());
    }

    #[test]
    fn test_encode_i64_to_bytes() {
        const SIZE: usize = 8;

        let mut buf = BytesMut::with_capacity(SIZE);
        ((42i64 << 56)
            + (42i64 << 48)
            + (42i64 << 40)
            + (42i64 << 32)
            + (42i64 << 24)
            + (42i64 << 16)
            + (42i64 << 8)
            + 42i64)
            .serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_u16_to_bytes() {
        const SIZE: usize = 2;

        let mut buf = BytesMut::with_capacity(SIZE);
        ((42u16 << 8) + 42u16).serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_u32_to_bytes() {
        const SIZE: usize = 4;

        let mut buf = BytesMut::with_capacity(SIZE);
        ((42u32 << 24) + (42u32 << 16) + (42u32 << 8) + 42u32).serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_u64_to_bytes() {
        const SIZE: usize = 8;

        let mut buf = BytesMut::with_capacity(SIZE);
        ((42u64 << 56)
            + (42u64 << 48)
            + (42u64 << 40)
            + (42u64 << 32)
            + (42u64 << 24)
            + (42u64 << 16)
            + (42u64 << 8)
            + 42u64)
            .serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![42u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_f32_to_bytes() {
        const SIZE: usize = 4;

        let mut buf = BytesMut::with_capacity(SIZE);
        0f32.serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![0u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_f64_to_bytes() {
        const SIZE: usize = 8;

        let mut buf = BytesMut::with_capacity(SIZE);
        0f64.serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(vec![0u8; SIZE], result);
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_string_to_bytes() {
        const SIZE: usize = 16;

        let input = "packline".to_string();
        let mut buf = BytesMut::with_capacity(SIZE);
        input.serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(
            [
                vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8,].as_slice(),
                input.as_bytes()
            ]
            .concat(),
            result
        );
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_vec_to_bytes() {
        const SIZE: usize = 12;

        let mut buf = BytesMut::with_capacity(SIZE);
        vec![2i8, 4i8, 8i8, 16i8].serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(
            vec![
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, //Vec length
                2u8, 4u8, 8u8, 16u8 //Vec values
            ],
            result
        );
        assert_eq!(SIZE, buf.capacity());
    }

    #[test]
    fn test_encode_struct_to_bytes() {
        #[derive(FlowSerializable, FlowSized)]
        struct TestingData {
            x: i8,
            y: i16,
            z: i32,
        }

        const SIZE: usize = 7;
        let mut buf = BytesMut::with_capacity(SIZE);

        TestingData {
            x: 42i8,
            y: 42i16,
            z: 42i32,
        }
        .serialize(&mut buf);

        let mut result = vec![0u8; SIZE];
        result.copy_from_slice(&buf);

        assert_eq!(
            vec![
                42u8, //x value
                0u8, 42u8, //y value
                0u8, 0u8, 0u8, 42u8 //z value
            ],
            result
        );
        assert_eq!(SIZE, buf.capacity());
    }
}
