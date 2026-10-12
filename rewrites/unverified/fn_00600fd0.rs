// original: 0x00600FD0 push_allocator_scope (proposed)

/// Push one allocator scope on the thread state and return `this`.
///
/// The thread state reached through TLS slot 0 holds two allocator
/// pointers, `CURRENT` (+0x08) and `NEXT` (+0x10). When they are equal
/// there is nothing to push and the function only bumps the re-entry
/// count at `DEPTH` (+0x68). Otherwise it installs `NEXT` as current and
/// stashes the old current pointer at `SAVED` (+0x64). Thiscall: `this`
/// in ECX, returned unchanged in EAX; the TLS value is read through the
/// checker's TLS mirror.
lf_checker_rt::export!(thiscall, rw_00600FD0(this: u32) -> u32 {
    unsafe {
        const CURRENT: u32 = 0x08;
        const NEXT: u32 = 0x10;
        const SAVED: u32 = 0x64;
        const DEPTH: u32 = 0x68;

        let state = lf_checker_rt::tls_slot(0);
        let cur = ((state + CURRENT) as *const u32).read();
        let nxt = ((state + NEXT) as *const u32).read();
        if cur == nxt {
            let d = ((state + DEPTH) as *const u32).read();
            ((state + DEPTH) as *mut u32).write(d.wrapping_add(1));
        } else {
            ((state + CURRENT) as *mut u32).write(nxt);
            ((state + SAVED) as *mut u32).write(cur);
        }
        this
    }
});
