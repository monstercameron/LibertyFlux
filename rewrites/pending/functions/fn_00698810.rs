// original: 0x00698810 rage::crCreatureComponentSkeleton::vf8
/// Skeleton release guard: releases the inner object unless the flag at
/// +0x11 is set, then forwards two of its words unless the flag at +0x10
/// is set. Returns the last callee answer; when neither call fires the
/// original returns whatever was in EAX on entry (contract pins +0x11 to
/// zero so that path never runs; the zero below is a placeholder for it).
lf_k2_rt::export!(thiscall, rw_00698810(this: *mut u8) -> u32 {
    unsafe {
        let obj = *((this.add(0x0c)) as *const u32);
        let mut ans = 0u32;
        if *this.add(0x11) == 0 {
            ans = lf_k2_rt::callee_thiscall!(1, u32, obj);
        }
        if *this.add(0x10) == 0 {
            let b = *((obj as *const u8).add(8) as *const u32);
            let c = *((obj as *const u8).add(0x14) as *const u32);
            ans = lf_k2_rt::callee_thiscall!(2, u32, obj, b, c);
        }
        ans
    }
});
