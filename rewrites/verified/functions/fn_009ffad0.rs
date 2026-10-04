// original: 0x009FFAD0 frag_side_b_followup (proposed)

/// Run the side-B follow-up step for `arg` when the tester agrees.
///
/// Offers (`arg`, side-B global) to the tester callee; when its low byte is
/// zero returns its answer. Otherwise runs the worker callee with (`arg`,
/// side-B global, residue), where `residue` is the caller's next stack word
/// past the declared argument, and returns the worker's answer.
///
/// Original: 0x009FFAD0 (cdecl, one declared stack word plus caller residue).
lf_checker_rt::export!(cdecl, rw_009FFAD0(arg: u32, residue: u32) -> u32 {
    unsafe {
        const SIDE_B: u32 = 0x0104B888;
        let g = (lf_checker_rt::relocated(SIDE_B) as *const u32).read_unaligned();
        let r = lf_checker_rt::callee_cdecl!(1, u32, arg, g);
        if (r as u8) == 0 {
            return r;
        }
        lf_checker_rt::callee_cdecl!(2, u32, arg, g, residue)
    }
});
