use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};

use libc::ENOENT;

use fuser::{
    FileAttr, FileType, Filesystem, ReplyEmpty, FUSE_ROOT_ID
};

// Default TTL value
const TTL: Duration = Duration::from_secs(1); // 1 second

// Root directory inode attribute
const ROOT_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

// Our filesystem
pub struct S3TMFS {
    next_inode: u64,
    inode_map: HashMap<u64, FileAttr>,
    name_map: HashMap<String, u64>,
}

impl S3TMFS {
    pub fn new() -> S3TMFS {
        let next_inode = FUSE_ROOT_ID + 1;

        let mut inode_map = HashMap::new();
        inode_map.insert(FUSE_ROOT_ID, ROOT_DIR_ATTR);

        let mut name_map = HashMap::new();
        name_map.insert(".".to_string(), FUSE_ROOT_ID);

        S3TMFS {
            next_inode,
            inode_map,
            name_map,
         }
    }
}

impl Filesystem for S3TMFS {

    fn init(&mut self, _req: &fuser::Request<'_>, _config: &mut fuser::KernelConfig) -> Result<(), libc::c_int> {
        println!(">>> init");
        Ok(())
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        println!(">>> getattr ino={ino}");
        match self.inode_map.get(&ino) {
            Some(attr) => reply.attr(&TTL, attr),
            None => reply.error(ENOENT),
        }
    }

    fn lookup(&mut self, _req: &fuser::Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: fuser::ReplyEntry) {
        println!(">>> lookup parent={parent} name={}", name.to_str().unwrap());
        reply.error(ENOENT);
    }

    fn create(
            &mut self,
            _req: &fuser::Request<'_>,
            parent: u64,
            name: &std::ffi::OsStr,
            _mode: u32,
            _umask: u32,
            _flags: i32,
            reply: fuser::ReplyCreate,
        ) {
        let name_str = name.to_str().unwrap();
        println!(">>> create parent={parent}, name={}", name_str);

        let attrs: FileAttr = FileAttr {
            ino: self.next_inode,
            size: 0,
            blocks: 0,
            atime: UNIX_EPOCH, // 1970-01-01 00:00:00
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o655,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
            blksize: 512,
        };

        self.inode_map.insert(attrs.ino, attrs);
        self.name_map.insert(name_str.to_string(), attrs.ino);

        self.next_inode = self.next_inode + 1;

        reply.created(&TTL, &attrs, 0, 1, 1);
    }

     fn access(&mut self, _req: &fuser::Request<'_>, ino: u64, mask: i32, reply: fuser::ReplyEmpty) {
         println!(">>> access ino={ino} mask={mask}");

         if self.inode_map.contains_key(&ino) {
            println!("\tok");
            reply.ok();
        } else {
            println!("\tENOENT");
            reply.error(ENOENT);
         }
     }

     fn bmap(&mut self, _req: &fuser::Request<'_>, _ino: u64, _blocksize: u32, _idx: u64, _reply: fuser::ReplyBmap) {
         panic!();
     }

     fn copy_file_range(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino_in: u64,
             _fh_in: u64,
             _offset_in: i64,
             _ino_out: u64,
             _fh_out: u64,
             _offset_out: i64,
             _len: u64,
             _flags: u32,
             _reply: fuser::ReplyWrite,
         ) {
         panic!();
     }

     fn destroy(&mut self) {
        println!(">>> destroy");
     }

     fn exchange(
             &mut self,
             _req: &fuser::Request<'_>,
             _parent: u64,
             _name: &std::ffi::OsStr,
             _newparent: u64,
             _newname: &std::ffi::OsStr,
             _options: u64,
             _reply: ReplyEmpty,
         ) {
         panic!();
     }

     fn fallocate(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _offset: i64,
             _length: i64,
             _mode: i32,
             _reply: ReplyEmpty,
         ) {
         panic!();
     }

     fn flush(&mut self, _req: &fuser::Request<'_>, ino: u64, fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        println!(">>> flush ino={ino} fh={fh}");
        reply.ok();
    }

     fn forget(&mut self, _req: &fuser::Request<'_>, _ino: u64, _nlookup: u64) {
         panic!();
     }

     fn fsync(&mut self, _req: &fuser::Request<'_>, _ino: u64, _fh: u64, _datasync: bool, _reply: ReplyEmpty) {
         panic!();
     }

     fn fsyncdir(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _datasync: bool,
             _reply: ReplyEmpty,
         ) {
         panic!();
     }

     fn getlk(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _lock_owner: u64,
             _start: u64,
             _end: u64,
             _typ: i32,
             _pid: u32,
             _reply: fuser::ReplyLock,
         ) {
         panic!();
     }

     fn getxattr(
             &mut self,
             _req: &fuser::Request<'_>,
             ino: u64,
             name: &std::ffi::OsStr,
             _size: u32,
             reply: fuser::ReplyXattr,
         ) {
            println!(">>> getxattr ino={ino}, name={}", name.to_str().unwrap());
            reply.size(0);
        }

     fn getxtimes(&mut self, _req: &fuser::Request<'_>, _ino: u64, _reply: fuser::ReplyXTimes) {
         panic!();
     }

     fn ioctl(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _flags: u32,
             _cmd: u32,
             _in_data: &[u8],
             _out_size: u32,
             _reply: fuser::ReplyIoctl,
         ) {
         panic!();
     }

