// original: 0x0089BB20 aud_voice_stage_update (proposed)

/// Advance the voice stage machine of an audio sound.
///
/// `this` is an audio sound object and the single stack word is an
/// unsigned position `pos`. Two slot bytes (`+0x48`, then `+0x49`) are
/// tried in turn: `0xFF` skips, otherwise the row
/// `stride * slot + row_base[byte(+0x40)]` over the global table must be
/// non-null with marker word 2 at `+6`, or the next slot is tried (both
/// failing returns 0). When `pos` is strictly past the mark at `+0xB0`
/// (UNSIGNED `ja`) the loop helper (callee 0, `thiscall` on `this` with
/// `pos`) runs. The mix helper (callee 1, `thiscall` on `this+0xDC` with
/// the float at `+0xD4` and `pos`) answers a float `tmp`; when the dword
/// at `+0xD8` is zero, `tmp` and `1 - tmp` are each scaled by the global
/// half-pi and passed through the curve helper (callee 2, float in and
/// out of `xmm0`, no stack arguments) giving `(first, second)`, otherwise
/// the pair is `(tmp, 1 - tmp)`. Each of the pair goes through the shape
/// helper (callee 3, cdecl, one float) giving `(shaped_a, shaped_b)`.
/// Each slot byte then gets a part block (skipped when the byte is `0xFF`
/// or its row is null or unmarked, same rule as above): the resolve
/// helper (callee 4, `thiscall` on `this` with `part_index` and the
/// shaped float, then with `part_index` and `pos`) answers objects that
/// go to the open helper (callee 5) and the read helper (callee 6); the
/// read answers are flags A and B (0 when the block is skipped). Returns
/// 1 unless both flags are zero. The original stages the mix answer, the
/// second curve answer and flag B in its incoming argument slot; the
/// rewrite keeps them in locals (the stack check is off for that reason;
/// all three are observed through the compared float call arguments and
/// the return value instead). The only ordered integer comparison (the
/// `pos` gate) is UNSIGNED; everything else is equality or float order
/// (the original's operand order, pinned).
///
/// Original: 0x0089BB20 (thiscall, one stack word, returns `al`).
lf_checker_rt::export!(thiscall, rw_0089BB20(this: u32, pos: u32) -> u32 {
    unsafe {
        const CAT_INDEX: u32 = 0x40;
        const SLOT_A: u32 = 0x48;
        const SLOT_B: u32 = 0x49;
        const MARK_POS: u32 = 0xB0;
        const MIX_LEVEL: u32 = 0xD4;
        const MODE: u32 = 0xD8;
        const CAT_STRIDE: u32 = 0x6F40;
        const TABLE_ROW: u32 = 0x6F10;
        const MARKER_OFF: u32 = 6;
        const MARKER_WANT: u16 = 2;
        const NO_SLOT: u8 = 0xFF;
        const G_HALF_PI: u32 = 0xFE8978;
        const G_ONE: u32 = 0xFE88E8;
        const G_STRIDE: u32 = 0x115D964;
        const G_TABLE_BASE: u32 = 0x115D988;
        const SHAPE_OBJ: u32 = 0xDC;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Marked row for a slot byte, or null when skipped/unmarked.
        unsafe fn marked_row(this: u32, slot_off: u32) -> u32 {
            unsafe {
                let slot = rd8(this + slot_off);
                if slot == NO_SLOT {
                    return 0;
                }
                let stride = (lf_checker_rt::global::<u32>(G_STRIDE)).read_unaligned();
                let base = (lf_checker_rt::global::<u32>(G_TABLE_BASE)).read_unaligned();
                let cat = rd8(this + CAT_INDEX) as u32;
                let row_base = rd32(
                    base.wrapping_add(cat.wrapping_mul(CAT_STRIDE)).wrapping_add(TABLE_ROW),
                );
                let row = stride.wrapping_mul(slot as u32).wrapping_add(row_base);
                if row == 0 || rd16(row.wrapping_add(MARKER_OFF)) != MARKER_WANT {
                    return 0;
                }
                row
            }
        }

        if marked_row(this, SLOT_A) == 0 && marked_row(this, SLOT_B) == 0 {
            return 0;
        }
        // Unsigned position gate.
        if pos > rd32(this + MARK_POS) {
            lf_checker_rt::callee_thiscall!(0, u32, this, pos);
        }
        let tmp: f32 = lf_checker_rt::callee_thiscall!(1, f32, this + SHAPE_OBJ, rdf(this + MIX_LEVEL).to_bits(), pos);
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(G_ONE)));
        let half_pi = f32::from_bits(rd32(lf_checker_rt::relocated(G_HALF_PI)));
        let (first, second);
        if rd32(this + MODE) != 0 {
            first = tmp;
            second = sub(one, tmp);
        } else {
            let a: u32 = lf_checker_rt::callee_cdecl!(2, u32, mul(tmp, half_pi).to_bits());
            first = f32::from_bits(a);
            let b: u32 = lf_checker_rt::callee_cdecl!(2, u32, mul(sub(one, tmp), half_pi).to_bits());
            second = f32::from_bits(b);
        }
        let shaped_a: f32 = lf_checker_rt::callee_cdecl!(3, f32, first.to_bits());
        let shaped_b: f32 = lf_checker_rt::callee_cdecl!(3, f32, second.to_bits());
        // Part block per slot; each flag is 0 when its block is skipped.
        let mut flag_a = 0u8;
        if marked_row(this, SLOT_A) != 0 {
            let obj_a: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 0, shaped_a.to_bits());
            lf_checker_rt::callee_thiscall!(5, u32, obj_a);
            let obj_b: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 0, pos);
            let ans: u32 = lf_checker_rt::callee_thiscall!(6, u32, obj_b);
            flag_a = ans as u8;
        }
        let mut flag_b = 0u8;
        if marked_row(this, SLOT_B) != 0 {
            let obj_a: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 1, shaped_b.to_bits());
            lf_checker_rt::callee_thiscall!(5, u32, obj_a);
            let obj_b: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 1, pos);
            let ans: u32 = lf_checker_rt::callee_thiscall!(6, u32, obj_b);
            flag_b = ans as u8;
        }
        if flag_a != 0 || flag_b != 0 {
            1
        } else {
            0
        }
    }
});
