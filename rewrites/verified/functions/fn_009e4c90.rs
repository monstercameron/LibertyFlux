// original: 0x009E4C90 ped_task_slot_probe (proposed)

/// Probe a ped task slot through the object's virtual table, then run two
/// validation stages over a scratch packet built on the stack.
///
/// `this` is the task object, `arg0` an auxiliary object passed through to
/// both stages, `arg1` a second object whose word at `+0x20` selects the
/// context for the second stage (null selects `arg1 + 0x10`, otherwise the
/// pointed-to object `+ 0x30`).
///
/// The virtual slot at `+0xb8` is called with `(out2, out1, 4, arg1)`, where
/// `out1`/`out2` are two scratch words (the first pre-zeroed). Its first
/// out-word is an index: the pair (`this`, `arg1`) is stored at that index
/// of a scratch array, and the index plus two travels with both stages. The
/// packet holds three copies of a float triple read from writable globals,
/// a `0xffff` marker, and zero padding. The first stage (five stack words:
/// `arg0`, packet, index+2, `0x8e`, -1) returning a non-zero low byte, or
/// the second stage (nine words) returning non-zero, answers 0; both zero
/// answers 1.
///
/// The low byte carries the boolean; the upper three bytes are the last
/// stage answer's with the low byte cleared. The out-index answers are
/// expected small (0-2); larger values would write past the scratch array.
///
/// Original: 0x009E4C90 (thiscall, this + two stack words).
lf_checker_rt::export!(thiscall, rw_009E4C90(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0xB8;
        const SLOT1: u32 = 0x14;
        const SLOT2: u32 = 0x18;
        const ARRAY_BASE: u32 = 0x18;
        const ARRAY_NEXT: u32 = 0x1C;
        const BUF_A: u32 = 0x18;
        const BUF_B: u32 = 0x30;
        const MARKER_OFF: u32 = 0x7C;
        const MARKER: u32 = 0xFFFF;
        const GLOB2: u32 = 0x01B4B320;
        const GLOB1: u32 = 0x01B4B324;
        const GLOB0: u32 = 0x01B4B328;
        const STAGE_TAG: u32 = 0x8E;
        const LOOKUP_CALLEE: u32 = 1;
        const STAGE1_CALLEE: u32 = 2;
        const STAGE2_CALLEE: u32 = 3;
        const LOW_MASK: u32 = 0xFFFF_FF00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn fget(base: u32, off: u32) -> u32 {
            unsafe { (base.wrapping_add(off) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn fset(base: u32, off: u32, v: u32) {
            unsafe { (base.wrapping_add(off) as *mut u32).write(v) }
        }

        // Scratch packet modelling the original's frame. Zero-initialised:
        // untouched slots read the checker's zero stack fill on both sides.
        let mut frame = [0u32; 40];
        let base = frame.as_mut_ptr() as u32;
        fset(base, SLOT1, 0);
        let table = rd32(this);
        let target = rd32(table.wrapping_add(VTABLE_SLOT));
        let lookup: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        lookup(
            this,
            base.wrapping_add(SLOT2),
            base.wrapping_add(SLOT1),
            4,
            arg1,
        );
        let out1 = fget(base, SLOT1);
        let at = out1.wrapping_mul(4);
        fset(base, ARRAY_BASE.wrapping_add(at), this);
        fset(base, ARRAY_NEXT.wrapping_add(at), arg1);
        let idx = out1.wrapping_add(2);
        fset(base, SLOT1, idx);
        let g2 = rd32(lf_checker_rt::relocated(GLOB2));
        let g1 = rd32(lf_checker_rt::relocated(GLOB1));
        let g0 = rd32(lf_checker_rt::relocated(GLOB0));
        let mut b = 0x40u32;
        while b <= 0x60 {
            fset(base, b, g2);
            fset(base, b.wrapping_add(4), g1);
            fset(base, b.wrapping_add(8), g0);
            b = b.wrapping_add(0x10);
        }
        fset(base, MARKER_OFF, MARKER);
        let stage1: u32 = lf_checker_rt::callee_cdecl!(
            STAGE1_CALLEE,
            u32,
            arg0,
            base.wrapping_add(SLOT2),
            idx,
            STAGE_TAG,
            0xFFFF_FFFF
        );
        if stage1 & 0xFF != 0 {
            return stage1 & LOW_MASK;
        }
        let entry = rd32(arg1.wrapping_add(0x20));
        let ctx = if entry == 0 {
            arg1.wrapping_add(0x10)
        } else {
            entry.wrapping_add(0x30)
        };
        let stage2: u32 = lf_checker_rt::callee_cdecl!(
            STAGE2_CALLEE,
            u32,
            ctx,
            arg0.wrapping_add(0x30),
            base.wrapping_add(BUF_A),
            base.wrapping_add(BUF_B),
            STAGE_TAG,
            1,
            idx,
            0,
            4
        );
        if stage2 != 0 {
            return stage2 & LOW_MASK;
        }
        1
    }
});
