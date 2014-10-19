#[phase(plugin, link)] extern crate log;
extern crate std;

use std::io;
use std::io::fs::PathExtensions;

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
        match self.get_file_for_append().write_line(message) {
            Ok(_) => {},
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
        match self.get_file_for_peek().lines().enumerate().last() {
            Some((offset, Ok(message))) => self.form_peek_result(message, offset, None),
            _ => None
        }
    }

    pub fn peek_at(&self, offset: uint) -> Option < PeekResult > {
        match self.get_file_for_peek().lines().take(offset).enumerate().last() {
            Some((actual_offset, Ok(message))) => {
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

    fn get_file_for_peek(&self) -> io::BufferedReader < io::IoResult < io::File > > {
        self.create_unless_exists();
        io::BufferedReader::new(io::File::open(&self.path))
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
}
