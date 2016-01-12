// The compiler needs to be instructed that this crate is an allocator in order
// to realize that when this is linked in another allocator like jemalloc should
// not be linked in
#![feature(allocator)]
#![allocator]

// Allocators are not allowed to depend on the standard library which in turn
// requires an allocator in order to avoid circular dependencies. This crate,
// however, can use all of libcore.
#![no_std]

extern {
    pub fn __rust_allocate(size: usize, _align: usize) -> *mut u8;

    pub fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize);

    pub fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                    _align: usize) -> *mut u8;

    pub fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                            _size: usize, _align: usize) -> usize;

    pub fn __rust_usable_size(size: usize, _align: usize) -> usize;
}
