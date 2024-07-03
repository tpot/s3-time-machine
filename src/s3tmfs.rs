use crate::wrapperfs::{
    ReplyAttr, ReplyBmap, ReplyCreate, ReplyData, ReplyDirectory, ReplyDirectoryPlus, ReplyEntry,
    ReplyIoctl, ReplyLock, ReplyLseek, ReplyOpen, ReplyStatfs, ReplyWrite, ReplyXattr,
    WrappedFilesystem,
};

#[cfg(feature = "macos")]
use crate::wrapperfs::ReplyXTimes;

use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};

use libc::ENOENT;

use fuser::{FileAttr, FileType, FUSE_ROOT_ID};

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

// WrappedFilesystem implements Filesystem and exposes request-less interface

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

impl WrappedFilesystem for S3TMFS {
    fn fuse_init(&mut self) -> Result<(), libc::c_int> {
        println!(">>> init");
        Ok(())
    }

    fn fuse_getattr(&mut self, ino: u64) -> Result<ReplyAttr, i32> {
        println!(">>> getattr ino={ino}");

        match self.inode_map.get(&ino) {
            Some(attr) => {
                println!("\tok");
                Ok(ReplyAttr { ttl: &TTL, attr })
            }
            None => {
                println!("\tENOENT");
                Err(ENOENT)
            }
        }
    }

    fn fuse_lookup(&mut self, parent: u64, name: &std::ffi::OsStr) -> Result<ReplyEntry, i32> {
        let name_str = name.to_str().unwrap();
        println!(">>> lookup parent={parent} name={}", name_str);

        match self.name_map.get(name_str) {
            Some(ino) => {
                println!("\tok ino={ino}");
                let attr = self.inode_map.get(ino);
                Ok(ReplyEntry {
                    ttl: &TTL,
                    attr: attr.unwrap(),
                    generation: 1,
                })
            }
            _ => {
                println!("\t ENOENT");
                Err(ENOENT)
            }
        }
    }

    fn fuse_create(
        &mut self,
        parent: u64,
        name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        _flags: i32,
    ) -> Result<ReplyCreate, i32> {
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

        Ok(ReplyCreate {
            ttl: TTL.clone(),
            attr: attrs,
            generation: 0,
            fh: 1,
            flags: 1,
        })
    }

    fn fuse_access(&mut self, ino: u64, mask: i32) -> Result<(), i32> {
        println!(">>> access ino={ino} mask={mask}");

        if self.inode_map.contains_key(&ino) {
            println!("\tok");
            Ok(())
        } else {
            println!("\tENOENT");
            Err(ENOENT)
        }
    }

    fn fuse_bmap(&mut self, _ino: u64, _blocksize: u32, _idx: u64) -> Result<ReplyBmap, i32> {
        panic!();
    }

    fn fuse_copy_file_range(
        &mut self,
        _ino_in: u64,
        _fh_in: u64,
        _offset_in: i64,
        _ino_out: u64,
        _fh_out: u64,
        _offset_out: i64,
        _len: u64,
        _flags: u32,
    ) -> Result<ReplyWrite, i32> {
        panic!();
    }

    fn fuse_destroy(&mut self) {
        println!(">>> destroy");
    }

    #[cfg(feature = "macos")]
    fn fuse_exchange(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _newparent: u64,
        _newname: &std::ffi::OsStr,
        _options: u64,
    ) -> Result<(), i32> {
        panic!();
    }

    fn fuse_fallocate(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _length: i64,
        _mode: i32,
    ) -> Result<(), i32> {
        panic!();
    }

    fn fuse_flush(&mut self, ino: u64, fh: u64, _lock_owner: u64) -> Result<(), i32> {
        println!(">>> flush ino={ino} fh={fh}");

        match self.inode_map.get(&ino) {
            Some(_) => {
                println!("\tok");
                Ok(())
            }
            _ => {
                println!("\tENOENT");
                Err(ENOENT)
            }
        }
    }

    fn fuse_forget(&mut self, ino: u64, _nlookup: u64) {
        println!(">>> forget ino={ino}");
    }

    fn fuse_fsync(&mut self, _ino: u64, _fh: u64, _datasync: bool) -> Result<(), i32> {
        panic!();
    }

