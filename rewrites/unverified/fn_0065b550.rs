// original: 0x0065B550 rage::AtmosphericScattering::vf0 (symbols)

/// Deleting destructor with an inline refcounted-member release.
///
/// Stamps the class vtable pointer at `+0x00`, then releases the member at
/// `MEMBER` (`+104`): a null member is skipped, otherwise its unsigned
/// 16-bit count at `+0x0a` is decremented and, when it reaches zero on an
/// owned member (kind byte at `+8` is 2 or 4), the member's slot-0 release
/// runs with argument 1. Then the base destructor runs (patched callee) and,
/// when the flag's low bit is set, the object is freed through the
/// thread-local allocator's free slot (`+0x0c`). Returns `this` (thiscall,
/// one argument).
lf_checker_rt::export!(thiscall, rw_0065b550(this: u32, flag: u32) -> u32 {
    unsafe {
        const MEMBER: u32 = 0x68;
        const VTABLE: u32 = 0xFE3010;
        const CALLEE_BASE: u32 = 2;
        ((this + 0x00) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let rel = ((this + MEMBER) as *const u32).read_unaligned();
        if rel != 0 {
            let count = ((rel + 0x0a) as *const u16).read_unaligned();
            if count != 0 {
                ((rel + 0x0a) as *mut u16).write_unaligned(count.wrapping_sub(1));
                let kind = ((rel + 8) as *const u8).read();
                let own = kind == 2 || kind == 4;
                if count == 1 && own {
            let rvt = (rel as *const u32).read_unaligned();
            let rtgt = (rvt as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rtgt as usize);
            release(rel, 1);
                }
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_BASE, u32, this);
        if flag & 1 == 0 {
            return this;
        }
        let holder = lf_checker_rt::tls_slot(0);
        let frobj = ((holder + 8) as *const u32).read_unaligned();
        let fvt = (frobj as *const u32).read_unaligned();
        let ftgt = ((fvt + 0x0c) as *const u32).read_unaligned();
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ftgt as usize);
        free(frobj, this);        this
    }
});
