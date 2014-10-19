#[phase(plugin, link)] extern crate log;
extern crate std;

use std::io;
use std::io::fs::PathExtensions;
use std::str;

const MESSAGE_SIZE: uint = 4096;
const LINE_END: u8 = '\n' as u8;

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
        let mut abundant_bytes: Vec < u8 > = range(0, MESSAGE_SIZE - bytes.len()).map(|_| { 0 }).collect();
        abundant_bytes[0] = '\n' as u8;

        match file.write(bytes) {
            Ok(_) => {
                match file.write(abundant_bytes.as_slice()) {
                    Ok(_) => {},
                    Err(err) => self.error("Unable to push message", &err)
                }
            },
            Err(err) => self.error("Unable to push message", &err)
        }
    }

    pub fn create_unless_exists(&self) {
        match self.path.exists() {
            false => {
                match io::File::create(&self.path) {
                    Ok(_) => {},
                    Err(err) => self.error("Unable to create river", &err)
                }
            }
            _ => {}
        }
    }

    pub fn peek(&self) -> Option < PeekResult > {
        match self.get_line(None) {
            Some((offset, message)) => self.form_peek_result(message, offset, None),
            _ => None
        }
    }

    pub fn peek_at(&self, offset: uint) -> Option < PeekResult > {
        match self.get_line(Some(offset)) {
            Some((actual_offset, message)) => {
                self.form_peek_result(message, actual_offset, Some(offset))
            }
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

        let size = match self.path.stat() {
            Ok(stat) => (stat.size.to_uint().unwrap() + MESSAGE_SIZE - 1) / MESSAGE_SIZE,
            Err(err) => { self.error("unable to get size of river", &err); 0 }
        };

        let adjusted_offset = match offset {
            Some(offset) => std::cmp::min(size, offset),
            None => size
        };

        if adjusted_offset == 0 {
            return None;
        }

        let actual_offset = adjusted_offset - 1;

        match file.seek((actual_offset * MESSAGE_SIZE).to_i64().unwrap(), io::SeekSet) {
            Ok(_) => match self.read_line(&mut file) {
                Some(string) => Some((actual_offset, string)),
                _ => None
            },
            _ => None
        }
    }

    fn read_line(&self, file: &mut io::IoResult < io::File >) -> Option < String > {
        let mut buf: Vec < u8 > = vec![];

        loop {
            match file.read_byte() {
                Ok(LINE_END) => break,
                Ok(byte) => {
                    buf.push(byte);
                },
                _ => break
            }
        }

        match str::from_utf8(buf.as_slice()) {
            Some(string) => Some(string.to_string()),
            _ => None
        }
    }

    fn are_offsets_match(&self, actual_offset: uint, offset: Option < uint >) -> bool {
        match offset {
            Some(offset) => actual_offset == offset - 1,
            None => true
        }
    }


    #[allow(unused_variables)]
    fn error(&self, message: &str, err: &std::fmt::Show) {
        ()
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn debug(&self, message: &std::fmt::Show) {
        println!("DEBUG: {}", message);
        ()
    }
}
