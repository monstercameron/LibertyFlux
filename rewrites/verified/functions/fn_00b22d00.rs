// original: 0x00B22D00 streaming_name_to_id_bind
/// Store the caller's second word at `this+0x44`, request storage using the
/// first word, and publish the allocator's pointer at `this+0x40`. Thiscall
/// with two stack arguments; return the pointer from the helper call.
lf_checker_rt::export!(thiscall, rw_00b22d00(this: u32, lookup_key: u32, record_count: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        const STORED_COUNT: u32 = 0x44;
        const ALLOCATED_POINTER: u32 = 0x40;
        write_u32(this.wrapping_add(STORED_COUNT), record_count);
        let allocated = lf_checker_rt::callee_cdecl!(1, u32, lookup_key);
        write_u32(this.wrapping_add(ALLOCATED_POINTER), allocated);
        allocated
    }
});
