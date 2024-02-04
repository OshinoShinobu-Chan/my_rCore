#![no_std]

extern crate alloc;

mod block_dev;
mod block_cache;
mod layout;
mod bitmap;
mod efs;
mod vfs;

pub const BLOCK_SIZE: usize = 512;
pub const BLOCK_CACHE_SIZE: usize = 16;
/// The size of one directory entry
pub const DIRENT_SIZE: usize = 32;

pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
pub use vfs::Inode;
