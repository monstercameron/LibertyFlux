// original: 0x00a0bce0 render_state_lazy_get (proposed)
/// Return the shared render-state block, creating it on first use.
///
/// Reads the cached pointer global; when it is null the creator runs and
/// the global is re-read. (Under the checker the creator is a stub that
/// leaves the global null, so those trials answer null on both sides.)
/// Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a0bce0() -> u32 {
    unsafe {
        const CACHE: u32 = 0x012bd0d4;
        const CREATE: u32 = 0;
        let g = lf_checker_rt::global::<u32>(CACHE) as *const u32;
        if g.read_unaligned() != 0 {
            return g.read_unaligned();
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(CREATE, u32,);
        g.read_unaligned()
    }
});
