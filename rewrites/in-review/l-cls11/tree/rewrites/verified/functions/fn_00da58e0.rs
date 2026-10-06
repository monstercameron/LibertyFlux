// original: 0x00da58e0 CTaskComplexShockingEventFlee::vf19

/// Flee from a shocking event: seed a reaction, then build a flee task, a
/// fallback task, or nothing, depending on the ped's state and a cooldown.
///
/// `this` is the task object (mode flag at `+0x46`, secondary state at
/// `+0x48`, behaviour id at `+0x20`, position at `+0x30`, flag byte at
/// `+0x70`), `ped` the reacting ped. The gate is the family's usual one:
/// return 0 unless the flag/state pair selects the main path, in which case
/// the squared position length must also exceed `LEN2_LIMIT`.
///
/// The main path reads the ped's type (signed word at `ped+0x2e`) through
/// the global pointer table `TASK_TABLE` and probes the entry's status byte
/// at `+0xee`. For ids in `0x15..0x1b` with a fresh entry, callee 1 answers
/// a random word that, scaled by `SCALE`, must stay below `PROB_SEED` for
/// callee 2 (ten constant words, one pushed from the global `SEED_ARG`) to
/// run; higher ids seed unconditionally.
///
/// Stage two keys off the secondary state as an object: unless its mode bits
/// at `+0x28` equal `MODE_FLEE` and callee 3 answers zero, control falls to
/// the fallback. For ids `0x1b..=0x1d` with the ped's flags right and the
/// global `CLOCK` below `CLOCK_BASE`, a second random word below
/// `PROB_SPAWN` refreshes the clock (`CLOCK_BASE + CLOCK_STEP`), asks callee
/// 5 for a builder and returns callee 6's spawn. Otherwise callee 8 builds
/// task A from a zeroed scratch word: bit 3 of its `+0x60` is set, its bit 4
/// takes bit 0 of the flag byte, and bit 18 is cleared for ids below `0x1b`.
///
/// The fallback checks the ped's flag at `+0x26c`: when set and `ped+0xb30`
/// is non-null, callee 12 builds from it (scratch pointer second) and its
/// answer is returned. Otherwise callee 10 builds task B from a scratch
/// pointer, a constant, and the read-only word `RATE_WORD`: bit 3 of its
/// `+0x6c` is set and bit 4 takes the flag bit the same way. Null builders
/// fault on the flag store exactly like the original, as does a null table
/// entry on the probe.
///
/// Original: 0x00da58e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00da58e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x46;
        const STATE: u32 = 0x48;
        const KIND: u32 = 0x20;
        const POS: u32 = 0x30;
        const FLAG_BYTE: u32 = 0x70;
        const LEN2_LIMIT: f32 = f32::from_bits(0x3d4c_cccd); // 0.05
        const SCALE: f32 = f32::from_bits(0x3800_0100);
        const PROB_SEED: f32 = f32::from_bits(0x3ea8_f5c3); // 0.33
        const PROB_SPAWN: f32 = f32::from_bits(0x3f00_0000); // 0.5
        const TASK_TABLE: u32 = 0x1295_cd8;
        const TABLE_PROBE: u32 = 0xee;
        const STALE_LIMIT: u8 = 3;
        const SEED_ARG: u32 = 0x0128_4530;
        const MODE_SLOT: u32 = 0x28;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_FLEE: u32 = 0xc0;
        const CLOCK: u32 = 0x017a_6500;
        const CLOCK_BASE: u32 = 0x0117_35b4;
        const CLOCK_STEP: u32 = 0x4e20;
        const MANAGER: u32 = 0x0167_e2a0;
        const RATE_WORD: u32 = 0x00ee_f924;
        const SET_BIT: u32 = 8;
        const FLAG_BIT: u32 = 0x10;
        const CLEAR_MASK: u32 = 0xfffb_ffff;
        const RAND1_CALLEE: u32 = 1;
        const SEED_CALLEE: u32 = 2;
        const CHECK_CALLEE: u32 = 3;
        const RAND2_CALLEE: u32 = 4;
        const MGR1_CALLEE: u32 = 5;
        const SPAWN_CALLEE: u32 = 6;
        const MGR2_CALLEE: u32 = 7;
        const BUILD_A_CALLEE: u32 = 8;
        const MGR3_CALLEE: u32 = 9;
        const BUILD_B_CALLEE: u32 = 10;
        const MGR4_CALLEE: u32 = 11;
        const BUILD_C_CALLEE: u32 = 12;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Set bit 3 of the word at `w`, then set bit 4 to `bit`.
        /// The original reads the word first, so a null `w` faults on the
        /// read; the volatile access keeps that order.
        #[inline(always)]
        unsafe fn set_flag_bits(w: u32, bit: u32) {
            unsafe {
                let p = w as *mut u32;
                let v = p.read_volatile() | SET_BIT;
                p.write_volatile(v);
                p.write_volatile((v & !FLAG_BIT) | ((bit & 1) << 4));
            }
        }

        let flag = rd8(this + FLAG);
        let state = rd32(this + STATE);
        if flag != 0 {
            if state == 0 {
                return 0;
            }
        } else {
            if state != 0 {
                return 0;
            }
            let x = rdf(this + POS);
            let y = rdf(this + POS + 4);
            let z = rdf(this + POS + 8);
            let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
            if !(len2 > LEN2_LIMIT) {
                return 0;
            }
        }

        let f0 = rdf(this + POS);
        let f1 = rdf(this + POS + 4);
        let f2 = rdf(this + POS + 8);
        let type_id = rd16(ped + 0x2e) as i16 as i32;
        let slot = lf_checker_rt::relocated(TASK_TABLE)
            .wrapping_add(type_id.wrapping_mul(4) as u32);
        let entry = (slot as *const u32).read_unaligned();
        let fresh = rd8(entry.wrapping_add(TABLE_PROBE)) < STALE_LIMIT;

        let kind = rd32(this + KIND) as i32;
        let mut do_seed = false;
        if fresh {
            if kind >= 0x1b {
                do_seed = true;
            } else if kind >= 0x15 {
                let r: u32 = lf_checker_rt::callee_cdecl!(RAND1_CALLEE, u32,);
                if PROB_SEED > mul((r as i32) as f32, SCALE) {
                    do_seed = true;
                }
            }
        }
        if do_seed {
            let g = lf_checker_rt::global::<u32>(SEED_ARG).read();
            let _: u32 = lf_checker_rt::callee_thiscall!(
                SEED_CALLEE,
                u32,
                ped.wrapping_add(0x570),
                lf_checker_rt::relocated(0x00ee_f5fc),
                0,
                0,
                g,
                0xffff_ffff,
                0,
                0,
                0x3f80_0000,
                0,
                0
            );
        }

        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        if state != 0 && rd32(state.wrapping_add(MODE_SLOT)) & MODE_MASK == MODE_FLEE {
            let answer: u32 =
                lf_checker_rt::callee_thiscall!(CHECK_CALLEE, u32, rd32(ped + 0x224), state);
            if (answer as u8) == 0 {
                let k2 = rd32(this + KIND);
                if k2 == 0x1b || k2 == 0x1c || k2 == 0x1d {
                    if rd8(ped + 0x26c) & 4 == 0 {
                        let clock = lf_checker_rt::global::<u32>(CLOCK).read();
                        let base = lf_checker_rt::global::<u32>(CLOCK_BASE).read();
                        if clock < base && rd8(ped + 0xf4) & 4 != 0 {
                            let r2: u32 = lf_checker_rt::callee_cdecl!(RAND2_CALLEE, u32,);
                            if PROB_SPAWN > mul((r2 as i32) as f32, SCALE) {
                                lf_checker_rt::global::<u32>(CLOCK)
                                    .write(base.wrapping_add(CLOCK_STEP));
                                let o1: u32 =
                                    lf_checker_rt::callee_thiscall!(MGR1_CALLEE, u32, mgr);
                                if o1 == 0 {
                                    return 0;
                                }
                                return lf_checker_rt::callee_thiscall!(
                                    SPAWN_CALLEE,
                                    u32,
                                    o1,
                                    lf_checker_rt::relocated(0x00ee_f608),
                                    1,
                                    state
                                );
                            }
                        }
                    }
                }
                // Task A. The original pushes the saved secondary state here.
                let o2: u32 = lf_checker_rt::callee_thiscall!(MGR2_CALLEE, u32, mgr);
                let edx: u32 = if o2 == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(BUILD_A_CALLEE, u32, o2, state, 0)
                };
                let flag_bit = rd8(this + FLAG_BYTE) as u32;
                set_flag_bits(edx.wrapping_add(0x60), flag_bit);
                if (rd32(this + KIND) as i32) < 0x1b {
                    let p = edx.wrapping_add(0x60) as *mut u32;
                    p.write_volatile(p.read_volatile() & CLEAR_MASK);
                }
                return edx;
            }
        }
        // Fallback: the scratch pointer the builders receive points at the
        // three position floats the function spilled on entry.
        if rd8(ped + 0x26c) & 4 != 0 {
            let target = rd32(ped + 0xb30);
            if target == 0 {
                return 0;
            }
            let o4: u32 = lf_checker_rt::callee_thiscall!(MGR4_CALLEE, u32, mgr);
            if o4 == 0 {
                return 0;
            }
            let mut buf = [0u32; 4];
            buf[0] = f0.to_bits();
            buf[1] = f1.to_bits();
            buf[2] = f2.to_bits();
            return lf_checker_rt::callee_thiscall!(
                BUILD_C_CALLEE,
                u32,
                o4,
                target,
                buf.as_mut_ptr() as u32
            );
        }
        let o3: u32 = lf_checker_rt::callee_thiscall!(MGR3_CALLEE, u32, mgr);
        let edx: u32 = if o3 == 0 {
            0
        } else {
            let mut buf = [0u32; 4];
            buf[0] = f0.to_bits();
            buf[1] = f1.to_bits();
            buf[2] = f2.to_bits();
            let rate = lf_checker_rt::global::<u32>(RATE_WORD).read();
            lf_checker_rt::callee_thiscall!(
                BUILD_B_CALLEE,
                u32,
                o3,
                buf.as_mut_ptr() as u32,
                0,
                0x4479_c000,
                rate,
                0
            )
        };
        set_flag_bits(edx.wrapping_add(0x6c), rd8(this + FLAG_BYTE) as u32);
        edx
    }
});
