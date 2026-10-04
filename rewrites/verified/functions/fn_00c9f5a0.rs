// original: 0x00c9f5a0 CSimpleIkManager::vf2

/// Dispatch an IK-manager refresh through three probe layers.
///
/// Loads the worker at `+0x10` (a null link faults on both sides, by
/// design). Vtable slot 0x24 (callee 1) is probed first: a non-zero low
/// byte takes the tail path (callee 3). Otherwise the flag word at `+0x28`
/// masked to 0x3c0 must equal 0x180 or the masked value is returned.
/// Vtable slot 0xd0 (callee 2) is probed next: bit 3 of the byte at
/// `+0x72` of its answer takes the tail path. Otherwise the signed word at
/// `+0x2e` indexes the global pointer table: a null entry returns 0, a live
/// entry's word at `+0x128` whose byte at `+2` is set takes the tail path,
/// and anything else returns that word. All three tail jumps share one
/// stub. Indirect calls go through the fabricated objects exactly like the
/// original.
///
/// Original: 0x00c9f5a0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c9f5a0(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn rd16(a: u32) -> u16 {
        unsafe { (a as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe {
        const LINK: u32 = 0x10;
        const SLOT_PROBE: u32 = 0x24;
        const SLOT_STATE: u32 = 0xd0;
        const FLAGS: u32 = 0x28;
        const FLAG_MASK: u32 = 0x3c0;
        const FLAG_WANT: u32 = 0x180;
        const KIND: u32 = 0x2e;
        const ENTRY_OFF: u32 = 0x128;
        const STATE_BIT_OFF: u32 = 0x72;
        const STATE_BIT: u8 = 8;
        let o = rd32(this + LINK);
        let s1 = rd32(rd32(o) + SLOT_PROBE);
        let f1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(s1 as usize);
        if (f1(o) & 0xff) != 0 {
            return lf_checker_rt::callee_thiscall!(3, u32, this);
        }
        let masked = rd32(o + FLAGS) & FLAG_MASK;
        if masked != FLAG_WANT {
            return masked;
        }
        let s2 = rd32(rd32(o) + SLOT_STATE);
        let f2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(s2 as usize);
        let st = f2(o);
        if rd8(st + STATE_BIT_OFF) & STATE_BIT != 0 {
            return lf_checker_rt::callee_thiscall!(3, u32, this);
        }
        let idx = rd16(o + KIND) as i16 as i32 as u32;
        let tab = rd32((lf_checker_rt::relocated(0x0129_5cd8))
            .wrapping_add(idx.wrapping_mul(4)));
        if tab == 0 {
            return 0;
        }
        let e = rd32(tab + ENTRY_OFF);
        if rd8(e + 2) != 0 {
            e
        } else {
            lf_checker_rt::callee_thiscall!(3, u32, this)
        }
    }
});
