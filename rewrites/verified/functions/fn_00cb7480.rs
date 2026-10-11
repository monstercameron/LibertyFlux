// original: 0x00CB7480 CTaskSimpleMoveGoToPointOnRoute::vf7

/// Copy the three-word route position at offsets 0x2c, 0x30, and 0x34 into the output pointer and return that pointer. The thiscall method pops its one pointer argument.
lf_checker_rt::export!(thiscall, rw_00cb7480(this: u32, output: u32) -> u32 {
unsafe {
        for offset in [0u32, 4, 8] {
            let word = ((this + 0x2c + offset) as *const u32).read_unaligned();
            ((output + offset) as *mut u32).write_unaligned(word);
        }
        output
     }
});
