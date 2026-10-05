// original: 0x0062BE20 procedural_reflection_dtor (proposed)

/// Destructor releasing three refcounted members.
///
/// Stamps the class vtable pointer at `+0x00`, then releases each member at
/// `+0x58`, `+0x54` and `+0x50` in order: a null member is skipped, otherwise
/// its unsigned 16-bit count at `+0x0a` is decremented and, when it reaches
/// zero on an owned member (kind byte at `+8` is 2 or 4), the member's slot-0
/// release runs with argument 1. No base call. The accumulator at return
/// holds incidental leftovers, so it is not compared (thiscall, no
/// arguments).
lf_checker_rt::export!(thiscall, rw_0062be20(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE24E0;
        ((this + 0x00) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let r1 = ((this + 0x58) as *const u32).read_unaligned();
        if r1 != 0 {
            let count = ((r1 + 0x0a) as *const u16).read_unaligned();
            if count != 0 {
                ((r1 + 0x0a) as *mut u16).write_unaligned(count.wrapping_sub(1));
                let kind = ((r1 + 8) as *const u8).read();
                let own = kind == 2 || kind == 4;
                if count == 1 && own {
                    let rvt = (r1 as *const u32).read_unaligned();
                    let rtgt = (rvt as *const u32).read_unaligned();
                    let release: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rtgt as usize);
                    release(r1, 1);
                }
            }
        }
        let r2 = ((this + 0x54) as *const u32).read_unaligned();
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
        let r3 = ((this + 0x50) as *const u32).read_unaligned();
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
        0
    }
});
