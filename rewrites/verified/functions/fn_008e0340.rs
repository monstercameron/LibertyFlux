// original: 0x008e0340 lookup_validate_notify

/// Look up slot `key` in the shared pool table, validate it, notify the
/// manager, and report whether the row's header word is non-zero.
///
/// Dead slots (flag high bit) resolve to a null row and fault on the header
/// read, like the original. The notify call's low byte gates the rest: when
/// it is zero the refresh and the probe are skipped. The probe runs only for
/// rows whose header is non-zero. Returns 1 when the header is non-zero,
/// else 0.
///
/// `aux` is only forwarded to the notify and refresh calls.
///
/// Original: cdecl (key, aux).
lf_checker_rt::export!(cdecl, rw_008e0340(key: u32, aux: u32) -> u32 {
    unsafe {
        let g = lf_checker_rt::global::<u32>(0x11764C0);
        let ctx = *g as *mut u32;
        let flags = *(ctx.add(1)) as *const u8;
        let flag = *flags.add(key as usize);
        let row = if flag & 0x80 == 0 {
            let stride = *(ctx.add(3));
            ((*(ctx)) as *mut u8).byte_add(key.wrapping_mul(stride) as usize) as *mut u32
        } else {
            core::hint::black_box(core::ptr::null_mut())
        };
        lf_checker_rt::callee_thiscall!(1, u32, *g);
        let mgr = lf_checker_rt::relocated(0x1173750);
        let notify = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            mgr,
            aux,
            lf_checker_rt::relocated(0xE81E40),
            8u32
        );
        if notify as u8 != 0 {
            lf_checker_rt::callee_cdecl!(
                3,
                u32,
                row as u32,
                aux,
                lf_checker_rt::relocated(0xE81E44),
                8u32,
                0u32
            );
            if *row != 0 {
                lf_checker_rt::callee_cdecl!(4, u32, key);
            }
        }
        (*row != 0) as u32
    }
});
