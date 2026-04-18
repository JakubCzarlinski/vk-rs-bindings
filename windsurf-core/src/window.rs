use crate::LogicalSize;
use alloc::string::String;
use alloc::vec::Vec;

extern crate alloc;

/// Stable opaque handle for a window managed by a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle(u8);

/// Allocates/recycles [`WindowHandle`] values from a fixed `u8` space.
///
/// Handles are reused after release. Allocation fails only when 255 handles are
/// concurrently live.
#[derive(Debug, Clone, Default)]
pub struct WindowHandleAllocator {
    next_id: u8,
    free_list: Vec<WindowHandle>,
    live_count: u16,
}

impl WindowHandleAllocator {
    pub const MAX_CONCURRENT_WINDOWS: u16 = 255;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self) -> Option<WindowHandle> {
        if let Some(handle) = self.free_list.pop() {
            self.live_count = self.live_count.saturating_add(1);
            return Some(handle);
        }

        if self.next_id == u8::MAX {
            return None;
        }

        self.next_id = self.next_id.saturating_add(1);
        self.live_count = self.live_count.saturating_add(1);
        Some(WindowHandle(self.next_id))
    }

    pub fn release(&mut self, handle: WindowHandle) {
        if self.live_count == 0 || self.free_list.contains(&handle) {
            return;
        }
        self.live_count -= 1;
        self.free_list.push(handle);
    }

    pub const fn live_count(&self) -> u16 {
        self.live_count
    }

    pub const fn at_capacity(&self) -> bool {
        self.live_count >= Self::MAX_CONCURRENT_WINDOWS
    }
}

/// Parameters shared by backend window constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowAttributes {
    /// Initial window title.
    pub title: String,
    /// Initial logical inner size.
    pub size: LogicalSize,
    /// Optional logical minimum size.
    pub min_size: Option<LogicalSize>,
    /// Optional logical maximum size.
    pub max_size: Option<LogicalSize>,
    /// Whether the platform should show system decorations when available.
    pub decorations: bool,
    /// Whether the compositor should allow transparency when supported.
    pub transparent: bool,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            title: String::from("windsurf"),
            size: LogicalSize::default(),
            min_size: None,
            max_size: None,
            decorations: true,
            transparent: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WindowHandleAllocator;
    use alloc::vec::Vec;

    extern crate alloc;

    #[test]
    fn handle_allocator_reuses_released_ids() {
        let mut allocator = WindowHandleAllocator::new();
        let first = allocator.allocate().unwrap();
        let second = allocator.allocate().unwrap();
        allocator.release(first);
        let third = allocator.allocate().unwrap();
        assert_eq!(third, first);
        assert_ne!(third, second);
    }

    #[test]
    fn handle_allocator_exhausts_at_255_live_windows() {
        let mut allocator = WindowHandleAllocator::new();
        let mut handles = Vec::new();
        for _ in 0..WindowHandleAllocator::MAX_CONCURRENT_WINDOWS {
            handles.push(allocator.allocate().unwrap());
        }
        assert!(allocator.allocate().is_none());
        assert_eq!(
            allocator.live_count(),
            WindowHandleAllocator::MAX_CONCURRENT_WINDOWS
        );

        allocator.release(handles[17]);
        let recycled = allocator.allocate().unwrap();
        assert_eq!(recycled, handles[17]);
    }

    #[test]
    fn allocator_never_issues_duplicate_live_handles() {
        let mut allocator = WindowHandleAllocator::new();
        let a = allocator.allocate().unwrap();
        let b = allocator.allocate().unwrap();
        let c = allocator.allocate().unwrap();
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
}
