use super::std::{sync::Arc, thread};
use crate::memory::range_allocator::RangeAllocator;
use crate::sparse::PageTable;
use alloc::vec::Vec;
use parking_lot::Mutex;

#[test]
fn drop_order_destroys_resource_before_memory() {
    struct Resource<'a>(&'a Mutex<Vec<&'static str>>);
    struct Memory<'a>(&'a Mutex<Vec<&'static str>>);

    impl Drop for Resource<'_> {
        fn drop(&mut self) {
            self.0.lock().push("resource");
        }
    }

    impl Drop for Memory<'_> {
        fn drop(&mut self) {
            self.0.lock().push("memory");
        }
    }

    struct Pair<'a> {
        _resource: Resource<'a>,
        _memory: Memory<'a>,
    }

    let log = Mutex::new(Vec::new());
    drop(Pair {
        _resource: Resource(&log),
        _memory: Memory(&log),
    });
    assert_eq!(&*log.lock(), &["resource", "memory"]);
}

#[test]
fn concurrent_allocate_free_stress_keeps_allocator_consistent() {
    let allocator = Arc::new(Mutex::new(RangeAllocator::new(1 << 20)));
    let mut threads = Vec::new();
    for _ in 0..8 {
        let allocator = allocator.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..500 {
                let offset = allocator.lock().allocate(256, 64).unwrap();
                allocator.lock().free(offset, 256);
            }
        }));
    }
    for handle in threads {
        handle.join().unwrap();
    }
    assert!(allocator.lock().is_non_overlapping());
}

#[test]
fn concurrent_sparse_updates_do_not_corrupt_page_table() {
    let table = Arc::new(Mutex::new(PageTable::<u32, u32>::default()));
    let mut threads = Vec::new();
    for i in 0..8u32 {
        let table = table.clone();
        threads.push(thread::spawn(move || {
            for n in 0..128u32 {
                table.lock().insert(i * 1000 + n, n);
            }
        }));
    }
    for handle in threads {
        handle.join().unwrap();
    }
    let mut count = 0usize;
    table.lock().for_each(|_, _| count += 1);
    assert_eq!(count, 8 * 128);
}
