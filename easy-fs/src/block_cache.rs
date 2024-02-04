use lazy_static::lazy_static;
use spin::Mutex;

use alloc::{sync::Arc, collections::VecDeque};

use crate::{BLOCK_SIZE, BlockDevice, BLOCK_CACHE_SIZE, block_dev};


/// Structure for cache block which is inside memory
pub struct BlockCache {
    /// cached block data
    cache: [u8; BLOCK_SIZE],
    /// block id on disk
    block_id: usize,
    /// block device
    block_device: Arc<dyn BlockDevice>,
    /// dirty flag
    modified: bool,
}

impl BlockCache {
    /// Load a new BlockCache from disk.
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SIZE];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }
    /// Get pointer from the cache by an offset
    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }
    /// Get a immutable reference of type T from cache by an offset
    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SIZE);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }
    /// Get mutable reference of type T from cache by an offset
    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized
    {
        let type_size = core::mem::size_of::<T>();
        // assert that the offset is within the block
        assert!(offset + type_size <= BLOCK_SIZE);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }
    /// read a block cache by closure f
    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }
    /// modify a block cache by closure f
    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }
    /// Synchronize the data in the cache andi on the disk
    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

pub struct BlockCacheManager {
    /// usize for block id
    queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    /// Try to get block cache, if not in the cache, load it from disk
    /// if the cache is full, remove one and load the new one using FIFO policy
    pub fn get_block_cache(
        &mut self,
        block_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>> {
        if let Some(pair) = 
            self.queue.iter().find(|pair| pair.0 == block_id) {
                Arc::clone(&pair.1)
        } else {
            // cannot find
            if self.queue.len() == BLOCK_CACHE_SIZE {
                // remove a block with no strong reference
                if let Some((idx, _)) = self
                    .queue
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    self.queue.drain(idx..=idx);
                } else {
                    panic!("Run out of BlockCache!");
                }
            }
            // load block into mem and push back to queue
            let block_cache = Arc::new(
                Mutex::new(BlockCache::new(
                    block_id, 
                    Arc::clone(&block_device)
            )));
            self.queue.push_back((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }
}

lazy_static! {
    /// A global block cache manager
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> = 
        Mutex::new(BlockCacheManager::new());
}

/// Get the block cache corresponding to the given block id and block device
pub fn get_block_cache(
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHE_MANAGER
        .lock()
        .get_block_cache(block_id, block_device)
}
/// Sync all block cache to block device
pub fn block_cache_syn_all() {
    let manager = BLOCK_CACHE_MANAGER.lock();
    for (_, cache) in manager.queue.iter() {
        cache.lock().sync()
    }
}