// original: 0x00D9F100 append_stream_record
/// Appends one eight-byte record to a pointer-backed vector. When its 16-bit size reaches its 16-bit capacity, it grows the capacity by the requested amount, asks the allocator helper for a new buffer, copies existing records, releases the old buffer, and updates the vector pointer.
lf_checker_rt::export!(thiscall, rw_s675_00d9f100(this: u32, growth: u32) -> u32 {
    const ARRAY_OFFSET: u32 = 0;
    const SIZE_OFFSET: u32 = 4;
    const CAPACITY_OFFSET: u32 = 6;
    const ENTRY_STRIDE: u32 = 8;
    let size = unsafe { (this.wrapping_add(SIZE_OFFSET) as *const u16).read_unaligned() };
    let mut entries = unsafe { (this.wrapping_add(ARRAY_OFFSET) as *const u32).read_unaligned() };
    let capacity = unsafe { (this.wrapping_add(CAPACITY_OFFSET) as *const u16).read_unaligned() };
    if size == capacity {
        let grown_capacity = (u32::from(capacity).wrapping_add(growth) as u16) as u32;
        unsafe { (this.wrapping_add(CAPACITY_OFFSET) as *mut u16).write_unaligned(grown_capacity as u16); }
        let new_entries: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, grown_capacity);
        for index in 0..u32::from(size) {
            let source = entries.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let destination = new_entries.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            for word_offset in [0, 4] {
                let word = unsafe { (source.wrapping_add(word_offset) as *const u32).read_unaligned() };
                unsafe { (destination.wrapping_add(word_offset) as *mut u32).write_unaligned(word); }
            }
        }
        let _: () = lf_checker_rt::callee_cdecl!(2, (), entries);
        unsafe { (this.wrapping_add(ARRAY_OFFSET) as *mut u32).write_unaligned(new_entries); }
        entries = new_entries;
    }
    unsafe { (this.wrapping_add(SIZE_OFFSET) as *mut u16).write_unaligned(size.wrapping_add(1)); }
    entries.wrapping_add(u32::from(size).wrapping_mul(ENTRY_STRIDE)).wrapping_add(0)
});
