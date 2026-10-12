// original: 0x00B51580 clear_live_blend_blocks

/// Visit three blend-block records separated by `0x520` bytes. For each
/// record whose leading dword is nonzero, call its cleanup routine with that
/// record as ECX. The cleanup routine is an intercepted thiscall with no
/// stack arguments; this method itself is a void thiscall.
lf_checker_rt::export!(thiscall, rw_clear_live(blocks: *const u8) -> () {
    const SLOT_COUNT: usize = 3;
    const SLOT_STRIDE: usize = 0x520;
    unsafe {
        for slot_index in 0..SLOT_COUNT {
            let slot = blocks.add(slot_index * SLOT_STRIDE);
            if (slot as *const u32).read_unaligned() != 0 {
                let _ = lf_checker_rt::callee_thiscall!(1, u32, slot as u32);
            }
        }
    }
});
