// original: 0x00d38e70 shared_epoch_refresh
/// Refresh the shared epoch: resolve the owner through two helpers and,
/// when it is live and distinct, publish either the current or the
/// alternate epoch depending on its pending flag.
export!(stdcall, rw_00d38e70(a1: u32) -> u32 {
    let h = callee_thiscall!(1, u32, a1);
    if h == 0 {
        return 0;
    }
    let r = callee_thiscall!(2, u32, h.wrapping_add(8));
    if r == 0 || r == a1 {
        return r;
    }
    if unsafe { *((r + 0x218) as *const u8) } != 0 {
        return r;
    }
    let v = if unsafe { *((r + 0x219) as *const u8) } != 0 {
        unsafe { *global::<u32>(0x11735b4) }
    } else {
        unsafe { *global::<u32>(0x171fad8) }
    };
    unsafe { *global::<u32>(0x171fad8) = v };
    v
});
