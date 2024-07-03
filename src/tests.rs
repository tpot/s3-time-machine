use std::ffi::OsStr;

use fuser::FUSE_ROOT_ID;

use crate::s3tmfs::S3TMFS;
use crate::wrapperfs::WrappedFilesystem;

fn make_fs() -> S3TMFS {
    let mut fs = S3TMFS::new();
    fs.fuse_init().unwrap();
    return fs;
}

#[test]
fn fuse_init_and_destroy() {
    let mut fs = S3TMFS::new();
    fs.fuse_init().unwrap();
    fs.fuse_destroy();
}

#[test]
fn fuse_getattr_root() {
    let mut fs = make_fs();
    match fs.fuse_getattr(FUSE_ROOT_ID) {
        Ok(ra) => {
            assert!(ra.duration.as_secs() > 0);
            assert!(ra.attr.ino == FUSE_ROOT_ID);
        },
        Err(err) => panic!("getattr returned {err}"),
    }
}

#[test]
fn fuse_create() {
    let mut fs = make_fs();
    match fs.fuse_create(FUSE_ROOT_ID, OsStr::new("foo"), 0, 0, 0) {
        Ok(rc) => {
            assert!(rc.attr.ino > FUSE_ROOT_ID);
            assert!(rc.attr.size == 0);
            assert!(rc.attr.blocks == 0);
        },
        Err(err) => panic!("create returned errno {err}"),
    }
}

#[test]
fn fuse_create_getattr() {
    let mut fs = make_fs();
    let result = fs.fuse_create(FUSE_ROOT_ID, OsStr::new("foo"), 0, 0, 0).unwrap();
    match fs.fuse_getattr(result.attr.ino) {
        Ok(ra) => {
            assert!(ra.duration.as_secs() > 0);
            assert!(ra.attr.ino == result.attr.ino);
        },
        Err(err) => panic!("getattr returned errno {err}"),
    }
}
