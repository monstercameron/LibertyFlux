// original: 0x0062D830 procedural_texture_skyhat_dtor (proposed)

/// Destructor: one plain release, two refcounted releases, tail to base.
///
/// Stamps the class vtable pointer at `+0x00`, releases the non-null member
/// at `+0x44` through its slot-0 release with argument 1, then releases the
/// refcounted members at `+0x50` and `+0x4c` (decrement the unsigned 16-bit
/// count at `+0x0a`; release with argument 1 when it reaches zero on an owned
/// member, kind byte at `+8` is 2 or 4). Finally tails to the base destructor
/// (patched tail callee) with `this`, whose answer is the answer (thiscall,
/// no arguments).
lf_checker_rt::export!(thiscall, rw_0062d830(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE2478;
        const CALLEE_BASE: u32 = 4;
        ((this + 0x00) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let r1 = ((this + 0x44) as *const u32).read_unaligned();
        if r1 != 0 {
            let rvt = (r1 as *const u32).read_unaligned();
            let rtgt = (rvt as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rtgt as usize);
            release(r1, 1);
        }
        let r2 = ((this + 0x50) as *const u32).read_unaligned();
        if r2 != 0 {
            let count = ((r2 + 0x0a) as *const u16).read_unaligned();
            if count != 0 {
                ((r2 + 0x0a) as *mut u16).write_unaligned(count.wrapping_sub(1));
                let kind = ((r2 + 8) as *const u8).read();
                let own = kind == 2 || kind == 4;
                if count == 1 && own {
                    let rvt = (r2 as *const u32).read_unaligned();
                    let rtgt = (rvt as *const u32).read_unaligned();
                    let release: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rtgt as usize);
                    release(r2, 1);
                }
            }
        }
        let r3 = ((this + 0x4c) as *const u32).read_unaligned();
        if r3 != 0 {
            let count = ((r3 + 0x0a) as *const u16).read_unaligned();
            if count != 0 {
                ((r3 + 0x0a) as *mut u16).write_unaligned(count.wrapping_sub(1));
                let kind = ((r3 + 8) as *const u8).read();
                let own = kind == 2 || kind == 4;
                if count == 1 && own {
                    let rvt = (r3 as *const u32).read_unaligned();
                    let rtgt = (rvt as *const u32).read_unaligned();
                    let release: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rtgt as usize);
                    release(r3, 1);
                }
            }
        }
        lf_checker_rt::callee_thiscall!(CALLEE_BASE, u32, this)
    }
});
