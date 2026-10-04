// original: 0x008d5a10 build_blend_block_and_dispatch
// Copies eight source words into a twelve-word blend block (first six in
// order, then words 0, 1, 4, 5, 6, 7) and dispatches it with a tag, a key
// and a parameter. Returns the dispatch answer. The original's cookie check
// preserves eax, so the answer survives it; both callees share one script
// here because the checker's stub would otherwise overwrite eax.
export!(cdecl, rw_008d5a10(src: *const u32, key: u32, param: u32) -> u32 {
    unsafe {
        let a0 = *src.wrapping_add(0);
        let a1 = *src.wrapping_add(1);
        let a2 = *src.wrapping_add(2);
        let a3 = *src.wrapping_add(3);
        let a4 = *src.wrapping_add(4);
        let a5 = *src.wrapping_add(5);
        let a6 = *src.wrapping_add(6);
        let a7 = *src.wrapping_add(7);
        let mut buf = [a0, a1, a2, a3, a4, a5, a0, a1, a4, a5, a6, a7];
        let r: u32 = callee_cdecl!(
            1,
            u32,
            buf.as_mut_ptr() as u32,
            2,
            key,
            param
        );
        let _c: u32 = callee_cdecl!(2, u32,);
        r
    }
});
