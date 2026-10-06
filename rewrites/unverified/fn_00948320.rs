// original: 0x00948320 tls_guarded_init_dispatch (proposed)

/// Dispatch through thread-local state, refreshing two cached objects when a
/// generation counter changed.
///
/// The TLS slot index is read from `TLS_INDEX`; the current thread's object
/// is `tls[slot]`. When the object's flag word at `+THREAD_FLAG` is zero the
/// object itself is returned and nothing else happens. Otherwise the global
/// generation counter `GEN_COUNTER` is compared (unsigned equality) with the
/// last-seen value `GEN_LAST_SEEN`: when equal, the counter is returned;
/// when different, `GEN_LAST_SEEN` is updated, callee 0 (the refresh
/// routine) is invoked with `this` = `REFRESH_THIS`, and the result is the
/// tail call of callee 1 (the dispatch routine) with `this` = `DISPATCH_THIS`.
///
/// Original: 0x00948320 (cdecl, no stack arguments; both callees take no
/// stack arguments and receive constant `this` pointers in ECX).
lf_checker_rt::export!(cdecl, rw_00948320() -> u32 {
    unsafe {
        const TLS_INDEX: u32 = 0x017aba14;
        const THREAD_FLAG: u32 = 0x8cc;
        const GEN_COUNTER: u32 = 0x01173604;
        const GEN_LAST_SEEN: u32 = 0x011f61b0;
        const REFRESH_THIS: u32 = 0x011d95f0;
        const DISPATCH_THIS: u32 = 0x011d97f0;
        const REFRESH_CALLEE: u32 = 0;
        const DISPATCH_CALLEE: u32 = 1;

        let slot = (lf_checker_rt::global::<u32>(TLS_INDEX)).read();
        let thread = lf_checker_rt::tls_slot(slot as usize);
        let flag = ((thread.wrapping_add(THREAD_FLAG)) as *const u32).read_unaligned();
        if flag == 0 {
            return thread;
        }
        let counter = (lf_checker_rt::global::<u32>(GEN_COUNTER)).read();
        if (lf_checker_rt::global::<u32>(GEN_LAST_SEEN)).read() == counter {
            return counter;
        }
        (lf_checker_rt::global::<u32>(GEN_LAST_SEEN)).write(counter);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, lf_checker_rt::relocated(REFRESH_THIS));
        lf_checker_rt::callee_thiscall!(DISPATCH_CALLEE, u32, lf_checker_rt::relocated(DISPATCH_THIS))
    }
});
