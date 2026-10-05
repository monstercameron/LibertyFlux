// original: 0x0093B5C0 stream_obj_setup (proposed)

/// Visit every node of the streaming list, then tail-finish.
///
/// Walks the global list head through each node's link word, calling the
/// visitor on every node, and tail-calls the finisher, returning its
/// answer. (The batch size overruns into the next function; the real
/// body ends at the tail jump.)
lf_checker_rt::export!(cdecl, rw_0093b5c0() -> u32 {
    unsafe {
        const VISIT: u32 = 1;
        const FINISH: u32 = 2;
        const HEAD: u32 = 0x11A4EE0;
        let mut node = lf_checker_rt::global::<u32>(HEAD).read_unaligned();
        while node != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(VISIT, u32, node);
            node = (node as *const u32).read_unaligned();
        }
        lf_checker_rt::callee_cdecl!(FINISH, u32,)
    }
});
