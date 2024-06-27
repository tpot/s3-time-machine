pub mod s3tmfs;
pub mod wrapperfs;

use crate::s3tmfs::S3TMFS;

use fuser::MountOption;
use clap::{arg, Command};

fn main() {

    // Command line options
    let matches = Command::new("s3-time-machine")
        .arg(arg!(--mountpoint <DIR>).required(true))
        .get_matches();

    // Get mount point directory
    let mountpoint = matches.get_one::<String>("mountpoint").unwrap();

    // Mount filesystem
    let mut options = vec![
        MountOption::AutoUnmount,
        MountOption::RW,
        MountOption::FSName("s3-tm".to_string())];
    options.push(MountOption::AutoUnmount);

    let fs = S3TMFS::new();
    fuser::mount2(fs, mountpoint, &options).unwrap();
}
