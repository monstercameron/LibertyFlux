// original: 0x00963F30 replay_target_open (proposed)
//
// Checked-in text of this file is in out/rewrites/fn_00963F30.rs; the mutant
// below it is in-crate only, never shipped.

/// Open a replay target: compute a buffer size from fixed tables, allocate
/// and construct the target graph, run it through a hook and a format
/// probe, publish the result, and tear everything down on any failure.
///
/// With no arguments. The prologue derives a size from four fixed doubles
/// (sign-bit selection, a 2^52 round-trip that lands on 5821.0, conversion
/// to float, truncation toward zero through the x87 control word, times 4 =
/// 23284) and allocates three blocks: the sized one is kept in `KEEP`, the
/// 0x34 one is constructed (or `HANDLE` stays null), the 0x10 one is
/// constructed into a local. A fixed pair of names is registered, an
/// eight-argument allocator builds the target from the `DIM_W`/`DIM_H`
/// globals, an init helper takes scratch, and a hook object reached through
/// `HOOK_HOLDER` (vtable slot +0x38) receives ("REPLAY_TARGET"-family name,
/// 3, dims, 0x20, param-block pointer) with the param block [0,0,1,1,1,0,1,0].
/// A null at any of the five live checks (`HANDLE`, target, hook result,
/// small object, `KEEP`) jumps to the corresponding teardown.
///
/// The main sequence formats a descriptor from fixed table bytes, clamps the
/// `MODE` global to 0/1 for a gate helper, resolves two names, reuses the stale dimension word as the
/// select word for the format choice, and runs
/// a result check: on failure the tail flag is cleared and control goes to
/// teardown, otherwise ids are published to `SLOT`/`SLOT_VAL`/`SLOT_ID` and a
/// battery of writers, two data-table calls, and four notifiers runs. A
/// table of five qwords plus a dword is copied from `TAB_SRC` to `TAB_DST`,
/// three table words are then set (4, 0x63, 3), two flags are raised, and an
/// optional release helper runs. The tail frees the formatted struct, the
/// small object, the target ( refcount drop, freeing at zero), the hook
/// result (virtual slot 0 with argument 1), and `KEEP`, zeroes `HANDLE` and
/// `KEEP`, and notifies the slot. The return value is the init helper's
/// answer on the success path (the set tail flag skips the teardown block
/// and the notifier) and the slot notifier's answer on every teardown path.
/// Every exit runs the security-cookie check.
///
/// Frame sharing the rewrite models explicitly: the prologue's 64-bit
/// conversion slot is later passed (value 5821, high 0) to three helpers and
/// its words flow into two more calls; the control-word slot (0xF7F) feeds
/// the select word twice; the descriptor area feeds four helpers at three
/// offsets. All are deterministic consequences of explicit writes plus the
/// checker's zero fill.
///
/// Original: 0x00963F30 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00963F30() -> u32 {
    unsafe {
        const DIM_W: u32 = 0x01037750;
        const DIM_H: u32 = 0x01037754;
        const HANDLE: u32 = 0x011F66A0;
        const KEEP: u32 = 0x011F66A4;
        const NAME_SRC: u32 = 0x011F6954;
        const TAB_DST: u32 = 0x011F68FC;
        const TAB_SRC: u32 = 0x01160E8C;
        const FLAG_A: u32 = 0x011F7048;
        const FLAG_B: u32 = 0x0115A43C;
        const FLAG_C: u32 = 0x0115A440;
        const FLAG_D: u32 = 0x0118DC44;
        const OBJ_MAYBE: u32 = 0x0118D800;
        const SLOT: u32 = 0x012088B8;
        const SLOT_VAL: u32 = 0x012088BC;
        const SLOT_ID: u32 = 0x012088C8;
        const HOOK_HOLDER: u32 = 0x017F5630;
        const HOOK_SLOT: u32 = 0x38;
        const OBJ_THIS: u32 = 0x012088B8;
        const OBJ_FMT: u32 = 0x0116BFF0;
        const OBJ_REL: u32 = 0x01162630;
        const KEY_OBJ: u32 = 0x011695D8;
        const AUX_OBJ: u32 = 0x0116A5D8;
        const FMT_BIG: u32 = 0x00E89AD0;
        const FMT_MID: u32 = 0x00E89AF4;
        const FMT_SMALL: u32 = 0x00E89B14;
        const NAME_A: u32 = 0x00E89A58;
        const NAME_B: u32 = 0x00E89A70;
        const HOOK_NAME: u32 = 0x00E89A84;
        const ARG_CONST: u32 = 0x00E89ACC;
        const S2_NAME: u32 = 0x00E89B74;
        const S2B_NAME: u32 = 0x00E89B98;
        const W1_A: u32 = 0x00E89C50;
        const W1_B: u32 = 0x00E89C88;
        const W2_A: u32 = 0x00E89CA4;
        const W2_B: u32 = 0x00E89CD0;
        const TBL_B4: u32 = 0x00E89AB4;
        const TBL_BC: u32 = 0x00E89ABC;
        const TBL_C4: u32 = 0x00E89AC4;
        const TBL_C8: u32 = 0x00E89AC8;
        const TBL_CA: u32 = 0x00E89ACA;
        const DTABLE_1: u32 = 0x00E732A4;
        const DTABLE_2: u32 = 0x00E732B0;
        const PRO_D3: u32 = 0x00E8AEB0;
        const PRO_D4: u32 = 0x00FE8D20;
        const PRO_D2: u32 = 0x00FE8BE0;
        const PRO_D5: u32 = 0x00FE89E0;
        const MODE: u32 = 0x01109A54;
        const INIT_CW: u32 = 0x37F;
        const INVALID_I64: i64 = i64::MIN;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64p(a: u32) -> u64 {
            unsafe { (rd32(a) as u64) | ((rd32(a.wrapping_add(4)) as u64) << 32) }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(47, u32,) };
        }
        /// The original's `fistp` with the truncate bit set: truncation
        /// toward zero, i64::MIN for NaN, infinities and out-of-range
        /// values (Rust's `as` saturates instead).
        #[inline(always)]
        fn cvt64(x: f32) -> i64 {
            if x.is_nan() || x >= 9223372036854775808.0 || x < -9223372036854775808.0 {
                INVALID_I64
            } else {
                x as i64
            }
        }

        let r = lf_checker_rt::relocated;
        let root: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, root);

        // Prologue: fixed-double derivation of the buffer size.
        let b3 = rd64p(r(PRO_D3));
        let x4p = rd64p(r(PRO_D4)) & b3;
        let x0p = b3 ^ x4p;
        let lt: u64 = if core::hint::black_box(f64::from_bits(x0p))
            < core::hint::black_box(f64::from_bits(rd64p(r(PRO_D2))))
        {
            0xFFFFFFFFFFFFFFFF
        } else {
            0
        };
        let f2 = f64::from_bits((rd64p(r(PRO_D2)) & lt) | x4p);
        let mut x1 =
            core::hint::black_box(f64::from_bits(b3)) + core::hint::black_box(f2);
        x1 = core::hint::black_box(x1) - core::hint::black_box(f2);
        let x0f =
            core::hint::black_box(x1) - core::hint::black_box(f64::from_bits(b3));
        let x4f = f64::from_bits(x4p);
        let m: u64 = if !(x0f <= x4f) { 0xFFFFFFFFFFFFFFFF } else { 0 };
        let adj = f64::from_bits(rd64p(r(PRO_D5)) & m);
        x1 = core::hint::black_box(x1) - core::hint::black_box(adj);
        let trunc = cvt64(x1 as f32);
        let trunc_u = trunc as u64;
        let size = (trunc_u as u32).wrapping_mul(4);
        // The 64-bit conversion slot, later shared with five helpers.
        let conv_slot = [(trunc_u as u32), (trunc_u >> 32) as u32];
        // The f32 store clobbered by fnstcw: high half of the prologue
        // float plus the fninit control word.
        let frank_slot = [
            ((x1 as f32).to_bits() & 0xFFFF0000) | INIT_CW,
            conv_slot[0],
        ];

        let n1: u32 = lf_checker_rt::callee_cdecl!(3, u32, size);
        wr32(r(KEEP), n1);
        let n2: u32 = lf_checker_rt::callee_cdecl!(4, u32, 0x34);
        if n2 == 0 {
            wr32(r(HANDLE), 0);
        } else {
            let built: u32 = lf_checker_rt::callee_thiscall!(6, u32, n2);
            wr32(r(HANDLE), built);
        }
        let this = r(OBJ_THIS);
        let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, r(NAME_B), r(NAME_A));
        let dim_w = rd32(r(DIM_W));
        let dim_h = rd32(r(DIM_H));
        let target: u32 =
            lf_checker_rt::callee_cdecl!(8, u32, dim_w, dim_h, 1, 1, 0, 0, 0, 0);
        // Pre-hook scratch: all zeros here (the hook-struct writes land
        // later at this same address on the original side).
        let pre_hook = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, pre_hook.as_ptr() as u32, 0);
        let params = [0u32, 0, 1, 1, 1, 0, 1, 0];
        let holder = rd32(r(HOOK_HOLDER));
        let hook_addr = rd32(rd32(holder).wrapping_add(HOOK_SLOT));
        let hook: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(hook_addr as usize) };
        let hooked = hook(holder, r(HOOK_NAME), 3, dim_w, dim_h, 0x20, params.as_ptr() as u32);
        let n3: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0x10);
        let small = if n3 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(10, u32, n3)
        };

        // Shared teardown tail. `free_handle`: the ebx==0 entry skips the
        // handle free (it is already null); every other early entry frees
        // it. Live values come from the globals/locals as indicated.
        macro_rules! tail {
            ($free_handle:expr, $esi_v:expr, $ebp_v:expr, $edi_v:expr) => {{
                if $free_handle {
                    let h = rd32(r(HANDLE));
                    if h != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(42, u32, h);
                        let _: u32 = lf_checker_rt::callee_cdecl!(40, u32, h);
                        wr32(r(HANDLE), 0);
                    }
                }
                let esi_v: u32 = $esi_v;
                if esi_v != 0 {
                    let left =
                        rd32(esi_v.wrapping_add(0x20)).wrapping_sub(1);
                    wr32(esi_v.wrapping_add(0x20), left);
                    if left == 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(43, u32, esi_v);
                        let _: u32 = lf_checker_rt::callee_cdecl!(40, u32, esi_v);
                    }
                }
                let ebp_v: u32 = $ebp_v;
                if ebp_v != 0 {
                    let va = rd32(rd32(ebp_v));
                    let rel: extern "thiscall" fn(u32, u32) -> u32 =
                        unsafe { core::mem::transmute(va as usize) };
                    let _: u32 = rel(ebp_v, 1);
                }
                let edi_v: u32 = $edi_v;
                if edi_v != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(45, u32, edi_v);
                    let _: u32 = lf_checker_rt::callee_cdecl!(40, u32, edi_v);
                }
                let k = rd32(r(KEEP));
                if k != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(40, u32, k);
                    wr32(r(KEEP), 0);
                }
                let ans: u32 = lf_checker_rt::callee_thiscall!(46, u32, this);
                cookie();
                return ans;
            }};
        }

        let handle = rd32(r(HANDLE));
        if handle == 0 {
            tail!(false, target, hooked, small);
        }
        if target == 0 || hooked == 0 || small == 0 || rd32(r(KEEP)) == 0 {
            tail!(true, target, hooked, small);
        }

        // Descriptor area (base E-0x904 on the original side).
        let mut area = [0u32; 10];
        area[0] = rd32(r(TBL_B4));
        area[1] = rd32(r(TBL_B4).wrapping_add(4));
        area[2] = rd32(r(TBL_BC));
        area[3] = rd32(r(TBL_BC).wrapping_add(4));
        area[4] = rd32(r(TBL_C4));
        area[5] = rd32(r(TBL_C8)) & 0xFFFF | (rd32(r(TBL_CA)) & 0xFF) << 16;
        area[6] = 0;
        area[7] = 0;
        area[8] = 0;
        area[9] = 0;
        let area_bytes = area.as_ptr() as *const u8;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            12,
            u32,
            unsafe { area_bytes.wrapping_add(27) } as u32,
            0,
            0x29
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, conv_slot.as_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            14,
            u32,
            frank_slot.as_ptr() as u32,
            area.as_ptr() as u32
        );
        // Vestigial clamp: negative goes through js, zero through jl, and
        // positive through the fall-through xor -- every input yields 0.
        let _mode_raw = rd32(r(MODE));
        let mode = 0u32;
        let gate: u32 = lf_checker_rt::callee_cdecl!(15, u32, mode);
        if gate != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                16,
                u32,
                mode,
                area.as_ptr() as u32,
                0x40
            );
        }
        let zero2 = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            17,
            u32,
            zero2.as_ptr() as u32,
            rd32(r(DIM_W)),
            rd32(r(DIM_H))
        );
        // Both words are the stale dimension globals (DIM_W twice read).
        let sel = dim_w;
        let ans18: u32 = lf_checker_rt::callee_cdecl!(18, u32, dim_w, dim_h);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            19,
            u32,
            conv_slot.as_ptr() as u32,
            r(ARG_CONST)
        );
        // Format choice on the select word (0xF7F >= 0x780: big).
        let fmt = if sel < 0x780 {
            if sel < 0x500 {
                r(FMT_SMALL)
            } else {
                r(FMT_MID)
            }
        } else {
            r(FMT_BIG)
        };
        let ans20: u32 = lf_checker_rt::callee_cdecl!(20, u32, fmt, 0, 0);
        let ans21: u32 = lf_checker_rt::callee_thiscall!(21, u32, r(OBJ_FMT), ans20);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            22,
            u32,
            conv_slot.as_ptr() as u32,
            ans21
        );
        let zero2b = [0u32; 2];
        let r23: u32 = lf_checker_rt::callee_thiscall!(
            23,
            u32,
            rd32(r(HANDLE)),
            zero2b.as_ptr() as u32
        );
        // Failure path: free the formatted struct, run init, then the full
        // teardown with the notifier (the cleared tail flag selects it).
        macro_rules! fail_tail {
            () => {{
                let _: u32 = lf_checker_rt::callee_cdecl!(40, u32, conv_slot[0]);
                let _: u32 = lf_checker_rt::callee_cdecl!(41, u32,);
                tail!(true, target, hooked, small);
            }};
        }
        if r23 == 0 {
            fail_tail!();
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(24, u32,);
        let zero2c = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            25,
            u32,
            rd32(r(NAME_SRC)),
            zero2c.as_ptr() as u32,
            0x40
        );
        // id26's frame args: the answer slot aliased as [ans18, hookword0]
        // and zeros; value args from the conversion slot (low 5821, high 0).
        let slot94c = [ans18, dim_h];
        let zero4 = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            26,
            u32,
            slot94c.as_ptr() as u32,
            zero4.as_ptr() as u32,
            0x20,
            conv_slot[0],
            (conv_slot[1] as u16).wrapping_add(1) as u32
        );
        let zero8a = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            27,
            u32,
            zero8a.as_ptr() as u32,
            0x200,
            r(S2_NAME),
            r(AUX_OBJ)
        );
        let zero8e = [0u32; 2];
        let tgt1 = rd32(r(DTABLE_1));
        let dt1: extern "stdcall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(tgt1 as usize) };
        let _: u32 = dt1(zero8e.as_ptr() as u32, 0);
        let zero8b = [0u32; 1];
        let zero8c = [0u32; 1];
        let zero8d = [0u32; 1];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            28,
            u32,
            zero8b.as_ptr() as u32,
            0x200,
            r(S2B_NAME),
            r(AUX_OBJ),
            zero8c.as_ptr() as u32,
            zero8d.as_ptr() as u32
        );
        let zero8f = [0u32; 2];
        let r30: u32 = lf_checker_rt::callee_thiscall!(
            30,
            u32,
            rd32(r(HANDLE)),
            zero8f.as_ptr() as u32
        );
        if r30 == 0 {
            fail_tail!();
        }
        wr32(r(SLOT), target);
        wr32(r(SLOT_VAL), hooked);
        wr32(r(SLOT_ID), small);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            31,
            u32,
            this,
            area.as_ptr() as u32
        );
        let derived = rd32(r(NAME_SRC)).wrapping_add(0x2C);
        let zero8g = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            32,
            u32,
            zero8g.as_ptr() as u32,
            r(W1_B),
            r(KEY_OBJ),
            derived,
            r(W1_A)
        );
        let zero8h = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            33,
            u32,
            zero8h.as_ptr() as u32,
            r(W2_B),
            r(KEY_OBJ),
            derived,
            conv_slot[0],
            r(W2_A)
        );
        let zero8i = [0u32; 2];
        let zero8j = [0u32; 2];
        let tgt2 = rd32(r(DTABLE_2));
        let dt2: extern "stdcall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(tgt2 as usize) };
        let _: u32 = dt2(
            zero8i.as_ptr() as u32,
            zero8j.as_ptr() as u32,
            0
        );
        wr32(r(FLAG_A), 0);
        wr32(r(FLAG_B), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(35, u32, r(OBJ_REL), 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(36, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(37, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(38, u32,);
        // Table copy then table words then flags.
        let mut k = 0u32;
        while k < 5 {
            let qw = rd64p(r(TAB_SRC).wrapping_add(k * 8));
            wr32(
                r(TAB_DST).wrapping_add(k * 8),
                (qw & 0xFFFFFFFF) as u32
            );
            wr32(
                r(TAB_DST).wrapping_add(k * 8 + 4),
                (qw >> 32) as u32
            );
            k += 1;
        }
        wr32(r(TAB_DST).wrapping_add(0x28), rd32(r(TAB_SRC).wrapping_add(0x28)));
        wr32(r(FLAG_C), 1);
        wr32(r(TAB_SRC).wrapping_add(4), 4);
        wr32(r(TAB_SRC).wrapping_add(0xC), 0x63);
        wr32(r(TAB_SRC).wrapping_add(0x1C), 3);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let maybe = rd32(r(OBJ_MAYBE));
        wr32(r(FLAG_D), 1);
        if maybe != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(39, u32, maybe);
        }
        // Success path: the set tail flag skips the whole teardown block
        // (ownership already transferred to the slots); only the struct
        // free and init run, and the return value is init's answer.
        let _: u32 = lf_checker_rt::callee_cdecl!(40, u32, conv_slot[0]);
        let ok_ans: u32 = lf_checker_rt::callee_cdecl!(41, u32,);
        cookie();
        ok_ans
    }
});
