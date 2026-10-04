// original: 0x00d39030 ranged_blend_dispatch
/// Ranged blend dispatch: validate the state window, resolve the blend
/// source, run the wide blend, and forward the sampled weight to the live
/// mixer when one is published. The weight travels bit-exact; no arithmetic
/// touches it here.
export!(thiscall, rw_00d39030(this: u32, a1: u32) -> u32 {
    unsafe fn word(addr: u32) -> u32 {
        *(addr as *const u32)
    }
    let st = unsafe { word(this + 0x34) };
    if st < 0x0b || st >= 0x0f {
        return callee_thiscall!(4, u32, this);
    }
    if unsafe { word(this + 0x50) } == 0xffff_ffff {
        return callee_thiscall!(4, u32, this);
    }
    if unsafe { word(this + 0x54) } == 0xffff_ffff {
        return callee_thiscall!(4, u32, this);
    }
    if unsafe { word(this + 0x10) } != 0 {
        return unsafe { word(this + 0x54) };
    }
    let c = unsafe { word(a1 + 0x78) };
    let p = callee_thiscall!(1, u32, c, unsafe { word(this + 0x54) });
    let mut tmp = 0u32;
    if p != 0 {
        tmp = unsafe { word(p + 0x4c) };
    }
    let r2 = callee_stdcall!(
        2,
        u32,
        a1,
        unsafe { word(this + 0x50) },
        unsafe { word(this + 0x54) },
        0x447a0000,
        2
    );
    if p != 0 {
        // Under scripted helpers the status word below never becomes
        // nonzero (only a real helper write could set it), so this call is
        // modeled but not covered; see the lane report.
        let live = unsafe { word(this + 0x10) };
        if live != 0 {
            return callee_thiscall!(3, u32, live, tmp);
        }
    }
    r2
});
