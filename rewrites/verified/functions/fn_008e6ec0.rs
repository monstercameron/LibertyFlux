// original: 0x008E6EC0 path_node_loaded_check (proposed)

/// Test whether one quantised path node is loaded.
///
/// The two stack floats are quantised by the indexer (callee 1, thiscall on
/// `obj`, second float first): `r1` from the second float, `r2` from the
/// first. The cell index is `r2 + r1 * 8`; the function returns whether the
/// word at `GRID` plus the index (as a dword offset) is non-zero.
///
/// Original: 0x008E6EC0 (thiscall, two float stack arguments).
lf_checker_rt::export!(thiscall, rw_008E6EC0(obj: u32, fa: u32, fb: u32) -> u32 {
    unsafe {
        /// Offset of the loaded-flag grid (dword per cell).
        const GRID: u32 = 0x804;
        /// Row stride multiplier applied to the second float's index.
        const ROW_MUL: u32 = 8;
        /// Indexer callee id.
        const INDEX: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let r1: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, fb);
        let r2: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, fa);
        let idx = r2.wrapping_add(r1.wrapping_mul(ROW_MUL));
        u32::from(rd32(obj.wrapping_add(GRID).wrapping_add(idx.wrapping_mul(4))) != 0)
    }
});
