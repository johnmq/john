extern crate std;

use std::io;
use std::io::fs::PathExtensions;
use std::str;

const MESSAGE_SIZE: uint = 4096;
const LINE_END: u8 = '\n' as u8;

#[deriving(Encodable, Decodable)]
/// Result of PeekCommand, when it was successful
/// Contains message and new offset to specify to peek command
/// for continuous peeking
pub struct PeekResult {
    /// Contains message
    pub message: String,
    /// Contains next offset to be specified to read next message from river
    pub offset: uint
}

impl Clone for PeekResult {
    fn clone(&self) -> PeekResult {
        PeekResult {
            message: self.message.clone(),
            offset: self.offset.clone()
        }
    }

    fn clone_from(&mut self, source: &PeekResult) {
        self.message = source.message.clone();
        self.offset = source.offset.clone();
    }
}

pub struct River {
    path: Path,
    name: String
}

impl River {
    pub fn new(river: &str) -> River {
        River {
            name: river.to_string(),
            path: Path::new(format!("./tmp/rivers/{}", river))
        }
    }

    pub fn push(&self, message: &str) {
        let mut file = self.get_file_for_append();
        let bytes = message.as_bytes();

        match file.write(bytes) {
            Ok(_) => self.write_abundant_bytes(&mut file, bytes.len()),
            Err(err) => self.error("Unable to push message", &err)
        }
    }

    pub fn create_unless_exists(&self) {
        if ! self.path.exists() {
            match io::File::create(&self.path) {
                Ok(_) => {},
                Err(err) => self.error("Unable to create river", &err)
            }
        }
    }

    pub fn peek_at(&self, offset: Option < uint >) -> Option < PeekResult > {
        match self.get_line(offset) {
            Some((actual_offset, message)) => self.form_peek_result(message, actual_offset, offset),
            _ => None
        }
    }

    pub fn destroy(&self) {
        match io::fs::unlink(&self.path) {
            Ok(_) => {},
            Err(err) => self.error(format!("Unable to clear river {}", self.name).as_slice(), &err)
        }
    }

    fn form_peek_result(&self, message: String, actual_offset: uint, offset: Option < uint >) -> Option < PeekResult > {
        match self.are_offsets_match(actual_offset, offset) {
            true => Some(PeekResult {
                message: message.replace("\n", ""),
                offset: actual_offset + 2
            }),
            false => None
        }
    }

    fn get_file_for_append(&self) -> io::IoResult < io::File > {
        self.create_unless_exists();
        io::File::open_mode(&self.path, io::Append, io::Write)
    }

    fn get_file_for_peek(&self) -> io::IoResult < io::File > {
        self.create_unless_exists();
        io::File::open(&self.path)
    }

    fn get_line(&self, offset: Option < uint >) -> Option < (uint, String) > {
        let mut file = self.get_file_for_peek();

        match self.seek_and_read_line(&mut file, offset) {
            Some(string) => Some((self.get_actual_offset(offset).unwrap(), string)),
            _ => None
        }
    }

    fn read_line(&self, file: &mut io::IoResult < io::File >) -> Option < String > {
        let mut buf: Vec < u8 > = vec![];

        self.read_to_buf_until_eol(file, &mut buf);
        self.convert_buf_to_string(buf)
    }

    fn are_offsets_match(&self, actual_offset: uint, offset: Option < uint >) -> bool {
        match offset {
            Some(offset) => actual_offset == offset - 1,
            None => true
        }
    }

    fn abundant_bytes(&self, count: uint) -> Vec < u8 > {
        let mut abundant_bytes: Vec < u8 > = range(0, MESSAGE_SIZE - count).map(|_| { 0 }).collect();
        abundant_bytes[0] = '\n' as u8;
        abundant_bytes
    }

    fn write_abundant_bytes(&self, file: &mut io::IoResult < io::File >, count: uint) {
        match file.write(self.abundant_bytes(count).as_slice()) {
            Ok(_) => {},
            Err(err) => self.error("Unable to push message", &err)
        }
    }

    fn size(&self) -> uint {
        match self.path.stat() {
            Ok(stat) => (stat.size.to_uint().unwrap() + MESSAGE_SIZE - 1) / MESSAGE_SIZE,
            Err(err) => { self.error("unable to get size of river", &err); 0 }
        }
    }

    fn get_actual_offset(&self, offset: Option < uint >) -> Option < uint > {
        let size = self.size();
        let adjusted_offset = match offset {
            Some(offset) => std::cmp::min(size, offset),
            None => size
        };

        match adjusted_offset {
            0 => None,
            offset => Some(offset - 1)
        }
    }

    fn get_seek_offset(&self, offset: Option < uint >) -> Option < i64 > {
        match self.get_actual_offset(offset) {
            Some(offset) => match (offset * MESSAGE_SIZE).to_i64() {
                Some(seek_offset) => Some(seek_offset),
                _ => None
            },
            _ => None
        }
    }

    fn seek_and_read_line(&self, file: &mut io::IoResult < io::File >, offset: Option < uint >) -> Option < String > {
        match self.get_seek_offset(offset) {
            Some(seek_offset) => match file.seek(seek_offset, io::SeekSet) {
                Ok(_) => self.read_line(file),
                _ => None
            },
            _ => None
        }
    }

    fn read_to_buf_until_eol(&self, file: &mut io::IoResult < io::File >, buf: &mut Vec < u8 >) {
        loop {
            if ! self.read_one_byte_to_buf(file, buf) {
                break
            }
        }
    }

    fn read_one_byte_to_buf(&self, file: &mut io::IoResult < io::File >, buf: &mut Vec < u8 >) -> bool {
        match file.read_byte() {
            Ok(LINE_END) => false,
            Ok(byte) => {
                buf.push(byte);
                true
            },
            _ => false
        }
    }

    fn convert_buf_to_string(&self, buf: Vec < u8 >) -> Option < String > {
        match str::from_utf8(buf.as_slice()) {
            Some(string) => Some(string.to_string()),
            _ => None
        }
    }

    #[allow(unused_variables)]
    fn error(&self, message: &str, err: &std::fmt::Show) {
        ()
    }
}
