use crate::MIME;
use fnv::FnvHashMap;

pub fn get_supported() -> Vec<MIME> {
    super::TYPES.to_vec()
}

/// Returns Vec of parent->child relations
pub fn get_subclasses() -> Vec<(MIME, MIME)> {
    vec![
        ("all/all", "all/allfiles"),
        ("all/all", "inode/directory"),
        ("all/allfiles", "application/octet-stream"),
        ("application/octet-stream", "text/plain"),
    ]
}

pub fn get_aliaslist() -> FnvHashMap<MIME, MIME> {
    FnvHashMap::default()
}
