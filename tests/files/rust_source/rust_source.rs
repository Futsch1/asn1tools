//! This file was generated by asn1tools version 0.145.2 Tue Jan 22 22:09:47 2019.

struct Encoder<'a> {
    buf: &'a mut [u8],
    size: usize,
    pos: usize,
    error: Option<&'static str>
}

struct Decoder<'a> {
    buf: &'a[u8],
    size: usize,
    pos: usize,
    error: Option<&'static str>
}
impl<'a> Encoder<'a> {

    fn new(dst: &'a mut [u8]) -> Encoder {
        Encoder {
            size: 8 * dst.len() as isize,
            buf: dst,
            pos: 0
        }
    }

    fn get_result(&self) -> Result<usize, &'static str> {
        if self.error.is_none() {
            return Ok((self.pos + 7) / 8);
        } else {
            return Err(self.error.unwrap());
        }
    }


    fn append_bytes(&mut self, buf: &[u8]) {
        if let Ok(pos) = self.alloc(8 * buf.len()) {
            let byte_pos = pos / 8;
            let pos_in_byte = pos % 8;

            if pos_in_byte == 0 {
                self.buf.get_mut(byte_pos..byte_pos + buf.len())
                    .unwrap()
                    .copy_from_slice(buf.get(0..buf.len()).unwrap());
            } else {
                for i in 0..buf.len() {
                    self.buf[byte_pos + i] |= buf[i] >> pos_in_byte;
                    self.buf[byte_pos + i + 1] = buf[i] << (8 - pos_in_byte);
                }
            }
        }
    }

    fn append_u8(&mut self, value: u8) {
        self.append_bytes(&[value]);
    }

    fn append_u16(&mut self, value: u16) {
        self.append_bytes(&value.to_be_bytes());
    }

    fn append_u32(&mut self, value: u32) {
        self.append_bytes(&value.to_be_bytes());
    }

    fn append_u64(&mut self, value: u64) {
        self.append_bytes(&value.to_be_bytes());
    }

    fn append_i8(&mut self, value: i8) {
        self.append_u8((value as u8).wrapping_add(128));
    }

    fn append_i16(&mut self, value: i16) {
        self.append_u16((value as u16).wrapping_add(32768));
    }

    fn append_i32(&mut self, value: i32) {
        self.append_u32((value as u32).wrapping_add(2147483648));
    }

    fn append_i64(&mut self, value: i64) {
        self.append_u64((value as u64).wrapping_add(9223372036854775808));
    }

    fn append_bool(&mut self, value: bool) {
        self.append_bit(value as u8);
    }
}

impl<'a> Decoder<'a> {

    fn new(src: &'a[u8]) -> Decoder {
        Decoder {
            buf: src,
            size: 8 * src.len() as isize,
            pos: 0
        }
    }

    fn get_result(&self) -> Result<usize, &'static str> {
        if self.error.is_none() {
            Ok((self.pos + 7) / 8)
        } else {
            Err(self.error.unwrap())
        }
    }

    fn read_bytes(&mut self, buf: &mut [u8]) {
        if let Ok(pos) = self.free(8 * buf.len()) {
            let byte_pos = pos / 8;
            let pos_in_byte = pos % 8;

            if pos_in_byte == 0 {
                buf.copy_from_slice(
                    self.buf.get(byte_pos..byte_pos + buf.len()).unwrap());
            } else {
                for i in 0..buf.len() {
                    buf[i] = self.buf[byte_pos + i] << pos_in_byte;
                    buf[i] |= self.buf[byte_pos + i + 1] >> (8 - pos_in_byte);
                }
            }
        }
    }

    fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];

        self.read_bytes(&mut buf, 1);

        return u8::from_be_bytes(buf);
    }

    fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];

        self.read_bytes(&mut buf, 2);

        return u16::from_be_bytes(buf);
    }

    fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];

        self.read_bytes(&mut buf, 4);

        return u32::from_be_bytes(buf);
    }

    fn read_u64(&mut self) -> u64 {
        let mut buf = [0; 8];

        self.read_bytes(&mut buf, 8);

        return u64::from_be_bytes(buf);
    }

    fn read_i8(&mut self) -> i8 {
        return self.read_u8().wrapping_sub(128) as i8;
    }

    fn read_i16(&mut self) -> i16 {
        return self.read_u16().wrapping_sub(32768) as i16;
    }

    fn read_i32(&mut self) -> i32 {
        return self.read_u32().wrapping_sub(2147483648) as i32;
    }

    fn read_i64(&mut self) -> i64 {
        return self.read_u64().wrapping_sub(9223372036854775808) as i64;
    }

    fn read_bool(&mut self) -> bool {
        return self.read_bit() != 0;
    }
}


// Type A in module RustSource.
struct RustSourceA {
    a: i8;
    b: i16;
    c: i32;
    d: i64;
    e: u8;
    f: u16;
    g: u32;
    h: u64;
    i: bool;
    struct {
        uint8_t buf[11];
    j: };
};

impl RustSourceA {

    pub fn to_bytes(&mut self,
                    dst: &mut [u8])
                    -> Result<usize, &'static str> {
        let mut encoder = Encoder::new(&mut dst);

        self.to_bytes_inner(&mut encoder);

        return encoder.get_result();
    }

    pub fn from_bytes(&mut self, src: &[u8]) -> Result<usize, &'static str> {
        let mut decoder = Decoder::new(&src);

        self.from_bytes_inner(&mut decoder);

        return decoder.get_result();
    }

    fn to_bytes_inner(&mut self, encoder: &mut Encoder) {
        encoder.append_i8(self.a);
        encoder.append_i16(self.b);
        encoder.append_i32(self.c);
        encoder.append_i64(self.d);
        encoder.append_u8(self.e);
        encoder.append_u16(self.f);
        encoder.append_u32(self.g);
        encoder.append_u64(self.h);
        encoder.append_bool(self.i);
        encoder.append_bytes(&self.j.buf,
                             11);
    }

    fn from_bytes_inner(&mut self, decoder: &mut Decoder) {
        self.a = decoder.read_i8();
        self.b = decoder.read_i16();
        self.c = decoder.read_i32();
        self.d = decoder.read_i64();
        self.e = decoder.read_u8();
        self.f = decoder.read_u16();
        self.g = decoder.read_u32();
        self.h = decoder.read_u64();
        self.i = decoder.read_bool();
        decoder.read_bytes(&mut self.j.buf,
                           11);
    }
}
