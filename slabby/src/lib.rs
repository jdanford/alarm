//! A typed slab allocator suitable for use with `#![no_std]`.
#![no_std]
#![feature(unique)]

use core::ptr::Unique;

#[derive(Clone)]
pub enum Entry<T> {
    /// A free entry.
    Free,
    /// A filled entry.
    Present(T),
}

pub struct Page<T: Sized> {
    /// Pointer to the head of the page.
    head: Unique<T>,

    /// Length of the page.
    len: usize,
    /*    next: Shared<Page<T>>,
     *    prev: Shared<Page<T>>, */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
