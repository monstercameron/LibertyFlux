// original: 0x00a07a90 pool_view_triplet_copy (proposed)
/// Copy the current pool view's 12-byte triplet to `dst`.
///
/// Fetches the singleton view, calls its slot at +0x2c for the triplet
/// pointer, and copies three dwords to `dst`. Returns `dst`. Stdcall.
lf_checker_rt::export!(stdcall, rw_00a07a90(dst: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 0;
        const SLOT: u32 = 0x2c;
        let view: u32 = lf_checker_rt::callee_cdecl!(FETCH, u32,);
        let vtab = (view as *const u32).read_unaligned();
        let slot = ((vtab + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let src = f(view);
        for i in 0..3u32 {
            let w = ((src + i * 4) as *const u32).read_unaligned();
            ((dst + i * 4) as *mut u32).write_unaligned(w);
        }
        dst
    }
});
