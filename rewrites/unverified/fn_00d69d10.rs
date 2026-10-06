// original: 0x00d69d10 CReplayRolloverMessage::vf2
/// Render the rollover message when visible (original 0x00D69D10,
/// thiscall/0).
///
/// Returns at once when the visible byte at `this+0x1d` is zero; when the
/// shown byte at `this+0x1c` is zero it also needs callee 1's low byte (on
/// the member at `this+4`) nonzero. Otherwise it tests the two extent floats
/// at `this+8`/`this+12` through callee 2 (whose two stack words sit in
/// pre-reserved scratch and which pops nothing): a zero low byte skips the
/// scale computation. In it, callee 3's low byte picks one of the two divisor
/// dwords (globals at file VA 0x0105C884/0x0105C888), callee 4 feeds an x87
/// float, and the scale `(fed + K) / (divisor as f32)` is formed with the
/// constant float at file VA 0x00FE8B20, in that operation order (a zero
/// divisor yields infinity/NaN, never a fault). Then callee 5 runs on `this`,
/// callee 6 (thiscall/1 on the constant object) takes `this+0x20`, and
/// callee 7 takes five words: the scale (or the first extent float when the
/// computation was skipped), the second extent float, callee 6's answer, and
/// two -1 words. Callee 8 runs last and the shown byte is cleared. All
/// callee-answer tests are low-byte zero/nonzero checks. Returns nothing
/// meaningful.
lf_checker_rt::export!(thiscall, rw_00d69d10(this_ptr: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 4;
        const EXT0_OFF: u32 = 8;
        const EXT1_OFF: u32 = 0xc;
        const SHOWN_OFF: u32 = 0x1c;
        const VISIBLE_OFF: u32 = 0x1d;
        const BUF_OFF: u32 = 0x20;
        const DIV_A: u32 = 0x0105C884;
        const DIV_B: u32 = 0x0105C888;
        const ADD_K: u32 = 0x00FE8B20;
        const DRAW_OBJ: u32 = 0x0116BFF0;
        if ((this_ptr + VISIBLE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let member =
            ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        if ((this_ptr + SHOWN_OFF) as *const u8).read() == 0 {
            let present: u32 =
                lf_checker_rt::callee_thiscall!(1, u32, member);
            if (present & 0xFF) == 0 {
                return 0;
            }
        }
        let f0 =
            ((this_ptr + EXT0_OFF) as *const u32).read_unaligned();
        let f1 =
            ((this_ptr + EXT1_OFF) as *const u32).read_unaligned();
        let inside: u32 =
            lf_checker_rt::callee_thiscall!(2, u32, member, f0, f1);
        let mut first = f0;
        if (inside & 0xFF) != 0 {
            let pick: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
            let slot = if (pick & 0xFF) != 0 { DIV_B } else { DIV_A };
            let divisor =
                (lf_checker_rt::relocated(slot) as *const u32)
                    .read_unaligned();
            let fed: f32 = lf_checker_rt::callee_thiscall!(4, f32, member);
            let k = f32::from_bits(
                (lf_checker_rt::relocated(ADD_K) as *const u32)
                    .read_unaligned(),
            );
            let scale = (fed + k) / (divisor as i32 as f32);
            first = scale.to_bits();
        }
        lf_checker_rt::callee_thiscall!(5, u32, this_ptr);
        let drawn: u32 = lf_checker_rt::callee_thiscall!(6, u32,
            lf_checker_rt::relocated(DRAW_OBJ), this_ptr + BUF_OFF);
        lf_checker_rt::callee_cdecl!(7, u32, first, f1, drawn,
            0xFFFFFFFFu32, 0xFFFFFFFFu32);
        let _: u32 = lf_checker_rt::callee_cdecl!(8, u32,);
        ((this_ptr + SHOWN_OFF) as *mut u8).write(0);
        0
    }
});
