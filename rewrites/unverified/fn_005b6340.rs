// original: 0x005B6340 dup_wide_string (proposed)

/// Duplicate a NUL-terminated wide-character string.
///
/// A null source returns null with no call. Otherwise scans wide characters
/// two bytes at a time for the NUL word; with C characters found, allocates
/// `(C + 1) * 2` bytes through the thread heap manager's alloc entry (TLS
/// slot 0 -> +8 -> vtable -> slot +8, taking (size, 2, 0)), where a multiply
/// overflow instead asks for 0xFFFFFFFF, then copies `(C + 1) * 2` bytes
/// (the multiplication wrapping). The original calls the CRT memmove for the
/// copy; it runs natively on the original side while this rewrite performs
/// the identical byte move inline (overlap-safe, like memmove). Returns the
/// buffer. Thiscall: source in ECX.
lf_checker_rt::export!(thiscall, rw_005B6340(src: u32) -> u32 {
    unsafe {
        if src == 0 {
            return 0;
        }
        let mut p = src;
        while ((p as *const u16).read_unaligned() != 0) {
            p = p.wrapping_add(2);
        }
        let n = p.wrapping_sub(src).wrapping_shr(1).wrapping_add(1);
        let (doubled, overflowed) = n.overflowing_mul(2);
        let ask = if overflowed { 0xFFFF_FFFF } else { doubled };
        let slot = lf_checker_rt::tls_slot(0);
        let mgr = (slot.wrapping_add(8) as *const u32).read();
        let vtable = (mgr as *const u32).read();
        let entry = (vtable.wrapping_add(8) as *const u32).read();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(entry as usize);
        let buf = alloc(mgr, ask, 2, 0);
        core::ptr::copy(
            src as *const u8,
            buf as *mut u8,
            n.wrapping_mul(2) as usize,
        );
        buf
    }
});
