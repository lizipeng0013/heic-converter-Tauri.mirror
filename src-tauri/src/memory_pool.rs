//! 内存池优化
//!
//! 用于复用图像缓冲区，减少内存分配开销

use once_cell::sync::Lazy;
use std::sync::Mutex;

/// 全局缓冲区池
static BUFFER_POOL: Lazy<Mutex<BufferPool>> = Lazy::new(|| {
    Mutex::new(BufferPool::new(1024 * 1024)) // 初始 1MB
});

/// 缓冲区池
/// 
/// 用于复用图像缓冲区，减少内存分配和释放的开销。
/// 对于大图像处理尤其有效，可以避免频繁的内存分配。
pub struct BufferPool {
    /// 可复用的缓冲区列表
    buffers: Vec<Vec<u8>>,
    /// 最小缓冲区大小
    min_buffer_size: usize,
}

impl BufferPool {
    /// 创建新的缓冲区池
    pub fn new(min_buffer_size: usize) -> Self {
        Self {
            buffers: Vec::new(),
            min_buffer_size,
        }
    }
    
    /// 获取一个缓冲区
    ///
    /// 如果池中有合适的缓冲区，则复用；否则分配新的缓冲区。
    pub fn acquire(&mut self, size: usize) -> Vec<u8> {
        // 查找合适的缓冲区（检查 capacity 而不是 len）
        if let Some(index) = self.buffers.iter().position(|buf| buf.capacity() >= size) {
            // 复用现有缓冲区
            let mut buffer = self.buffers.remove(index);
            buffer.clear();
            buffer
        } else {
            // 分配新缓冲区
            vec![0u8; size.max(self.min_buffer_size)]
        }
    }
    
    /// 释放缓冲区回池
    ///
    /// 将缓冲区回收，以便后续复用。
    /// 注意：缓冲区内容会被清空。
    pub fn release(&mut self, mut buffer: Vec<u8>) {
        // 清空缓冲区内容
        buffer.clear();
        
        // 只保留合理大小的缓冲区，避免占用过多内存
        if buffer.capacity() <= 100 * 1024 * 1024 { // 最大 100MB
            self.buffers.push(buffer);
        }
        // 如果缓冲区太大，让它自然释放
    }
    
    /// 获取池中的缓冲区数量
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }
    
    /// 清空池中的所有缓冲区
    pub fn clear(&mut self) {
        self.buffers.clear();
    }
}

/// 从全局缓冲区池获取缓冲区
///
/// # 示例
/// ```rust
/// use heic_converter_lib::memory_pool::acquire_buffer;
/// let buffer = acquire_buffer(1024 * 1024); // 获取 1MB 缓冲区
/// ```
pub fn acquire_buffer(size: usize) -> Vec<u8> {
    BUFFER_POOL.lock().unwrap().acquire(size)
}

/// 释放缓冲区回全局池
///
/// # 示例
/// ```rust
/// use heic_converter_lib::memory_pool::release_buffer;
/// let buffer = vec![0u8; 1024];
/// release_buffer(buffer);
/// ```
pub fn release_buffer(buffer: Vec<u8>) {
    BUFFER_POOL.lock().unwrap().release(buffer);
}

/// 获取池中的缓冲区数量
pub fn buffer_pool_count() -> usize {
    BUFFER_POOL.lock().unwrap().buffer_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_pool() {
        let mut pool = BufferPool::new(1024);
        
        // 获取小缓冲区
        let buffer = pool.acquire(1024);  // 使用小缓冲区
        assert_eq!(buffer.len(), 1024);
        
        // 释放缓冲区
        pool.release(buffer);
        assert_eq!(pool.buffer_count(), 1);
        
        // 再次获取应该复用
        let _buffer2 = pool.acquire(1024);
        assert_eq!(pool.buffer_count(), 0);
    }
    
    #[test]
    fn test_buffer_pool_too_small() {
        let mut pool = BufferPool::new(1024);
        
        // 获取小缓冲区
        let buffer = pool.acquire(512); // 小于 min_buffer_size
        assert_eq!(buffer.len(), 1024); // 应该分配 min_buffer_size
        
        pool.release(buffer);
        assert_eq!(pool.buffer_count(), 1);
    }
}
