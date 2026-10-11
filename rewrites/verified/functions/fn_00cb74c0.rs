// original: 0x00CB74C0 CTaskSimpleMoveTrackingEntity::vf7

/// Copy the three-word position at offsets 0x2c, 0x30, and 0x34 into the output pointer and return it. Both branches select the same fields; the method pops one pointer argument.
lf_checker_rt::export!(thiscall, rw_00cb74c0(this: u32, output: u32) -> u32 {
unsafe {
        for offset in [0u32, 4, 8] {
            let word = ((this + 0x2c + offset) as *const u32).read_unaligned();
            ((output + offset) as *mut u32).write_unaligned(word);
        }
        output
     }
});
