// original: 0x00963930 cache_shutdown_free
/// Shut down the cache subsystem and free its two live blocks.
///
/// Takes no arguments. Runs the three stage helpers and the 2-argument
/// flusher (both words zero) in order, then, when the live pointer at
/// `0x11F7068` is non-null, releases it through the 1-argument deleter and
/// clears the slot. Always releases the pointer at `0x11FA014` (even when
/// null) and clears both words at `0x11FA014`/`0x11FA018`. Returns 0.
lf_checker_rt::export!(cdecl, rw_00963930() -> u32 {
    unsafe {
        const LIVE: u32 = 0x11f7068;
        const AUX: u32 = 0x11fa014;
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, 0, 0);
        let p = (lf_checker_rt::global::<u32>(LIVE) as *const u32).read_unaligned();
        if p != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(5, u32, p);
            (lf_checker_rt::global::<u32>(LIVE) as *mut u32).write_unaligned(0);
        }
        let q = (lf_checker_rt::global::<u32>(AUX) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32, q);
        (lf_checker_rt::global::<u32>(AUX) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(AUX + 4) as *mut u32).write_unaligned(0);
        0
    }
});
