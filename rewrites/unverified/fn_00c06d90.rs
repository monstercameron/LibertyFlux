// original: 0x00c06d90 stream_alloc_slot_array
/// Allocate `count` 80-byte streaming slots and clear each one.
///
/// Allocates `count * 80` bytes through the allocator helper (cdecl/1) and,
/// for a positive `count`, clears every element through the slot-clear helper
/// (thiscall/0), skipping the call when the allocator returned null. Returns
/// the allocated pointer (null when the allocator failed). The comparison on
/// `count` is signed. Stdcall: one stack word, callee cleans 4.
lf_checker_rt::export!(stdcall, rw_00c06d90(count: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CLEAR: u32 = 2;
        const SLOT_STRIDE: u32 = 80;
        let bytes = (count as i32).wrapping_mul(SLOT_STRIDE as i32) as u32;
        let base: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, bytes);
        if (count as i32) > 0 {
            let mut slot = base;
            let mut left = count as i32;
            while left != 0 {
                if slot != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(CLEAR, u32, slot);
                }
                slot = slot.wrapping_add(SLOT_STRIDE);
                left -= 1;
            }
        }
        base
    }
});
