// original: 0x00c05790 stream_register (proposed)

/// Register a streaming record with two lookup services, when eligible.
///
/// `this` points to the record. Does nothing unless the flag at `+0x02`
/// is clear and the key at `+0x08` is not `-1`. Otherwise resolves the key
/// through the lookup callee and, for a non-null handle, resolves the key
/// at `+0x10` the same way unless it is `-1` (a `-1` contributes a null
/// handle); then calls the attach callee with (the word at `+0x0c`, the
/// second handle, the word at `+0x14`) and the options callee with (the
/// word at `+0x18`, whether `+0x01` is non-zero), both against the first
/// handle's service object. No return channel is compared: the early-exit
/// path leaves the incoming `eax` untouched.
///
/// Original: 0x00c05790 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c05790(this: u32) -> u32 {
    unsafe {
        const CALLEE_LOOKUP: u32 = 1;
        const CALLEE_ATTACH: u32 = 2;
        const CALLEE_OPTS: u32 = 3;
        const MISSING: u32 = 0xffff_ffff;
        const SERVICE_OFF: u32 = 0x3c0;
        #[inline(always)]
        unsafe fn rdd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        if ((this.wrapping_add(2)) as *const u8).read() != 0 {
            return 0;
        }
        let key = rdd(this.wrapping_add(8));
        if key == MISSING {
            return 0;
        }
        let h1 = lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, 1, key);
        if h1 == 0 {
            return 0;
        }
        let key2 = rdd(this.wrapping_add(0x10));
        let h2 = if key2 == MISSING {
            0
        } else {
            lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, 1, key2)
        };
        let svc = h1.wrapping_add(SERVICE_OFF);
        lf_checker_rt::callee_thiscall!(
            CALLEE_ATTACH, u32, svc, rdd(this.wrapping_add(0x0c)), h2,
            rdd(this.wrapping_add(0x14))
        );
        let nz = u32::from(((this.wrapping_add(1)) as *const u8).read() != 0);
        lf_checker_rt::callee_thiscall!(CALLEE_OPTS, u32, svc, rdd(this.wrapping_add(0x18)), nz);
        0
    }
});
