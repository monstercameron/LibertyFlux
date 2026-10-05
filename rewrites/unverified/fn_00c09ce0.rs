// original: 0x00c09ce0 stream_pair_register (proposed)

/// Register the caller's item with its tag in a fresh pair.
///
/// `this` is advanced by `THIS_BIAS` for the allocator (callee 1, also handed
/// the constant `SIZE`); the two words it returns receive `item` and the byte
/// at `TAG` past `item`. Returns the pair pointer.
///
/// Original: 0x00c09ce0 (thiscall, one stack word; allocator is thiscall).
lf_checker_rt::export!(thiscall, rw_00c09ce0(this: u32, item: u32) -> u32 {
    unsafe {
        const THIS_BIAS: u32 = 0x522c;
        const SIZE: u32 = 0x10;
        const TAG: u32 = 0x4d;
        const ALLOC: u32 = 1;
        let tag = (item.wrapping_add(TAG) as *const u8).read() as u32;
        let pair: u32 =
            lf_checker_rt::callee_thiscall!(ALLOC, u32, this.wrapping_add(THIS_BIAS), SIZE);
        (pair as *mut u32).write_unaligned(item);
        (pair.wrapping_add(4) as *mut u32).write_unaligned(tag);
        pair
    }
});
