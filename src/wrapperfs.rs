use crate::s3tmfs::S3TMFS;

use fuser::{FileAttr, Filesystem, ReplyEmpty};
use std::time::Duration;

// If at some stage the request struct is required, we can define it using an enum.

// struct LocalRequest {}

// enum Request<'a> {
//     Live(&'a fuser::Request<'a>),
//     Test(&'a LocalRequest),
// }

pub struct ReplyAttr<'a> {
    pub ttl: &'a Duration,
    pub attr: &'a FileAttr,
}

pub struct ReplyEntry<'a> {
    pub ttl: &'a Duration,
    pub attr: &'a FileAttr,
    pub generation: u64,
}

pub struct ReplyCreate {
    pub ttl: Duration,
    pub attr: FileAttr,
    pub generation: u64,
    pub fh: u64,
    pub flags: u32,
}

pub trait WrappedFilesystem {
    fn fuse_init(&mut self) -> Result<(), libc::c_int>;
    fn fuse_getattr(&mut self, ino: u64) -> Result<ReplyAttr, i32>;
    fn fuse_lookup(&mut self, parent: u64, name: &std::ffi::OsStr) -> Result<ReplyEntry, i32>;
    fn fuse_create(
        &mut self,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
    ) -> Result<ReplyCreate, i32>;
    fn fuse_access(&mut self, ino: u64, mask: i32) -> Result<(), i32>;
    fn fuse_bmap(&mut self, _ino: u64, _blocksize: u32, _idx: u64, _reply: fuser::ReplyBmap);
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
        _reply: fuser::ReplyWrite,
    );
    fn fuse_destroy(&mut self);
    #[cfg(feature = "macos")]
    fn fuse_exchange(
        &mut self,
        parent: u64,
        name: &std::ffi::OsStr,
        newparent: u64,
        newname: &std::ffi::OsStr,
        options: u64,
    ) -> Result<(), i32>;
    fn fuse_fallocate(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _length: i64,
        _mode: i32,
    ) -> Result<(), i32>;
    fn fuse_flush(&mut self, ino: u64, fh: u64, _lock_owner: u64) -> Result<(), i32>;
    fn fuse_forget(&mut self, ino: u64, _nlookup: u64);
    fn fuse_fsync(&mut self, _ino: u64, _fh: u64, _datasync: bool) -> Result<(), i32>;
    fn fuse_fsyncdir(&mut self, _ino: u64, _fh: u64, _datasync: bool) -> Result<(), i32>;
    fn fuse_getlk(
        &mut self,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: i32,
        _pid: u32,
        _reply: fuser::ReplyLock,
    );
    fn fuse_getxattr(
        &mut self,
        ino: u64,
        name: &std::ffi::OsStr,
        _size: u32,
        reply: fuser::ReplyXattr,
    );
    #[cfg(feature = "macos")]
    fn fuse_getxtimes(&mut self, _ino: u64, _reply: fuser::ReplyXTimes);
    fn fuse_ioctl(
        &mut self,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        _cmd: u32,
        _in_data: &[u8],
        _out_size: u32,
        _reply: fuser::ReplyIoctl,
    );
    fn fuse_link(
        &mut self,
        _ino: u64,
        _newparent: u64,
        _newname: &std::ffi::OsStr,
        _reply: fuser::ReplyEntry,
    );
    fn fuse_listxattr(&mut self, _ino: u64, _sizee: u32, _reply: fuser::ReplyXattr);
    fn fuse_lseek(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _whence: i32,
        _reply: fuser::ReplyLseek,
    );
    fn fuse_mkdir(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        _reply: fuser::ReplyEntry,
    );
    fn fuse_mknod(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        _rdev: u32,
        _reply: fuser::ReplyEntry,
    );
    fn fuse_open(&mut self, ino: u64, flags: i32, reply: fuser::ReplyOpen);
    fn fuse_opendir(&mut self, _ino: u64, _flags: i32, _reply: fuser::ReplyOpen);
    fn fuse_read(
        &mut self,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        _reply: fuser::ReplyData,
    );
    fn fuse_readdir(&mut self, _ino: u64, _fh: u64, _offset: i64, _reply: fuser::ReplyDirectory);
    fn fuse_readdirplus(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _reply: fuser::ReplyDirectoryPlus,
    );
    fn fuse_readlink(&mut self, _ino: u64, _replyy: fuser::ReplyData);
    fn fuse_release(
        &mut self,
        ino: u64,
        fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
    ) -> Result<(), i32>;
    fn fuse_releasedir(&mut self, _ino: u64, _fh: u64, _flags: i32) -> Result<(), i32>;
    fn fuse_removexattr(&mut self, _ino: u64, _name: &std::ffi::OsStr) -> Result<(), i32>;
    fn fuse_rename(
        &mut self,
        _parent: u64,
        _name: &std::ffi::OsStr,
        _newparent: u64,
        _newname: &std::ffi::OsStr,
        _flags: u32,
    ) -> Result<(), i32>;
    fn fuse_rmdir(&mut self, _parent: u64, _name: &std::ffi::OsStr) -> Result<(), i32>;
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
        reply: fuser::ReplyAttr,
    );
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
    ) -> Result<(), i32>;
    #[cfg(feature = "macos")]
    fn fuse_setvolname(&mut self, _name: &std::ffi::OsStr) -> Result<(), i32>;
    fn fuse_setxattr(
        &mut self,
        ino: u64,
        name: &std::ffi::OsStr,
        _value: &[u8],
        _flags: i32,
        _position: u32,
    ) -> Result<(), i32>;
    fn fuse_statfs(&mut self, _ino: u64, reply: fuser::ReplyStatfs);
    fn fuse_symlink(
        &mut self,
        _parent: u64,
        _link_name: &std::ffi::OsStr,
        _target: &std::path::Path,
        _reply: fuser::ReplyEntry,
    );
    fn fuse_unlink(&mut self, parent: u64, name: &std::ffi::OsStr) -> Result<(), i32>;
    fn fuse_write(
        &mut self,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    );
}