     fn link(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _newparent: u64,
             _newname: &std::ffi::OsStr,
             _reply: fuser::ReplyEntry,
         ) {
         panic!();
     }

     fn listxattr(&mut self, _req: &fuser::Request<'_>, _ino: u64,_sizee: u32, _reply: fuser::ReplyXattr) {
         panic!();
     }

     fn lseek(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _offset: i64,
             _whence: i32,
             _reply: fuser::ReplyLseek,
         ) {
         panic!();
     }

     fn mkdir(
             &mut self,
             _req: &fuser::Request<'_>,
             _parent: u64,
             _name: &std::ffi::OsStr,
             _mode: u32,
             _umask: u32,
             _reply: fuser::ReplyEntry,
         ) {
         panic!();
     }

     fn mknod(
             &mut self,
             _req: &fuser::Request<'_>,
             _parent: u64,
             _name: &std::ffi::OsStr,
             _mode: u32,
             _umask: u32,
             _rdev: u32,
             _reply: fuser::ReplyEntry,
         ) {
         panic!();
     }

     fn open(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, _reply: fuser::ReplyOpen) {
         panic!();
     }

     fn opendir(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, _reply: fuser::ReplyOpen) {
         panic!();
     }

     fn read(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _offset: i64,
             _size: u32,
             _flags: i32,
             _lock_owner: Option<u64>,
             _reply: fuser::ReplyData,
         ) {
         panic!();
     }

     fn readdir(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _offset: i64,
             _reply: fuser::ReplyDirectory,
         ) {
         panic!();
     }

     fn readdirplus(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _offset: i64,
             _reply: fuser::ReplyDirectoryPlus,
         ) {
         panic!();
     }

     fn readlink(&mut self, _req: &fuser::Request<'_>, _ino: u64,_replyy: fuser::ReplyData) {
         panic!();
     }

     fn release(
             &mut self,
             _req: &fuser::Request<'_>,
             ino: u64,
             fh: u64,
             _flags: i32,
             _lock_owner: Option<u64>,
             _flush: bool,
             reply: ReplyEmpty,
         ) {
            println!(">>> release ino={ino}, fh={fh}");
            reply.ok();
        }

     fn releasedir(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _flags: i32,
             _reply: ReplyEmpty,
         ) {
         panic!();
     }

     fn removexattr(&mut self, _req: &fuser::Request<'_>, _ino: u64, _name: &std::ffi::OsStr, _reply: ReplyEmpty) {
         panic!();
     }

     fn rename(
             &mut self,
             _req: &fuser::Request<'_>,
             _parent: u64,
             _name: &std::ffi::OsStr,
             _newparent: u64,
             _newname: &std::ffi::OsStr,
             _flags: u32,
             _reply: ReplyEmpty,
         ) {
         panic!();
     }

     fn rmdir(&mut self, _req: &fuser::Request<'_>, _parent: u64, _name: &std::ffi::OsStr, _reply: ReplyEmpty) {
         panic!();
     }

     fn setattr(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _mode: Option<u32>,
             _uid: Option<u32>,
             _gid: Option<u32>,
             _size: Option<u64>,
             _atime: Option<fuser::TimeOrNow>,
             _mtime: Option<fuser::TimeOrNow>,
             _ctime: Option<std::time::SystemTime>,
             _fh: Option<u64>,
             _crtime: Option<std::time::SystemTime>,
             _chgtime: Option<std::time::SystemTime>,
             _bkuptime: Option<std::time::SystemTime>,
             _flags: Option<u32>,
             _reply: fuser::ReplyAttr,
         ) {
         panic!();
     }

     fn setlk(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _lock_owner: u64,
             _start: u64,
             _end: u64,
             _typ: i32,
             _pid: u32,
             _sleep: bool,
             _reply: ReplyEmpty,
         ) {
         panic!();
     }

     fn setvolname(&mut self, _req: &fuser::Request<'_>, _name: &std::ffi::OsStr, _reply: ReplyEmpty) {
         panic!();
     }

     fn setxattr(
             &mut self,
             _req: &fuser::Request<'_>,
             ino: u64,
             name: &std::ffi::OsStr,
             _value: &[u8],
             _flags: i32,
             _position: u32,
             reply: ReplyEmpty,
         ) {
            println!(">>> setxattr ino={ino}, name={}", name.to_str().unwrap());
            reply.ok();
     }

     fn statfs(&mut self, _req: &fuser::Request<'_>, _ino: u64, reply: fuser::ReplyStatfs) {
        println!(">>> statfs");
        reply.statfs(100000, 50000, 50000, 1000, 1000, 4096, 90, 100);
     }

     fn symlink(
             &mut self,
             _req: &fuser::Request<'_>,
             _parent: u64,
             _link_name: &std::ffi::OsStr,
             _target: &std::path::Path,
             _reply: fuser::ReplyEntry,
         ) {
         panic!();
     }

     fn unlink(&mut self, _req: &fuser::Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: ReplyEmpty) {
        println!(">>> unlink parent={parent}, name={}", name.to_str().unwrap());
        reply.ok();
    }

     fn write(
             &mut self,
             _req: &fuser::Request<'_>,
             _ino: u64,
             _fh: u64,
             _offset: i64,
             _data: &[u8],
             _write_flags: u32,
             _flags: i32,
             _lock_owner: Option<u64>,
             _reply: fuser::ReplyWrite,
         ) {
         panic!();
     }

}
