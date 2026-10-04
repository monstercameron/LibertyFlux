// original: 0x00d708f0 replay_bar_store_cursor
/// Store the cursor position and generation.
///
/// Writes `pos` to +0x20 and `gen` to +0x100, and returns `gen`.
lf_checker_rt::export!(thiscall, rw_00d708f0(this_ptr: u32, pos_bits: u32, gen: u32) -> u32 {
    unsafe {
        let b = this_ptr as *mut u8;
        *((b.add(0x100)) as *mut u32) = gen;
        *((b.add(0x20)) as *mut u32) = pos_bits;
        gen
    }
});
