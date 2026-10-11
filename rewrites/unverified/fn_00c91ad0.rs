// original: 0x00C91AD0 StoreIndexedTaskWord

/// Store one 32-bit word in the object's indexed task array. ECX points to the object; the first stack word is the value and the second is an unsigned element index. The array begins at byte offset 0x44, each element is four bytes, the function returns the stored value in EAX, and the thiscall epilogue removes both stack arguments.
lf_checker_rt::export!(thiscall, rw_00c91ad0(this: u32, value: u32, index: u32) -> u32 {
    const TASK_ARRAY: u32 = 0x44;
    unsafe {
        let slot = this.wrapping_add(TASK_ARRAY).wrapping_add(index.wrapping_mul(4));
        (slot as *mut u32).write_unaligned(value);
    }
    value
});
