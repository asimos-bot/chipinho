use core::{alloc::{GlobalAlloc, Layout}, cell::UnsafeCell};

const ARENA_SIZE: usize = 128 * 1024;
#[repr(C, align(32))]
struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    head: UnsafeCell<usize>,
}

impl SimpleAllocator {
    const fn new() -> Self {
        SimpleAllocator {
            arena: UnsafeCell::new([0; ARENA_SIZE]),
            head: UnsafeCell::new(0),
        }
    }
}

unsafe impl Sync for SimpleAllocator {}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator::new();

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Find the next address that has the right alignment.
        let idx = (*self.head.get()).next_multiple_of(align);
        // Bump the head to the next free byte
        *self.head.get() = idx + size;
        let arena: &mut [u8; ARENA_SIZE] = &mut (*self.arena.get());
        // If we ran out of arena space, we return a null pointer, which
        // signals a failed allocation.
        match arena.get_mut(idx) {
            Some(item) => item as *mut u8,
            _ => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        /* lol */
    }
}
