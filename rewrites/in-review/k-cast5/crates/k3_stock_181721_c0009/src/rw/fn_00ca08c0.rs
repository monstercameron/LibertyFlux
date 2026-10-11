// original: 0x00ca08c0 CTaskSimpleIK::vf1 (merged symbol; vtable placeholder)

/// Advance one IK task step for a ped: run the guard checks, then move the
/// blend weight toward its target.
///
/// `task` is the task object, `ped` the ped it runs on. The first half is a
/// row of independent guards, each firing the strike callee (`CALLEE_STRIKE`)
/// with the word at `ARG_SLOT` (+0x28) when its condition holds: the task's
/// own virtual slot `VT_PRE` unless flag bit 0 is set; the flag-bit-1 path
/// (which also clears the bit and resets `RESET_SLOT` to -1) when the word
/// at `STATE_SLOT` (+0x14) is zero; the ped bytes/words at `PED_*`; the
/// stance callee (`CALLEE_STANCE`) on the ped's word at +0x78; and, past a
/// gauntlet (two ped bytes, a global table index that must not be -1, a live
/// table entry whose word at +0x4C8 is zero), a ped-object check plus up to
/// two task-list queries (`CALLEE_FIND1`/`CALLEE_FIND2`) of which the second
/// runs only when the first answers zero.
///
/// The second half moves the blend weight at `W_SLOT` (+0x50): a chopped
/// time step (`TIME_GLOB` times `SCALE_GLOB`, truncated toward zero exactly
/// like the original's x87 truncate-store; out-of-range or NaN stores the
/// integer indefinite, whose low word the original keeps, i.e. zero) is
/// divided, as signed integers turned
/// float, by the word at `ARG_SLOT` or `DIV_ALT` (+0x24) per flag bit 3;
/// the quotient is added when the word at `W_TARGET` (+0x54) equals 1.0 and
/// subtracted otherwise (NaN included); the result is clamped to [0, 1]
/// (NaN passes through) and stored. When the word at `COUNT_SLOT` (+0x20)
/// is not -1, an exactly-zero weight invokes virtual slot `VT_ZERO` and
/// returns 1, an exactly-one weight subtracts the chopped step from the
/// count (firing the strike callee when the new count is negative) and
/// falls through; every other path runs the tail callee (`CALLEE_TAIL`)
/// and returns 0. The count-compare on the zero path is dead (the count is
/// already known non--1 there) and is not repeated.
///
/// Original: 0x00ca08c0 (thiscall, one stack word, callee pops 4; 8-bit
/// boolean in al, upper bits are callee leftovers).
lf_checker_rt::export!(thiscall, rw_00ca08c0(task: u32, ped: u32) -> u32 {
    unsafe {
        const STATE_SLOT: u32 = 0x14;
        const COUNT_SLOT: u32 = 0x20;
        const DIV_ALT: u32 = 0x24;
        const ARG_SLOT: u32 = 0x28;
        const RESET_SLOT: u32 = 0x2C;
        const W_SLOT: u32 = 0x50;
        const W_TARGET: u32 = 0x54;
        const MODE_SLOT: u32 = 0x58;
        const FLAG_SLOT: u32 = 0x5C;
        const VT_PRE: u32 = 0x08;
        const VT_ZERO: u32 = 0x0C;
        const PED_STANCE: u32 = 0x78;
        const PED_HIT: u32 = 0x210;
        const PED_GATE0: u32 = 0x218;
        const PED_GATE1: u32 = 0x219;
        const PED_TASKS: u32 = 0x224;
        const PED_COVER: u32 = 0x26C;
        const PED_FLAGS: u32 = 0x29C;
        const PED_OBJ: u32 = 0xB30;
        const FIND_THIS_OFF: u32 = 0x2E0;
        const FIND_ID_A: u32 = 0x410;
        const FIND_ID_B: u32 = 0x2E4;
        const OBJ_READY: u32 = 0x1304;
        const TAB_LIVE: u32 = 0x4C8;
        const STANCE_TAG: u32 = 0x1400_0000;
        const TWO63_BITS: u32 = 0x5F00_0000;
        /// Low word of the x87 integer indefinite (INT64_MIN): zero.
        const INDEF_LO: u32 = 0x0000_0000;
        const TIME_GLOB: u32 = 0x11735BC;
        const SCALE_GLOB: u32 = 0xFE8C58;
        const ONE_GLOB: u32 = 0xFE88E8;
        const IDX_GLOB: u32 = 0x1036F14;
        const TAB_GLOB: u32 = 0x11A8808;
        const CALLEE_STRIKE: u32 = 1;
        const CALLEE_STANCE: u32 = 2;
        const CALLEE_FIND1: u32 = 3;
        const CALLEE_FIND2: u32 = 4;
        const CALLEE_VPRE: u32 = 5;
        const CALLEE_VZERO: u32 = 6;
        const CALLEE_TAIL: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn glob_f(va: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn strike(task: u32, arg: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(CALLEE_STRIKE, u32, task, arg);
            }
        }
        /// Call a virtual slot with the ped as its stack argument.
        #[inline(always)]
        unsafe fn vcall_ped(obj: u32, slot: u32, ped: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj, ped)
            }
        }
        /// Original-order float ops: both operands pinned so the compiler
        /// keeps the instruction and its operand order (it otherwise merges
        /// the add/sub select into a negated add with swapped operands,
        /// which propagates the wrong NaN).
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncate a float toward zero to a 64-bit integer and keep the
        /// low 32 bits, exactly like the original's x87 chop-store:
        /// NaN, infinities and magnitudes at or past 2^63 (except exactly
        /// -2^63, which is a valid minimum) store the integer indefinite,
        /// whose low word is `INDEF_LO`.
        #[inline(always)]
        fn chop_lo32(t: f32) -> u32 {
            let two63 = f32::from_bits(TWO63_BITS);
            if t.is_nan() || t >= two63 || t < -two63 {
                INDEF_LO
            } else {
                t.trunc() as i64 as u32
            }
        }

        let arg28 = rd32(task.wrapping_add(ARG_SLOT));
        if rd8(task.wrapping_add(FLAG_SLOT)) & 1 == 0 {
            vcall_ped(task, VT_PRE, ped);
        }
        let flags = rd32(task.wrapping_add(FLAG_SLOT));
        if flags & 2 != 0 && rd32(task.wrapping_add(STATE_SLOT)) == 0 {
            wr32(task.wrapping_add(FLAG_SLOT), flags & !2);
            wr32(task.wrapping_add(RESET_SLOT), 0xFFFF_FFFF);
            strike(task, arg28);
        }
        if rd8(ped.wrapping_add(PED_HIT)) != 0 {
            strike(task, arg28);
        }
        let pflags = rd32(ped.wrapping_add(PED_FLAGS));
        if pflags & 0x300000 != 0 {
            strike(task, arg28);
        }
        if pflags & 0xC00000 != 0 && rd8(task.wrapping_add(MODE_SLOT)) & 0x20 == 0 {
            strike(task, arg28);
        }
        let stance = lf_checker_rt::callee_thiscall!(
            CALLEE_STANCE,
            u32,
            rd32(ped.wrapping_add(PED_STANCE)),
            STANCE_TAG,
            1
        );
        if stance != 0 {
            strike(task, arg28);
        }
        if rd8(ped.wrapping_add(PED_GATE0)) == 0 && rd8(ped.wrapping_add(PED_GATE1)) != 0 {
            let idx = lf_checker_rt::global::<u32>(IDX_GLOB).read_unaligned() as i32;
            if idx != -1 {
                let entry =
                    lf_checker_rt::global::<u32>(
                        TAB_GLOB.wrapping_add((idx as u32).wrapping_mul(4)),
                    )
                    .read_unaligned();
                if entry != 0 && rd32(entry.wrapping_add(TAB_LIVE)) == 0 {
                    if rd8(ped.wrapping_add(PED_COVER)) & 4 != 0 {
                        let obj = rd32(ped.wrapping_add(PED_OBJ));
                        if obj != 0
                            && rd32(obj.wrapping_add(OBJ_READY)) == 1
                            && rd32(task.wrapping_add(MODE_SLOT)) & 0x400 == 0
                        {
                            strike(task, arg28);
                        }
                    }
                    let list = rd32(ped.wrapping_add(PED_TASKS)).wrapping_add(FIND_THIS_OFF);
                    let found = lf_checker_rt::callee_thiscall!(
                        CALLEE_FIND1, u32, list, FIND_ID_A, 0
                    );
                    let found = if found & 0xFF != 0 {
                        1
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            CALLEE_FIND2, u32, list, FIND_ID_B, 0
                        ) & 0xFF
                    };
                    if found != 0 {
                        strike(task, arg28);
                    }
                }
            }
        }

        let one = glob_f(ONE_GLOB);
        let step = chop_lo32(fmul(glob_f(TIME_GLOB), glob_f(SCALE_GLOB)));
        let chopped = (step as i32) as f32;
        let divword = if rd8(task.wrapping_add(FLAG_SLOT)) & 8 != 0 {
            arg28
        } else {
            rd32(task.wrapping_add(DIV_ALT))
        };
        let ratio = fdiv(chopped, (divword as i32) as f32);
        let acc = f32::from_bits(rd32(task.wrapping_add(W_SLOT)));
        let acc = if f32::from_bits(rd32(task.wrapping_add(W_TARGET))) == one {
            fadd(acc, ratio)
        } else {
            fsub(acc, ratio)
        };
        let clamped = if acc < 0.0 {
            0.0
        } else if acc > one {
            one
        } else {
            acc
        };
        wr32(task.wrapping_add(W_SLOT), clamped.to_bits());
        let count = rd32(task.wrapping_add(COUNT_SLOT));
        if count == 0xFFFF_FFFF {
            lf_checker_rt::callee_thiscall!(CALLEE_TAIL, u32, task);
            return 0;
        }
        if clamped == 0.0 {
            vcall_ped(task, VT_ZERO, ped);
            return 1;
        }
        if clamped == one {
            let new = count.wrapping_sub(step);
            wr32(task.wrapping_add(COUNT_SLOT), new);
            if ((new as i32) as f32) < 0.0 {
                strike(task, arg28);
            }
        }
        lf_checker_rt::callee_thiscall!(CALLEE_TAIL, u32, task);
        0
    }
});
