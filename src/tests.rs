use crate::s3tmfs::S3TMFS;
use crate::wrapperfs::WrappedFilesystem;

#[test]
fn fuse_init_and_destroy() {
    let mut fs = S3TMFS::new();
    fs.fuse_init().unwrap();
    fs.fuse_destroy();
}