    fn fuse_fsyncdir(&mut self, _ino: u64, _fh: u64, _datasync: bool) -> Result<(), i32> {
        panic!();
    }

    fn fuse_getlk(
        &mut self,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: i32,
        _pid: u32,
    ) -> Result<ReplyLock, i32> {
        panic!();
    }

    fn fuse_getxattr(
        &mut self,
        ino: u64,
        name: &std::ffi::OsStr,
        _size: u32,
    ) -> Result<ReplyXattr, i32> {
        println!(">>> getxattr ino={ino}, name={}", name.to_str().unwrap());

        match self.inode_map.get(&ino) {
            Some(_) => {
                println!("\tok");
                Ok(ReplyXattr { size: 0 })
            }
            _ => {
                println!("\tENOENT");
                Err(ENOENT)
            }
        }
    }

    #[cfg(feature = "macos")]
    fn fuse_getxtimes(&mut self, _ino: u64) -> Result<ReplyXTimes, i32> {
        panic!();
    }

    fn fuse_ioctl(
        &mut self,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        _cmd: u32,
        _in_data: &[u8],
        _out_size: u32,
    ) -> Result<ReplyIoctl, i32> {
        panic!();
    }

    fn fuse_link(
        &mut self,
        _ino: u64,
        _newparent: u64,
        _newname: &std::ffi::OsStr,
    ) -> Result<ReplyEntry, i32> {
        panic!();
    }

    fn fuse_listxattr(&mut self, _ino: u64, _sizee: u32) -> Result<ReplyXattr, i32> {
        panic!();
    }

    fn fuse_lseek(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _whence: i32,
    ) -> Result<ReplyLseek, i32> {
        panic!();
    }

    fn fuse_mkdir(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
    ) -> Result<ReplyEntry, i32> {
        panic!();
    }

    fn fuse_mknod(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        _rdev: u32,
    ) -> Result<ReplyEntry, i32> {
        panic!();
    }

    fn fuse_open(&mut self, ino: u64, flags: i32) -> Result<ReplyOpen, i32> {
        println!(">>> TODO: open ino={ino}, flags={flags}");
        Ok(ReplyOpen { fh: 1, flags: 0 })
    }

    fn fuse_opendir(&mut self, _ino: u64, _flags: i32) -> Result<ReplyOpen, i32> {
        panic!();
    }

    fn fuse_read(
        &mut self,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
    ) -> Result<ReplyData, i32> {
        println!(">>> TODO: read ino={ino}, fh={fh}, offset={offset}, size={size}");
        Ok(ReplyData { data: &[] })
    }

    fn fuse_readdir(&mut self, _ino: u64, _fh: u64, _offset: i64) -> Result<ReplyDirectory, i32> {
        panic!();
    }

    fn fuse_readdirplus(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
    ) -> Result<ReplyDirectoryPlus, i32> {
        panic!();
    }

    fn fuse_readlink(&mut self, _ino: u64) -> Result<ReplyData, i32> {
        panic!();
    }

    fn fuse_release(
        &mut self,
        ino: u64,
        fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
    ) -> Result<(), i32> {
        println!(">>> release ino={ino}, fh={fh}");

        match self.inode_map.get(&ino) {
            Some(_) => {
                println!("\tok");
                Ok(())
            }
            _ => {
                println!("\tENOENT");
                Err(ENOENT)
            }
        }
    }

    fn fuse_releasedir(&mut self, _ino: u64, _fh: u64, _flags: i32) -> Result<(), i32> {
        panic!();
    }

    fn fuse_removexattr(&mut self, _ino: u64, _name: &std::ffi::OsStr) -> Result<(), i32> {
        panic!();
    }

    fn fuse_rename(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _newparent: u64,
        _newname: &std::ffi::OsStr,
        _flags: u32,
    ) -> Result<(), i32> {
        panic!();
    }

    fn fuse_rmdir(&mut self, _parent: u64, _name: &std::ffi::OsStr) -> Result<(), i32> {
        panic!();
    }

