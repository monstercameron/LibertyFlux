// original: 0x008e0340 lookup_validate_notify

/// Look up slot `key` in the shared pool table, validate the row through the
/// pool helper, notify the manager of the row, refresh the row, then return
/// the row's header word.
///
/// Dead slots (flag high bit set) fault: the header read lands at address 0.
/// The manager object lives at a fixed data address; the notify call passes
/// the key, a fixed descriptor address and a channel of 8, and the refresh
/// passes the row, the key, a second descriptor, the channel and a zero tag.
///
/// Original: cdecl (key, aux). `aux` is only forwarded.
lf_checker_rt::export!(cdecl, rw_008e0340(key: u32, aux: u32) -> u32 {
    unsafe {
        let ctx = lf_checker_rt::global::<u32>(lf_checker_rt::relocated(0x11764C0)) as *mut u32;
        let base = *ctx as *mut u32;
        let flags = *ctx.add(1) as *const u8;
        let flag = *flags.add(key as usize);
        let row = if flag & 0x80 == 0 {
            let stride = *ctx.add(3);
            base.byte_add((key.wrapping_mul(stride)) as usize)
        } else {
            core::hint::black_box(core::ptr::null_mut())
        };
        lf_checker_rt::callee_thiscall!(1, u32, ctx as u32);
        let mgr = lf_checker_rt::relocated(0x1173750) as *mut u32;
        lf_checker_rt::callee_thiscall!(2, u32, mgr as u32, key, lf_checker_rt::relocated(0xE81E40) as u32, 8u32);
        lf_checker_rt::callee_cdecl!(3, u32, row as u32, key, lf_checker_rt::relocated(0xE81E44) as u32, 8u32, 0u32);
        lf_checker_rt::callee_cdecl!(4, u32, key);
        *row
    }
});
