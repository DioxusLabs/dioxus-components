//! Core types for the virtual list implementation.

/// A unique key for identifying items in the virtualizer.
pub type Key = usize;

/// A single virtualized item with computed position.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualItem {
    key: Key,
    index: usize,
    start: u32,
    size: u32,
}

impl VirtualItem {
    /// Create a new virtual item measurement.
    pub fn new(key: Key, index: usize, start: u32, size: u32) -> Self {
        Self {
            key,
            index,
            start,
            size,
        }
    }

    /// Return the stable key used for size caching.
    pub fn key(&self) -> Key {
        self.key
    }

    /// Return the absolute item index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Return the item start offset in pixels.
    pub fn start(&self) -> u32 {
        self.start
    }

    /// Return the item end offset in pixels.
    pub fn end(&self) -> u32 {
        self.start + self.size
    }

    /// Return the item size in pixels.
    pub fn size(&self) -> u32 {
        self.size
    }
}
