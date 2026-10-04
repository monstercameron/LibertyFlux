// original: 0x0099f650 megaphone_foot_pursuit
/// Tests whether an entity's voice handle is one of four known keys.
///
/// Returns zero when callee 1 answers zero. Otherwise it initializes the
/// four global slots through callee 2 exactly once (guarded by bit 0 of a
/// fifth global) and returns whether the dword at offset 0xa8 matches any
/// of the four slots.
export!(thiscall, rw_0099f650(this: u32) -> u32 {
    unsafe {
        const GUARD: u32 = 0x12845b0;
        const SLOTS: u32 = 0x12845a0;
        const KEYS: [u32; 4] = [0xe91208, 0xe91220, 0xe9123c, 0xe91248];
        if (callee_thiscall!(1, u32, this) as u8) == 0 {
            return 0;
        }
        let g = global::<u32>(GUARD);
        if (*g & 1) == 0 {
            *g |= 1;
            let s = global::<u32>(SLOTS);
            let mut i = 0;
            while i < 4 {
                *s.add(i) = callee_cdecl!(2, u32, relocated(KEYS[i]), 0);
                i += 1;
            }
        }
        let want = *((this.wrapping_add(0xa8)) as *const u32);
        let s = global::<u32>(SLOTS);
        let mut i = 0;
        while i < 4 {
            if *s.add(i) == want {
                return 1;
            }
            i += 1;
        }
        0
    }
});
