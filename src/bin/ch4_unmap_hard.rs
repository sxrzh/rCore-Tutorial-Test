#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{mmap, munmap, trace_read, trace_write};

/*
理想结果：输出 Test 04_ex ummap_hard OK!
*/

#[no_mangle]
fn main() -> i32 {
    const PAGE: usize = 4096;

    {
        // unmap2
        let start: usize = 0x10000000;
        let len: usize = 4096;
        let prot: usize = 3;
        assert_eq!(0, mmap(start, len, prot));
        assert_eq!(munmap(start, len + 1), -1);
        assert_eq!(munmap(start + 1, len - 1), -1);
    }

    {
        // map previously unmapped page
        let start: usize = 0x10001000;
        let len: usize = 4096;
        let prot: usize = 3;
        assert_eq!(0, mmap(start, len, prot));
        assert_eq!(munmap(start, len), 0);
        assert_eq!(0, mmap(start, len, prot));
        assert_eq!(munmap(start, len), 0);
    }

    {
        // unmap splits 1 area into 2
        let start: usize = 0x10002000;
        let len: usize = PAGE * 5;
        let prot: usize = 3;
        assert_eq!(0, mmap(start, len, prot));
        assert_eq!(munmap(start + PAGE * 2, PAGE * 2), 0);
        assert_eq!(0, trace_write((start + 10) as *const u8, 99));
        assert_eq!(Some(99), trace_read((start + 10) as *const u8));
        assert_eq!(0, trace_write((start + 4 * PAGE + 30) as *const u8, 66));
        assert_eq!(Some(66), trace_read((start + 4 * PAGE + 30) as *const u8));
        assert_eq!(-1, trace_write((start + 3 * PAGE + 50) as *const u8, 55));
        assert_eq!(None, trace_read((start + 3 * PAGE + 50) as *const u8));
    }

    {
        // unmap crosses several areas
        let start: usize = 0x10010000;
        let len1: usize = PAGE * 3;
        let len2: usize = PAGE * 4;
        let len3: usize = PAGE * 5;
        let prot: usize = 3;
        assert_eq!(0, mmap(start, len1, prot));
        assert_eq!(0, mmap(start + len1, len2, prot));
        assert_eq!(0, mmap(start + len1 + len2, len3, prot));
        assert_eq!(munmap(start + PAGE * 2, PAGE * 7), 0);
        
        assert_eq!(0, trace_write((start + 10) as *const u8, 99));
        assert_eq!(Some(99), trace_read((start + 10) as *const u8));
        assert_eq!(0, trace_write((start + 9 * PAGE + 30) as *const u8, 66));
        assert_eq!(Some(66), trace_read((start + 9 * PAGE + 30) as *const u8));
        assert_eq!(-1, trace_write((start + 5 * PAGE + 50) as *const u8, 55));
        assert_eq!(None, trace_read((start + 5 * PAGE + 50) as *const u8));
    }

    {
        // unmap contains unmapped page
        let start: usize = 0x10040000;
        let len1: usize = PAGE * 3;
        let len2: usize = PAGE * 4;
        let len3: usize = PAGE * 5;
        let prot: usize = 3;
        assert_eq!(0, mmap(start, len1, prot));
        assert_eq!(0, mmap(start + len1, len2, prot));
        assert_eq!(0, mmap(start + len1 + len2 + PAGE, len3, prot));
     
        assert_eq!(munmap(start - PAGE, PAGE * 2), -1);
        assert_eq!(munmap(start + PAGE * 2, PAGE * 7), -1);
        assert_eq!(munmap(start + len1 + len2 + PAGE + PAGE * 3, PAGE * 3), -1);
    }

    println!("Test 04_ex ummap_hard OK!");
    0
}
