// original: 0x005B5FB0 vec_init_24 (proposed)

/// Initialise an empty 24-byte-element vector with room for `count`.
///
/// Zeroes the vector header (begin, end, cap), then, unless `count` is zero,
/// allocates `count * 24` bytes through the thread heap manager's alloc
/// entry (TLS slot 0 -> +8 -> vtable -> slot +8, taking (size, 0x10, 0)).
/// Begin and end become the buffer (equal: the vector starts empty) and cap
/// becomes buffer + `count * 24`, all with wrapping arithmetic, so a null
/// buffer still yields a non-null cap without any store through it. Counts
/// above 0x0aaaaaaa (*unsigned*) abort the process; the proof never feeds
/// those (see the narrowed list). The second stack argument is unread.
/// Returns the vector. Thiscall: vector in ECX.
lf_checker_rt::export!(thiscall, rw_005B5FB0(this: u32, count: u32, _unused: u32) -> u32 {
    unsafe {
        const ELEM: u32 = 24;
        const ALLOC_FLAG: u32 = 0x10;

        (this.wrapping_add(0) as *mut u32).write(0);
        (this.wrapping_add(4) as *mut u32).write(0);
        (this.wrapping_add(8) as *mut u32).write(0);
        let buf = if count == 0 {
            0
        } else {
            let slot = lf_checker_rt::tls_slot(0);
            let mgr = (slot.wrapping_add(8) as *const u32).read();
            let vtable = (mgr as *const u32).read();
            let entry = (vtable.wrapping_add(8) as *const u32).read();
            let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(entry as usize);
            alloc(mgr, count.wrapping_mul(ELEM), ALLOC_FLAG, 0)
        };
        (this.wrapping_add(8) as *mut u32).write(buf.wrapping_add(count.wrapping_mul(ELEM)));
        (this.wrapping_add(0) as *mut u32).write(buf);
        (this.wrapping_add(4) as *mut u32).write(buf);
        this
    }
});
