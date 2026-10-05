// original: 0x00d549f0 ccam_lookup_param

/// Look up a camera parameter through two levels of indirection.
///
/// `handle` is null or points to a descriptor whose word at `LINK` points to
/// a second block; the result is the word at `INNER + SLOT` into that block.
/// A null handle returns 0. A null link is dereferenced anyway (the original
/// reads address `SLOT` with a zero base), which faults; the rewrite faults
/// the same way so fault trials compare as parity.
///
/// Original: 0x00d549f0 (stdcall, one stack argument, no calls).
lf_checker_rt::export!(stdcall, rw_00d549f0(handle: u32) -> u32 {
    unsafe {
        /// Link from the descriptor to the parameter block.
        const LINK: u32 = 0x228;
        /// Base adjustment applied to the linked block.
        const INNER: u32 = 0x70;
        /// Slot read from the adjusted block (also the fault address when
        /// the link is null).
        const SLOT: u32 = 0x3a4;
        if handle == 0 {
            return 0;
        }
        let block = (handle.wrapping_add(LINK) as *const u32).read_unaligned();
        if block == 0 {
            return (SLOT as *const u32).read_unaligned();
        }
        (block.wrapping_add(INNER).wrapping_add(SLOT) as *const u32).read_unaligned()
    }
});