    fn fuse_setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<fuser::TimeOrNow>,
        mtime: Option<fuser::TimeOrNow>,
        ctime: Option<std::time::SystemTime>,
        fh: Option<u64>,
        crtime: Option<std::time::SystemTime>,
        chgtime: Option<std::time::SystemTime>,
        bkuptime: Option<std::time::SystemTime>,
        flags: Option<u32>,
    ) -> Result<ReplyAttr, i32> {
        println!(">>> TODO: setattr ino={ino}");

        // Look up file
        let opt_attr = self.inode_map.get_mut(&ino);
        if let None = opt_attr {
            return Err(ENOENT);
        }

        // Mutate file attribute
        let attr = opt_attr.unwrap();

        if let Some(mode) = mode {
            println!("\t mode={mode}");
            panic!();
        } else if let Some(uid) = uid {
            println!("\t uid={uid}");
            panic!();
        } else if let Some(gid) = gid {
            println!("\t gid={gid}");
            panic!();
        } else if let Some(size) = size {
            println!("\t size={size}");
            attr.size = size;
        } else if let Some(_) = atime {
            println!("\t atime=?");
            panic!();
        } else if let Some(_) = mtime {
            println!("\t mtime=?");
            panic!();
        } else if let Some(ctime) = ctime {
            println!("\t ctime={}", ctime.elapsed().unwrap().as_millis());
            panic!();
        } else if let Some(fh) = fh {
            println!("\t TODO: fh={fh}");
        } else if let Some(crtime) = crtime {
            println!("\t crtime={}", crtime.elapsed().unwrap().as_millis());
            panic!();
        } else if let Some(chgtime) = chgtime {
            println!("\t chgtime={}", chgtime.elapsed().unwrap().as_millis());
            panic!();
        } else if let Some(bkuptime) = bkuptime {
            println!("\t bkuptime={}", bkuptime.elapsed().unwrap().as_millis());
            panic!();
        } else if let Some(flags) = flags {
            println!("\t flags={flags}");
            panic!();
        } else {
            // Not sure what to do...
            panic!();
        }

        Ok(ReplyAttr { ttl: &TTL, attr })
    }

    fn fuse_setlk(
        &mut self,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: i32,
        _pid: u32,
        _sleep: bool,
    ) -> Result<(), i32> {
        panic!();
    }

    #[cfg(feature = "macos")]
    fn fuse_setvolname(&mut self, _name: &std::ffi::OsStr) -> Result<(), i32> {
        panic!();
    }

    fn fuse_setxattr(
        &mut self,
        ino: u64,
        name: &std::ffi::OsStr,
        _value: &[u8],
        _flags: i32,
        _position: u32,
    ) -> Result<(), i32> {
        println!(">>> setxattr ino={ino}, name={}", name.to_str().unwrap());
        Ok(())
    }

    fn fuse_statfs(&mut self, ino: u64) -> Result<ReplyStatfs, i32> {
        println!(">>> statfs ino={ino}");
        Ok(ReplyStatfs {
            blocks: 100000,
            bfree: 50000,
            bavail: 50000,
            files: 1000,
            ffree: 1000,
            bsize: 4096,
            namelen: 90,
            frsize: 100,
        })
    }

    fn fuse_symlink(
        &mut self,
        _parent: u64,
        _link_name: &std::ffi::OsStr,
        _target: &std::path::Path,
    ) -> Result<ReplyEntry, i32> {
        panic!();
    }

    fn fuse_unlink(&mut self, parent: u64, name: &std::ffi::OsStr) -> Result<(), i32> {
        let name_str = name.to_str().unwrap();
        println!(">>> unlink parent={parent}, name={}", name_str);

        match self.name_map.get(name_str) {
            Some(ino) => {
                println!("\tok ino={ino}");
                self.inode_map.remove(ino);
                self.name_map.remove(name_str);
                Ok(())
            }
            _ => {
                println!("\t ENOENT");
                Err(ENOENT)
            }
        }
    }

    fn fuse_write(
        &mut self,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
    ) -> Result<ReplyWrite, i32> {
        println!(">>> TODO: write ino={ino}, fh={fh}, offset={offset}");
        Ok(ReplyWrite {
            size: data.len().try_into().unwrap(),
        })
    }
}
