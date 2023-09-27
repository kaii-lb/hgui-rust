use crate::{read_bytes, MIME};
use fnv::FnvHashMap;
use std::path::Path;

pub(crate) struct BaseType;

impl crate::Checker for BaseType {
    fn from_u8(&self, file: &[u8], mimetype: &str) -> bool {
        from_u8(file, mimetype)
    }

    fn from_filepath(&self, filepath: &Path, mimetype: &str) -> bool {
        from_filepath(filepath, mimetype)
    }

    fn get_supported(&self) -> Vec<MIME> {
        super::init::get_supported()
    }

    fn get_subclasses(&self) -> Vec<(MIME, MIME)> {
        super::init::get_subclasses()
    }

    fn get_aliaslist(&self) -> FnvHashMap<MIME, MIME> {
        super::init::get_aliaslist()
    }
}

/// If there are any null bytes, return False. Otherwise return True.
fn is_text_plain_from_u8(b: &[u8]) -> bool {
    bytecount::count(b, 0) == 0
}

// TODO: Hoist the main logic here somewhere else. This'll get redundant fast!
fn is_text_plain_from_filepath(filepath: &Path) -> bool {
    let b = match read_bytes(filepath, 512) {
        Ok(x) => x,
        Err(_) => return false,
    };
    is_text_plain_from_u8(b.as_slice())
}

#[allow(unused_variables)]
pub fn from_u8(b: &[u8], mimetype: &str) -> bool {
    if mimetype == "application/octet-stream" || mimetype == "all/allfiles" {
        // Both of these are the case if we have a bytestream at all
        return true;
    }
    if mimetype == "text/plain" {
        is_text_plain_from_u8(b)
    } else {
        // ...how did we get bytes for this?
        false
    }
}

pub fn from_filepath(filepath: &Path, mimetype: &str) -> bool {
    use std::fs;

    // Being bad with error handling here,
    // but if you can't open it it's probably not a file.
    let meta = match fs::metadata(filepath) {
        Ok(x) => x,
        Err(_) => {
            return false;
        }
    };

    match mimetype {
        "all/all" => true,
        "all/allfiles" | "application/octet-stream" => meta.is_file(),
        "inode/directory" => meta.is_dir(),
        "text/plain" => is_text_plain_from_filepath(filepath),
        _ => false,
    }
}
