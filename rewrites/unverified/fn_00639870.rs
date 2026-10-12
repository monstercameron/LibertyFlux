// original: 0x00639870 copy_three_position_words

/// Copy three consecutive 32-bit words from `source` into the position fields
/// of `this`. The source is read and each destination word is written in
/// order, so overlapping source and destination ranges retain the original
/// sequential behavior. `this` is passed in ECX and the source pointer is the
/// single stack argument; the callee removes that argument.
lf_checker_rt::export!(thiscall, rw_00639870(this: u32, source: u32) -> u32 {
    unsafe {
        const POSITION_X: u32 = 0x190;
        const POSITION_Y: u32 = 0x194;
        const POSITION_Z: u32 = 0x198;

        let x = (source as *const u32).read_unaligned();
        (this.wrapping_add(POSITION_X) as *mut u32).write_unaligned(x);
        let y = (source.wrapping_add(4) as *const u32).read_unaligned();
        (this.wrapping_add(POSITION_Y) as *mut u32).write_unaligned(y);
        let z = (source.wrapping_add(8) as *const u32).read_unaligned();
        (this.wrapping_add(POSITION_Z) as *mut u32).write_unaligned(z);
        0
    }
});
