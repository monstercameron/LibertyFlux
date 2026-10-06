// original: 0x0094BB80 slot_row_remap (proposed)

/// Refresh one 72-byte slot row and evaluate a float from two arguments.
///
/// The slot index comes from `SLOT_INDEX`; each slot is `SLOT_STRIDE` bytes
/// at `SLOT_BASE + index * SLOT_STRIDE`. Callee 0 picks a source row and
/// callee 1 fills an 18-word scratch buffer for it (its `this` is the
/// buffer, its stack argument the source row address). The slot's kind word
/// (unsigned equality against `KIND_GUARD_A`/`KIND_GUARD_B`) selects an
/// optional probe: for kinds 7 and 8 callee 2 runs, and callee 3 runs only
/// when the probe's low byte is zero. Then the kind word goes to callee 4,
/// the two slot floats at `-F0_BACK`/`-F1_BACK` (moved, never computed) go
/// to callee 5, the sign-extended slot byte at `-B0_BACK` converted exactly
/// to float goes to callee 6, and the two zero-extended slot bytes at
/// `-U0_BACK`/`-U1_BACK` go to callees 7 and 8. Callee 9 evaluates
/// `(arg0, arg1)` to the float result (saved and reloaded across callee
/// 10, which picks the destination row); the scratch buffer is finally
/// copied as 18 words over the destination row at
/// `ROW_TABLE + row * ROW_STRIDE`, and the float is returned in ST0.
///
/// Original: 0x0094BB80 (cdecl, two stack words, float return in ST0).
lf_checker_rt::export!(cdecl, rw_0094BB80(arg0: u32, arg1: u32) -> f32 {
    unsafe {
        const ROW_TABLE: u32 = 0x0119bf18;
        const ROW_STRIDE: u32 = 72;
        const ROW_WORDS: usize = 18;
        const SLOT_INDEX: u32 = 0x011db230;
        const SLOT_STRIDE: u32 = 0x7c;
        const SLOT_BASE: u32 = 0x011ee5ec;
        const KIND_GUARD_A: u32 = 7;
        const KIND_GUARD_B: u32 = 8;
        const F0_BACK: u32 = 0x38;
        const F1_BACK: u32 = 0x34;
        const B0_BACK: u32 = 0x03;
        const U0_BACK: u32 = 0x21;
        const U1_BACK: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let src_row: u32 = lf_checker_rt::callee_cdecl!(0, u32,);
        let mut buf = [0u32; ROW_WORDS];
        let src_addr = src_row
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(lf_checker_rt::relocated(ROW_TABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            buf.as_mut_ptr() as u32,
            src_addr
        );
        let slot = lf_checker_rt::relocated(SLOT_BASE).wrapping_add(
            (lf_checker_rt::global::<u32>(SLOT_INDEX)).read().wrapping_mul(SLOT_STRIDE),
        );
        let kind = rd32(slot);
        if kind == KIND_GUARD_A || kind == KIND_GUARD_B {
            let probe: u32 = lf_checker_rt::callee_cdecl!(2, u32, kind);
            if (probe as u8) == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, kind);
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, kind);
        let f0 = rd32(slot.wrapping_sub(F0_BACK));
        let f1 = rd32(slot.wrapping_sub(F1_BACK));
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32, f0, f1);
        let b0 = ((slot.wrapping_sub(B0_BACK)) as *const u8).read() as i8 as i32;
        let _: u32 = lf_checker_rt::callee_cdecl!(6, u32, (b0 as f32).to_bits());
        let u0 = ((slot.wrapping_sub(U0_BACK)) as *const u8).read() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, u0);
        let u1 = ((slot.wrapping_sub(U1_BACK)) as *const u8).read() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, u1);
        let value: f32 = lf_checker_rt::callee_cdecl!(9, f32, arg0, arg1);
        let dst_row: u32 = lf_checker_rt::callee_cdecl!(10, u32,);
        let dst = dst_row
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(lf_checker_rt::relocated(ROW_TABLE));
        for i in 0..ROW_WORDS {
            ((dst.wrapping_add((i as u32).wrapping_mul(4))) as *mut u32).write_unaligned(buf[i]);
        }
        value
    }
});
