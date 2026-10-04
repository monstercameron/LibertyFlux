// original: 0x009FFEB0 frag_dual_probe (proposed)

/// Run the two probe callees over two uninitialised frame blocks.
///
/// Passes a pointer to one frame block to the first probe callee and a
/// pointer to another to the second (the pair updater). Both blocks are
/// uninitialised in the original, so zeroed stand-ins are passed and their
/// contents left uncompared. Returns the second probe's answer.
///
/// Original: 0x009FFEB0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009FFEB0() -> u32 {
    unsafe {
        let blk_a = [0u32; 4];
        let blk_b = [0u32; 4];
        lf_checker_rt::callee_cdecl!(1, u32, blk_a.as_ptr() as u32);
        lf_checker_rt::callee_cdecl!(2, u32, blk_b.as_ptr() as u32)
    }
});
