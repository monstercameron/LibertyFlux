// original: 0x00a92130 stream_boot_reset_pair (proposed)

/// Reset two streaming singletons and clear two state words.
///
/// The first lookup (callee 1, cdecl/1 with a fixed key) is resolved; a
/// miss (-1) runs the fallback (callee 2, cdecl/2 with 0 and a second fixed
/// key). The slot reset (callee 3, cdecl/1) then takes the fallback's
/// answer on a miss, the index on a hit, and two global state words are
/// cleared.
///
/// Returns the slot reset's answer. Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a92130() -> u32 {
    unsafe {
        const KEY1: u32 = 0xea2fec;
        const KEY2: u32 = 0xea2ff4;
        const MISS: u32 = 0xffffffff;
        const STATE1: u32 = 0x012fb274;
        const STATE2: u32 = 0x012fb368;
        // The keys are relocated immediates: the original pushes the loaded
        // addresses, so the rewrite derives them the same way.
        let idx = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(KEY1));
        // On a miss the fallback's answer (still in eax), not the index,
        // becomes the slot reset's argument.
        let slot_arg = if idx == MISS {
            lf_checker_rt::callee_cdecl!(2, u32, 0, lf_checker_rt::relocated(KEY2))
        } else {
            idx
        };
        let r = lf_checker_rt::callee_cdecl!(3, u32, slot_arg);
        lf_checker_rt::global::<u32>(STATE1).write_unaligned(0);
        lf_checker_rt::global::<u32>(STATE2).write_unaligned(0);
        r
    }
});
