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
        Ok((duration, attr)) => {
            assert!(duration.as_secs() > 0);
            assert!(attr.ino == FUSE_ROOT_ID);
        },
        Err(err) => panic!("getattr returned {err}"),
    }
}
