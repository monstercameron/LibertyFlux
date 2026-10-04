// original: 0x00aba800 input_ui_slot_assign (proposed)

/// Assign one entry in a two-slot input-ui table.
///
/// `this` points to two 0x40-byte slots. Each slot holds an id at `+0x0`, a
/// sub-index at `+0x4`, a value at `+0x8`, two looked-up ids at `+0xc` and
/// `+0x10`, a parameter at `+0x14`, a 16-byte vector at `+0x20`, and a second
/// 16-byte vector at `+0x30`.
///
/// Behaviour: when `arg0` is not -1 the sub-index `arg1` must be in
/// `[0, 8)`, else the function returns. The slot whose id equals `arg0` is
/// used; if none matches, the first slot whose id is -1 (empty) is used; if
/// neither exists the function returns. The slot takes `arg0`, `arg1` and
/// `arg2`, then two names are formatted from a global table entry and looked
/// up, giving the two ids. The parameter `arg3` is stored; when it is -1 the
/// function returns without touching the vectors. Otherwise `arg4`, when
/// non-null, supplies the first vector (a null pointer zeroes only its first
/// three words, leaving the fourth untouched), and `arg5`, when non-null,
/// supplies the second vector (a null pointer zeroes its first three words
/// and writes 1.0 to the fourth).
///
/// The name table and the two format strings are read from the original
/// image. The function returns nothing meaningful. Original is thiscall with
/// six stack words.
lf_checker_rt::export!(thiscall, rw_00aba800(
    this: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
) -> u32 {
    unsafe {
        const SLOTS: u32 = 2;
        const SLOT_STRIDE: u32 = 0x40;
        const EMPTY: u32 = 0xffffffff;
        const ID_OFF: u32 = 0x0;
        const SUB_OFF: u32 = 0x4;
        const VAL_OFF: u32 = 0x8;
        const LOOKUP_A_OFF: u32 = 0xc;
        const LOOKUP_B_OFF: u32 = 0x10;
        const PARAM_OFF: u32 = 0x14;
        const VEC_A_OFF: u32 = 0x20;
        const VEC_B_OFF: u32 = 0x30;
        const NAME_TABLE: u32 = 0x103ee8c;
        const NAME_FMT_A: u32 = 0xea5768;
        const NAME_FMT_B: u32 = 0xea5770;
        const ONE_BITS: u32 = 0x3f800000;
        const CAL_FORMAT_A: u32 = 1;
        const CAL_LOOKUP: u32 = 2;
        const CAL_FORMAT_B: u32 = 3;
        const CAL_COOKIE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if arg0 != EMPTY && ((arg1 as i32) < 0 || (arg1 as i32) >= 8) {
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
        let mut slot = this;
        let mut found = false;
        for i in 0..SLOTS {
            let s = this + i * SLOT_STRIDE;
            if rd32(s + ID_OFF) == arg0 {
                slot = s;
                found = true;
                break;
            }
        }
        if !found {
            for i in 0..SLOTS {
                let s = this + i * SLOT_STRIDE;
                if rd32(s + ID_OFF) == EMPTY {
                    slot = s;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
        wr32(slot + VAL_OFF, arg2);
        wr32(slot + ID_OFF, arg0);
        wr32(slot + SUB_OFF, arg1);
        let entry = rd32(lf_checker_rt::relocated(NAME_TABLE).wrapping_add(arg0.wrapping_mul(4)));
        let mut name_a = [0u32; 8];
        lf_checker_rt::callee_cdecl!(
            CAL_FORMAT_A,
            u32,
            name_a.as_mut_ptr() as u32,
            lf_checker_rt::relocated(NAME_FMT_A),
            entry,
            arg1
        );
        wr32(
            slot + LOOKUP_A_OFF,
            lf_checker_rt::callee_cdecl!(CAL_LOOKUP, u32, name_a.as_mut_ptr() as u32),
        );
        let mut name_b = [0u32; 8];
        lf_checker_rt::callee_cdecl!(
            CAL_FORMAT_B,
            u32,
            name_b.as_mut_ptr() as u32,
            lf_checker_rt::relocated(NAME_FMT_B),
            entry,
            arg1,
            arg2.wrapping_add(0x61)
        );
        wr32(
            slot + LOOKUP_B_OFF,
            lf_checker_rt::callee_cdecl!(CAL_LOOKUP, u32, name_b.as_mut_ptr() as u32),
        );
        wr32(slot + PARAM_OFF, arg3);
        if arg3 == EMPTY {
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
        if arg4 != 0 {
            wr32(slot + VEC_A_OFF, rd32(arg4));
            wr32(slot + VEC_A_OFF + 4, rd32(arg4 + 4));
            wr32(slot + VEC_A_OFF + 8, rd32(arg4 + 8));
            wr32(slot + VEC_A_OFF + 12, rd32(arg4 + 12));
        } else {
            wr32(slot + VEC_A_OFF, 0);
            wr32(slot + VEC_A_OFF + 4, 0);
            wr32(slot + VEC_A_OFF + 8, 0);
        }
        if arg5 != 0 {
            wr32(slot + VEC_B_OFF, rd32(arg5));
            wr32(slot + VEC_B_OFF + 4, rd32(arg5 + 4));
            wr32(slot + VEC_B_OFF + 8, rd32(arg5 + 8));
            wr32(slot + VEC_B_OFF + 12, rd32(arg5 + 12));
        } else {
            wr32(slot + VEC_B_OFF, 0);
            wr32(slot + VEC_B_OFF + 4, 0);
            wr32(slot + VEC_B_OFF + 8, 0);
            wr32(slot + VEC_B_OFF + 12, ONE_BITS);
        }
        lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        0
    }
});
