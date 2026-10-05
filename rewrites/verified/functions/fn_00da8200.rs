// original: 0x00DA8200 flee_task_dispatch (proposed)

/// Dispatch a flee-task request by its request code to one of five makers.
///
/// `this` is the flee-task object, `a0` the request code, `a1` an auxiliary
/// object used only by the sweep arm. The dispatch is a subtract-and-compare
/// chain (signed above-compare against `0x38F` first): `0x385` builds a
/// guided task through a position callback, `0x2D4` a sweep task from
/// `a1+0xB30`, `0x2BE` a plain task from the `+0x38` sub-object, `0x38F` a
/// timed task and `0x3A0` a full task; `0x516` and every other code return 0.
///
/// Every arm first asks the blast-manager global for a dispatcher and
/// returns 0 on a null answer. The guided arm reads a base float from a
/// read-only constant, indexes a global pointer table by the sign-extended
/// word at `sub+0x2E`, adds a second read-only constant to the entry's
/// `+0x1C` float, and passes the sub-object, the entry position
/// (`sub+0x20` plus `0x30`) and a three-word zeroed stack buffer to the
/// callback. The sweep arm passes fifteen words (the `a1` field, small
/// constants, `-1.0`, three stack zeros). The timed and full arms pass
/// fields of `this` (`+0x14` selector, `+0x18` flag byte, floats at `+0x28`
/// and `+0x34`, dwords at `+0x1C/+0x20/+0x2C/+0x30`).
///
/// The one stack-buffer pointer argument is skipped in the call comparison
/// (frame addresses legitimately differ); everything else is compared.
///
/// Original: 0x00DA8200 (thiscall, two stack words, returns eax).
lf_checker_rt::export!(thiscall, rw_00DA8200(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const BLAST_MANAGER: u32 = 0x0167E2A0;
        const GUIDE_BASE: u32 = 0x00ED7EE0;
        const GUIDE_BIAS: u32 = 0x00FE87E4;
        const GUIDE_TABLE: u32 = 0x01295CD8;
        const TASK_SELECTOR: u32 = 0x14;
        const TASK_FLAG: u32 = 0x18;
        const TASK_D1: u32 = 0x1C;
        const TASK_D2: u32 = 0x20;
        const TASK_F1: u32 = 0x28;
        const TASK_D3: u32 = 0x2C;
        const TASK_D4: u32 = 0x30;
        const TASK_F2: u32 = 0x34;
        const TASK_SUB: u32 = 0x38;
        const SUB_POS: u32 = 0x20;
        const SUB_INDEX: u32 = 0x2E;
        const POS_OFF: u32 = 0x30;
        const ENTRY_VALUE: u32 = 0x1C;
        const AUX_FIELD: u32 = 0xB30;
        const NEG_ONE_BITS: u32 = 0xBF800000;
        const ONE_BITS: u32 = 0x3F800000;
        const FLEE_RANGE: u32 = 0x3E8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Request-code dispatch: signed above-compare, then subtract chain.
        let code: u32 = a0;
        let arm: u32;
        if (code as i32) > 0x38F {
            if code == 0x3A0 {
                arm = 5;
            } else {
                return 0;
            }
        } else if code == 0x38F {
            arm = 4;
        } else {
            let t: u32 = code.wrapping_sub(0x2BE);
            if t == 0 {
                arm = 2;
            } else {
                let t2: u32 = t.wrapping_sub(0x16);
                if t2 == 0 {
                    arm = 1;
                } else if t2.wrapping_sub(0xB1) != 0 {
                    return 0;
                } else {
                    arm = 0;
                }
            }
        }

        let manager: u32 = lf_checker_rt::global::<u32>(BLAST_MANAGER).read();
        if arm == 0 {
            // Guided task through the position callback.
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
            let base: u32 = lf_checker_rt::global::<u32>(GUIDE_BASE).read();
            let index: u32 = ((sub.wrapping_add(SUB_INDEX) as *const i16).read_unaligned() as i32) as u32;
            let table: u32 = lf_checker_rt::relocated(GUIDE_TABLE);
            let entry: u32 = rd32(table.wrapping_add(index.wrapping_mul(4)));
            let biased: f32 = add(
                rdf(entry.wrapping_add(ENTRY_VALUE)),
                lf_checker_rt::global::<f32>(GUIDE_BIAS).read(),
            );
            let pos: u32 = rd32(sub.wrapping_add(SUB_POS)).wrapping_add(POS_OFF);
            let outbuf: [u32; 3] = [0, 0, 0];
            lf_checker_rt::callee_thiscall!(
                2, u32, dispatch, 3, pos, sub, (&outbuf as *const u32) as u32,
                biased.to_bits(), base, 0
            )
        } else if arm == 1 {
            // Sweep task from the auxiliary object.
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            let field: u32 = rd32(a1.wrapping_add(AUX_FIELD));
            lf_checker_rt::callee_thiscall!(
                3, u32, dispatch, field, 0, 1, 2, 0x28, 0, 0, 0, 0, 4, 0x14,
                NEG_ONE_BITS, 0x1E, 0x14, 1
            )
        } else if arm == 2 {
            // Plain task from the sub-object.
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(4, u32, dispatch, rd32(this.wrapping_add(TASK_SUB)))
        } else if arm == 4 {
            // Timed task.
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            let flag: u32 = (this.wrapping_add(TASK_FLAG) as *const u8).read() as u32;
            lf_checker_rt::callee_thiscall!(
                5, u32, dispatch, rd32(this.wrapping_add(TASK_SELECTOR)), flag,
                rd32(this.wrapping_add(TASK_F1)), rd32(this.wrapping_add(TASK_D3)),
                FLEE_RANGE, ONE_BITS, 0
            )
        } else {
            // Full task.
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            let flag: u32 = (this.wrapping_add(TASK_FLAG) as *const u8).read() as u32;
            lf_checker_rt::callee_thiscall!(
                6, u32, dispatch, rd32(this.wrapping_add(TASK_SELECTOR)), flag,
                rd32(this.wrapping_add(TASK_F1)), rd32(this.wrapping_add(TASK_D3)),
                rd32(this.wrapping_add(TASK_D1)), rd32(this.wrapping_add(TASK_D2)),
                rd32(this.wrapping_add(TASK_D4)), rd32(this.wrapping_add(TASK_F2))
            )
        }
    }
});
