// original: 0x005B5E80 free_if_flagged (proposed)

/// Release the object's buffer when its flag word is set.
///
/// Returns immediately when the flag word at +6 is zero or when the buffer
/// pointer at +0 is null; on those paths the original leaves EAX untouched,
/// so the return value is whatever EAX held on entry. A Rust rewrite cannot
/// observe incoming EAX, so the contract pins EAX to `PINNED_EAX` and this
/// rewrite returns that constant (see the proof's narrowed list). Otherwise
/// the buffer is handed to the thread heap manager's free entry (TLS slot 0
/// -> +8 -> vtable -> slot +0xc, which takes the pointer and pops it) and
/// the free answer is returned. Thiscall: object in ECX, no stack arguments.
lf_checker_rt::export!(thiscall, rw_005B5E80(this: u32) -> u32 {
    unsafe {
        const BUFFER: u32 = 0x00;
        const FLAG: u32 = 0x06;
        const PINNED_EAX: u32 = 0x1234_5678;

        if (this.wrapping_add(FLAG) as *const u16).read() == 0 {
            return PINNED_EAX;
        }
        let buf = (this.wrapping_add(BUFFER) as *const u32).read();
        if buf == 0 {
            return PINNED_EAX;
        }
        let slot = lf_checker_rt::tls_slot(0);
        let mgr = (slot.wrapping_add(8) as *const u32).read();
        let vtable = (mgr as *const u32).read();
        let entry = (vtable.wrapping_add(0xC) as *const u32).read();
        let free: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(entry as usize);
        free(mgr, buf)
    }
});
