//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;
use crate::bitmap::*;
use crate::AllocError;
const PAGE_SIZE:usize = 4096;
use crate::{AllocResult, BaseAllocator, ByteAllocator};
use crate::PageAllocator;

pub struct SimpleByteAllocator
{
    start:usize,
    end:usize,
    allocation:usize,
    next:usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start:0,
            end:0,
            allocation:0,
            next:0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start=_start;
        self.end=_start+_size;
        self.allocation=0;
        self.next=_start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let start=(self.next+(_layout.align()-1))&!(_layout.align()-1);
        if start + _layout.size() > self.end {
            Err(AllocError::NoMemory)
        }
        else
        {
            self.allocation+=1;
            self.next=start+_layout.size();
            Ok(NonZeroUsize::new(start).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocation=self.allocation-1;
        if self.allocation==0{
            self.next=self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end-self.start
    }

    fn used_bytes(&self) -> usize {
        self.next-self.start
    }

    fn available_bytes(&self) -> usize {
        self.end-self.next
    }
}