impl Filesystem for S3TMFS {
    fn init(
        &mut self,
        _req: &fuser::Request<'_>,
        _config: &mut fuser::KernelConfig,
    ) -> Result<(), libc::c_int> {
        self.fuse_init()
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        match self.fuse_getattr(ino) {
            Ok(ra) => reply.attr(ra.ttl, ra.attr),
            Err(err) => reply.error(err),
        }
    }

    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        match self.fuse_lookup(parent, name) {
            Ok(rl) => reply.entry(rl.ttl, rl.attr, rl.generation),
            Err(err) => reply.error(err),
        }
    }

    fn create(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        match self.fuse_create(parent, name, mode, umask, flags) {
            Ok(rc) => reply.created(&rc.ttl, &rc.attr, rc.generation, rc.fh, rc.flags),
            Err(err) => reply.error(err),
        }
    }

    fn access(&mut self, _req: &fuser::Request<'_>, ino: u64, mask: i32, reply: ReplyEmpty) {
        match self.fuse_access(ino, mask) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn bmap(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        blocksize: u32,
        idx: u64,
        reply: fuser::ReplyBmap,
    ) {
        self.fuse_bmap(ino, blocksize, idx, reply)
    }

    fn copy_file_range(
        &mut self,
        _req: &fuser::Request<'_>,
        ino_in: u64,
        fh_in: u64,
        offset_in: i64,
        ino_out: u64,
        fh_out: u64,
        offset_out: i64,
        len: u64,
        flags: u32,
        reply: fuser::ReplyWrite,
    ) {
        self.fuse_copy_file_range(
            ino_in, fh_in, offset_in, ino_out, fh_out, offset_out, len, flags, reply,
        )
    }

    fn destroy(&mut self) {
        self.fuse_destroy()
    }

    #[cfg(feature = "macos")]
    fn exchange(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        newparent: u64,
        newname: &std::ffi::OsStr,
        options: u64,
        reply: ReplyEmpty,
    ) {
        match self.fuse_exchange(parent, name, newparent, newname, options) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn fallocate(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        length: i64,
        mode: i32,
        reply: ReplyEmpty,
    ) {
        match self.fuse_fallocate(ino, fh, offset, length, mode) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn flush(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        reply: ReplyEmpty,
    ) {
        match self.fuse_flush(ino, fh, lock_owner) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn forget(&mut self, _req: &fuser::Request<'_>, ino: u64, nlookup: u64) {
        self.fuse_forget(ino, nlookup)
    }

    fn fsync(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        match self.fuse_fsync(ino, fh, datasync) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn fsyncdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        match self.fuse_fsyncdir(ino, fh, datasync) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn getlk(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        reply: fuser::ReplyLock,
    ) {
        self.fuse_getlk(ino, fh, lock_owner, start, end, typ, pid, reply)
    }

    fn getxattr(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        name: &std::ffi::OsStr,
        size: u32,
        reply: fuser::ReplyXattr,
    ) {
        self.fuse_getxattr(ino, name, size, reply)
    }

    #[cfg(feature = "macos")]
    fn getxtimes(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyXTimes) {
        self.fuse_getxtimes(ino, reply)
    }

    fn ioctl(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: u32,
        cmd: u32,
        in_data: &[u8],
        out_size: u32,
        reply: fuser::ReplyIoctl,
    ) {
        self.fuse_ioctl(ino, fh, flags, cmd, in_data, out_size, reply)
    }

    fn link(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        newparent: u64,
        newname: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        self.fuse_link(ino, newparent, newname, reply)
    }

    fn listxattr(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        size: u32,
        reply: fuser::ReplyXattr,
    ) {
        self.fuse_listxattr(ino, size, reply)
    }

    fn lseek(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        whence: i32,
        reply: fuser::ReplyLseek,
    ) {
        self.fuse_lseek(ino, fh, offset, whence, reply)
    }

    fn mkdir(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        reply: fuser::ReplyEntry,
    ) {
        self.fuse_mkdir(parent, name, mode, umask, reply)
    }

    fn mknod(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: fuser::ReplyEntry,
    ) {
        self.fuse_mknod(parent, name, mode, umask, rdev, reply)
    }

    fn open(&mut self, _req: &fuser::Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        self.fuse_open(ino, flags, reply)
    }

    fn opendir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        flags: i32,
        reply: fuser::ReplyOpen,
    ) {
        self.fuse_opendir(ino, flags, reply)
    }

    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        self.fuse_read(ino, fh, offset, size, flags, lock_owner, reply)
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: fuser::ReplyDirectory,
    ) {
        self.fuse_readdir(ino, fh, offset, reply)
    }

    fn readdirplus(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: fuser::ReplyDirectoryPlus,
    ) {
        self.fuse_readdirplus(ino, fh, offset, reply)
    }

    fn readlink(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyData) {
        self.fuse_readlink(ino, reply)
    }

    fn release(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        lock_owner: Option<u64>,
        flush: bool,
        reply: ReplyEmpty,
    ) {
        match self.fuse_release(ino, fh, flags, lock_owner, flush) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn releasedir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        reply: ReplyEmpty,
    ) {
        match self.fuse_releasedir(ino, fh, flags) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn removexattr(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        name: &std::ffi::OsStr,
        reply: ReplyEmpty,
    ) {
        match self.fuse_removexattr(ino, name) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn rename(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        newparent: u64,
        newname: &std::ffi::OsStr,
        flags: u32,
        reply: ReplyEmpty,
    ) {
        match self.fuse_rename(parent, name, newparent, newname, flags) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn rmdir(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: ReplyEmpty,
    ) {
        match self.fuse_rmdir(parent, name) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn setattr(
        &mut self,
        _req: &fuser::Request<'_>,
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
        reply: fuser::ReplyAttr,
    ) {
        self.fuse_setattr(
            ino, mode, uid, gid, size, atime, mtime, ctime, fh, crtime, chgtime, bkuptime, flags,
            reply,
        )
    }

    fn setlk(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        sleep: bool,
        reply: ReplyEmpty,
    ) {
        match self.fuse_setlk(ino, fh, lock_owner, start, end, typ, pid, sleep) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    #[cfg(feature = "macos")]
    fn setvolname(&mut self, _req: &fuser::Request<'_>, name: &std::ffi::OsStr, reply: ReplyEmpty) {
        match self.fuse_setvolname(name) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn setxattr(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        name: &std::ffi::OsStr,
        value: &[u8],
        flags: i32,
        position: u32,
        reply: ReplyEmpty,
    ) {
        match self.fuse_setxattr(ino, name, value, flags, position) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn statfs(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyStatfs) {
        self.fuse_statfs(ino, reply)
    }

    fn symlink(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        link_name: &std::ffi::OsStr,
        target: &std::path::Path,
        reply: fuser::ReplyEntry,
    ) {
        self.fuse_symlink(parent, link_name, target, reply)
    }

    fn unlink(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: ReplyEmpty,
    ) {
        match self.fuse_unlink(parent, name) {
            Ok(_) => reply.ok(),
            Err(err) => reply.error(err),
        }
    }

    fn write(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    ) {
        self.fuse_write(ino, fh, offset, data, write_flags, flags, lock_owner, reply)
    }
}
