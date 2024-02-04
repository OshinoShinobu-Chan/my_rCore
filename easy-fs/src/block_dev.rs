use core::any::Any;
// Trait for block device
pub trait BlockDevice: Send + Sync + Any {
    /// Read data from block device to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    /// Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}