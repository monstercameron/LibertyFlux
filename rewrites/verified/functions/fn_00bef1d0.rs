// original: 0x00bef1d0 ped_task_bind_assets (proposed)

/// Bind a ped task's resolved assets and refresh its mover block.
///
/// `this` is the task, `dst` the mover block. A setup callee runs first with
/// (`dst`, `aux`, `t`). The anchor triple (`this+0x70/+0x74/+0x78`) is copied
/// to `dst+0x14a0/+0x14a4/+0x14a8` with three tag bytes/words
/// (`this+0x84` to `dst+0x14e7`, `this+0x85` to `dst+0x14e4`,
/// `this+0x87` to `dst+0x14e6`).
///
/// Two asset slots are then resolved the same way: unless the signed handle
/// at `this+0x7c` (then `this+0x80`) is negative, a lookup callee maps it to
/// an asset, and a nonzero asset replaces the slot at `dst+0x14d0` (then
/// `dst+0x14d4`), releasing the previous occupant first when the slot is not
/// empty. Finally two zero-anchor refresh callees run only when the copied
/// anchor z is exactly zero (NaN skips them, matching the original's
/// `ucomiss`+`lahf` test), and two tail callees always run, all four addressed
/// at the mover (`dst+0x210`) with `dst` as their argument.
///
/// Original: thiscall, three stack words (`dst`, `aux`, `t`), no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00bef1d0(this: u32, dst: u32, aux: u32, t: u32) -> u32 {
    unsafe {
        const ANCHOR: u32 = 0x70;
        const HANDLE0: u32 = 0x7c;
        const HANDLE1: u32 = 0x80;
        const TAG_B0: u32 = 0x84;
        const TAG_W: u32 = 0x85;
        const TAG_B1: u32 = 0x87;
        const DST_ANCHOR: u32 = 0x14a0;
        const DST_MOVER: u32 = 0x210;
        const SLOT0: u32 = 0x14d0;
        const SLOT1: u32 = 0x14d4;
        const DST_TAG_W: u32 = 0x14e4;
        const DST_TAG_B1: u32 = 0x14e6;
        const DST_TAG_B0: u32 = 0x14e7;
        const CALLEE_SETUP: u32 = 1;
        const CALLEE_LOOKUP: u32 = 2;
        const CALLEE_RELEASE: u32 = 3;
        const CALLEE_RETAIN: u32 = 4;
        const CALLEE_ZERO_A: u32 = 5;
        const CALLEE_ZERO_B: u32 = 6;
        const CALLEE_TAIL_A: u32 = 7;
        const CALLEE_TAIL_B: u32 = 8;

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
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        lf_checker_rt::callee_thiscall!(CALLEE_SETUP, u32, this, dst, aux, t);
        wr32(dst + DST_ANCHOR, rd32(this + ANCHOR));
        wr32(dst + DST_ANCHOR + 4, rd32(this + ANCHOR + 4));
        wr32(dst + DST_ANCHOR + 8, rd32(this + ANCHOR + 8));
        wr8(dst + DST_TAG_B0, rd8(this + TAG_B0));
        wr16(dst + DST_TAG_W, rd16(this + TAG_W));
        wr8(dst + DST_TAG_B1, rd8(this + TAG_B1));

        for (handle_off, slot) in [(HANDLE0, SLOT0), (HANDLE1, SLOT1)] {
            let handle = rd32(this + handle_off) as i32;
            if handle >= 0 {
                let asset = lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, 1u32, handle as u32);
                if asset != 0 {
                    let old = rd32(dst + slot);
                    if old != 0 {
                        lf_checker_rt::callee_thiscall!(CALLEE_RELEASE, u32, old, dst + slot);
                    }
                    wr32(dst + slot, asset);
                    lf_checker_rt::callee_thiscall!(CALLEE_RETAIN, u32, asset, dst + slot);
                }
            }
        }

        // Exactly-zero anchor z runs the zero refresh pair (`==` is false
        // for NaN, so NaN skips it, matching the `ucomiss`+`lahf` test).
        let z = f32::from_bits(rd32(dst + DST_ANCHOR + 8));
        if z == 0.0 {
            lf_checker_rt::callee_thiscall!(CALLEE_ZERO_A, u32, dst + DST_MOVER, dst);
            lf_checker_rt::callee_thiscall!(CALLEE_ZERO_B, u32, dst + DST_MOVER, dst);
        }
        lf_checker_rt::callee_thiscall!(CALLEE_TAIL_A, u32, dst + DST_MOVER, dst);
        lf_checker_rt::callee_thiscall!(CALLEE_TAIL_B, u32, dst + DST_MOVER, dst);
        0
    }
});
