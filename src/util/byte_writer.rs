use crate::model::{Arg, ByteString, Function};

pub struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    pub fn push_byte(&mut self, byte: u8) -> &mut Self {
        self.buf.push(byte);
        self
    }

    pub fn push_bytes(&mut self, bytes: &mut Vec<u8>) -> &mut Self {
        self.buf.append(bytes);
        self
    }

    pub fn push_int(&mut self, int: i32) -> &mut Self {
        let mut int_bytes: Vec<u8> = int.to_be_bytes().into();
        self.buf.append(&mut int_bytes);
        self
    }

    pub fn push_long(&mut self, long: i64) -> &mut Self {
        let mut long_bytes: Vec<u8> = long.to_be_bytes().into();
        self.buf.append(&mut long_bytes);
        self
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.buf.clone()
    }

    pub fn bytes_from_function(function: &Function) -> Vec<u8> {
        let mut byte_writer = ByteWriter::new();
        if function.is_default() {
            return byte_writer.push_byte(0).bytes();
        } else {
            byte_writer
                .push_bytes(&mut vec![1, 9, 1])
                .push_int(function.name().len() as i32)
                .push_bytes(&mut function.name().into_bytes())
                .push_func_args(function.args())
                .bytes()
        }
    }

    pub fn push_func_args(&mut self, args: Vec<Arg>) -> &mut Self {
        self.push_int(args.len() as i32);
        for arg in args {
            match arg {
                Arg::Integer(integer) => {
                    self.push_byte(0).push_long(integer);
                }
                Arg::Binary(binary) => {
                    let bytes_len = binary.bytes().len() as i32;
                    self.push_byte(1)
                        .push_int(bytes_len)
                        .push_bytes(&mut binary.bytes());
                }
                Arg::String(string) => {
                    let mut string_bytes = string.into_bytes();
                    self.push_byte(2)
                        .push_int(string_bytes.len() as i32)
                        .push_bytes(&mut string_bytes);
                }
                Arg::Boolean(boolean) => {
                    if boolean {
                        self.push_byte(6);
                    } else {
                        self.push_byte(7);
                    }
                }
                Arg::List(list) => {
                    self.push_byte(11).push_func_args(list);
                }
            }
        }
        self
    }
}

impl Default for ByteWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Arg, Base64String, Function};
    use crate::util::ByteWriter;

    #[test]
    fn test_push_int() {
        assert_eq!(
            vec![0, 18, 213, 59],
            ByteWriter::new().push_int(1234235).bytes()
        );
    }

    #[test]
    fn test_push_long() {
        assert_eq!(
            vec![0, 0, 0, 0, 0, 18, 213, 59],
            ByteWriter::new().push_long(1234235).bytes()
        );
    }

    #[test]
    fn test_bytes_from_function() {
        let function = Function::new(
            "storeData".to_owned(),
            vec![
                Arg::Boolean(true),
                Arg::String("some string".to_owned()),
                Arg::Integer(123),
                Arg::Binary(Base64String::from_bytes(vec![3, 5, 2, 11, 15])),
                Arg::List(vec![Arg::Integer(123), Arg::Integer(543)]),
            ],
        );

        let function_bytes = ByteWriter::bytes_from_function(&function);
        assert_eq!(
            vec![
                1, 9, 1, 0, 0, 0, 9, 115, 116, 111, 114, 101, 68, 97, 116, 97, 0, 0, 0, 5, 6, 2, 0,
                0, 0, 11, 115, 111, 109, 101, 32, 115, 116, 114, 105, 110, 103, 0, 0, 0, 0, 0, 0,
                0, 0, 123, 1, 0, 0, 0, 5, 3, 5, 2, 11, 15, 11, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
                123, 0, 0, 0, 0, 0, 0, 0, 2, 31
            ],
            function_bytes
        )
    }
}
