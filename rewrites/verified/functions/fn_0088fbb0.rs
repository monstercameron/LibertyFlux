// original: 0x0088FBB0 audSound_dispatchSlots (proposed)

/// Dispatch each live voice slot of this sound through the slot table.
///
/// `this` points to the sound and `a0` is relayed to every dispatch.
/// The eight slot bytes at `+0x48` are scanned: a slot holding 0xff is
/// skipped, otherwise a target is derived as the scalar global times the
/// slot byte plus the voice-table word for voice id `[this+0x40]`, and a
/// zero target is skipped. A non-zero target's sub-id byte at `+0x3b`
/// selects an entry in the dispatch table global, which runs (callee 1)
/// with the target and `a0`.
///
/// Original: 0x0088FBB0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088FBB0(this: u32, a0: u32) -> () {
    unsafe {
        const VOICE_ID: u32 = 0x40;
        const SLOTS: u32 = 0x48;
        const SLOT_COUNT: u32 = 8;
        const EMPTY: u8 = 0xff;
        const SCALAR_GLOBAL: u32 = 0x0115_d964;
        const TABLE_GLOBAL: u32 = 0x0115_d988;
        const DISPATCH_TABLE: u32 = 0x0115_d6b4;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f10;
        const SUB_ID: u32 = 0x3b;
        const DISPATCH: u32 = 1;

        let scalar =
            (lf_checker_rt::relocated(SCALAR_GLOBAL) as *const u32)
                .read_unaligned();
        let tab =
            (lf_checker_rt::relocated(TABLE_GLOBAL) as *const u32)
                .read_unaligned();
        let vid = ((this + VOICE_ID) as *const u8).read() as u32;
        let w = ((tab
            .wrapping_add(vid.wrapping_mul(ROW_STRIDE))
            .wrapping_add(ROW_BIAS)) as *const u32)
            .read_unaligned();
        let mut i = 0;
        while i < SLOT_COUNT {
            let si = ((this.wrapping_add(SLOTS).wrapping_add(i)) as *const u8)
                .read();
            if si != EMPTY {
                let target = scalar
                    .wrapping_mul(si as u32)
                    .wrapping_add(w);
                if target != 0 {
                    let sub = ((target.wrapping_add(SUB_ID)) as *const u8)
                        .read() as u32;
                    let dtab = lf_checker_rt::relocated(DISPATCH_TABLE);
                    let f: extern "cdecl" fn(u32, u32) -> u32 =
                        core::mem::transmute(
                            ((dtab.wrapping_add(sub.wrapping_mul(4)))
                                as *const u32)
                                .read_unaligned()
                                as usize,
                        );
                    f(target, a0);
                }
            }
            i += 1;
        }
    }
});
