// original: 0x00CB74A0 CTaskSimpleMovePathfindProblem::vf7

/// Write three zero words to the output pointer and return that pointer. The thiscall method pops its one pointer argument.
lf_checker_rt::export!(thiscall, rw_00cb74a0(this: u32, output: u32) -> u32 {
unsafe {
        for offset in [0u32, 4, 8] {
            ((output + offset) as *mut u32).write_unaligned(0);
        }
        output
     }
});
