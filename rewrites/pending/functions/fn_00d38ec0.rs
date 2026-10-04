// original: 0x00d38ec0 epoch_eligibility_probe
/// Eligibility probe for the shared epoch: resolve the owner, take the fast
/// accept when the epoch already covers it, otherwise poll the two readiness
/// helpers and accept unless the window check lands inside the dead band.
export!(stdcall, rw_00d38ec0(a1: u32) -> u32 {
    let h = callee_thiscall!(1, u32, a1);
    if h == 0 {
        return 0;
    }
    let r = callee_thiscall!(2, u32, h.wrapping_add(8));
    if r == 0 || r == a1 {
        return 0;
    }
    if unsafe { *((r + 0x218) as *const u8) } == 0
        && unsafe { *((r + 0x219) as *const u8) } != 0
    {
        let lo = unsafe { *global::<u32>(0x171fad8) }.wrapping_add(0x1f4);
        if lo > unsafe { *global::<u32>(0x11735b4) } {
            return 1;
        }
    }
    if unsafe { *((r + 0x26c) as *const u8) } & 4 != 0 {
        return 0;
    }
    let e = unsafe { *((r + 0x224) as *const u32) };
    let ready = callee_thiscall!(3, u32, e.wrapping_add(0x2e0), 0x2de, 0);
    if ready & 0xff == 0 {
        return 1;
    }
    let a = callee_thiscall!(4, u32, e.wrapping_add(0x2e0), 0x2de, 5);
    if a < 0x18 {
        return 1;
    }
    let e2 = unsafe { *((r + 0x224) as *const u32) };
    let b = callee_thiscall!(4, u32, e2.wrapping_add(0x2e0), 0x2de, 5);
    if b > 0x1d {
        return 1;
    }
    0
});
