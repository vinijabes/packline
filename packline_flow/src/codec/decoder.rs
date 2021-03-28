use bytes::BytesMut;
use tokio_util::codec::Decoder;

impl Decoder for super::FlowCodec {
    type Item = crate::messages::Message;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut decoder = ByteDecoder { offset: 0, buf: src };

        let mut x = vec![0u8; 4];
        let buf = decoder.next(4);
        x[..4].clone_from_slice(buf);
        println!("{}", String::from_utf8(x).unwrap());
        Ok(None)
    }
}

pub struct ByteDecoder<'a> {
    offset: usize,
    buf: &'a [u8],
}

impl<'a> ByteDecoder<'a> {
    fn new(src: &'a [u8]) -> ByteDecoder<'a> {
        ByteDecoder { offset: 0, buf: src }
    }

    pub fn next(&mut self, size: usize) -> &[u8] {
        let result = &self.buf[self.offset..self.offset + size];
        self.offset += size;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::DeserializableSchema;

    #[test]
    fn test_decode_i8_from_bytes() {
        let buf = vec![42u8; 1];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(42i8, i8::deserialize(&mut decoder).unwrap())
    }

    #[test]
    fn test_decode_i16_from_bytes() {
        let buf = vec![42u8; 2];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!((42i16 << 8) + 42i16, i16::deserialize(&mut decoder).unwrap())
    }

    #[test]
    fn test_decode_i32_from_bytes() {
        let buf = vec![42u8; 4];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(
            (42i32 << 24) + (42i32 << 16) + (42i32 << 8) + 42i32,
            i32::deserialize(&mut decoder).unwrap()
        )
    }

    #[test]
    fn test_decode_i64_from_bytes() {
        let buf = vec![42u8; 8];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(
            (42i64 << 56)
                + (42i64 << 48)
                + (42i64 << 40)
                + (42i64 << 32)
                + (42i64 << 24)
                + (42i64 << 16)
                + (42i64 << 8)
                + 42i64,
            i64::deserialize(&mut decoder).unwrap()
        )
    }

    #[test]
    fn test_decode_u16_from_bytes() {
        let buf = vec![42u8; 2];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!((42u16 << 8) + 42u16, u16::deserialize(&mut decoder).unwrap())
    }

    #[test]
    fn test_decode_u32_from_bytes() {
        let buf = vec![42u8; 4];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(
            (42u32 << 24) + (42u32 << 16) + (42u32 << 8) + 42u32,
            u32::deserialize(&mut decoder).unwrap()
        )
    }

    #[test]
    fn test_decode_u64_from_bytes() {
        let buf = vec![42u8; 8];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(
            (42u64 << 56)
                + (42u64 << 48)
                + (42u64 << 40)
                + (42u64 << 32)
                + (42u64 << 24)
                + (42u64 << 16)
                + (42u64 << 8)
                + 42u64,
            u64::deserialize(&mut decoder).unwrap()
        )
    }

    #[test]
    fn test_decode_f32_from_bytes() {
        let buf = vec![0u8; 4];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(0f32, f32::deserialize(&mut decoder).unwrap())
    }

    #[test]
    fn test_decode_f64_from_bytes() {
        let buf = vec![0u8; 8];
        let mut decoder = ByteDecoder::new(buf.as_slice());
        assert_eq!(0f64, f64::deserialize(&mut decoder).unwrap())
    }

    #[test]
    fn test_decode_string_from_bytes() {
        let input = "packline";
        let len = input.len();

        let str_bytes = input.as_bytes();
        let len_bytes = i64::to_be_bytes(len as i64);
        let concat_vec = [&len_bytes[0..8], &str_bytes[0..len]].concat();

        let buf = concat_vec.as_slice();

        let mut decoder = ByteDecoder::new(buf);
        assert_eq!("packline", String::deserialize(&mut decoder).unwrap())
    }

    #[test]
    fn test_decode_vec_from_bytes() {
        let buf = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, //Vec length
            42u8, 42u8, 42u8, 42u8, //Vec values
        ];

        let mut decoder = ByteDecoder::new(&buf);
        let result = Vec::<i8>::deserialize(&mut decoder).unwrap();

        assert_eq!(vec![42i8, 42i8, 42i8, 42i8], result)
    }
}
