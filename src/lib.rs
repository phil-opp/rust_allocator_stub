#![feature(allocator)]
#![allocator]

#![no_std]

extern "C" {
    pub fn __rust_allocate(size: usize, align: usize) -> *mut u8;
    pub fn __rust_usable_size(size: usize, align: usize) -> usize;
    pub fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize);
    pub fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> *mut u8;
    pub fn __rust_reallocate_inplace(ptr: *mut u8,
                                     size: usize,
                                     new_size: usize,
                                     align: usize)
                                     -> usize;
}
