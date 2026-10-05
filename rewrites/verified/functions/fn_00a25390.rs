// original: 0x00a25390 task_state_value_blend (proposed)

/// Blend the word at `this + 0x1f4` towards a target picked from the state of
/// the task object `ped`, then store the result back and return it.
///
/// `ped` is a task owner (null skips every test); `flags` contributes only its
/// low byte. The target and rate table is:
///
/// * `ped` null, or the duck/busy tests below fail: target `+0.0`, rate 0.25.
/// * busy with the flag object present and flag bit set: target `-0.0`,
///   rate 0.1.
/// * busy without that case: target `-0.4` when `[ped + 0xb80] == 1`,
///   else `-0.15`, rate 0.25.
///
/// Busy means the ducking callee on `ped` or the busy bit says so, while the
/// block and invert bits stay clear. The busy bit is set when the second
/// task lookup finds an object whose flag word's `0x18` bits are not all set;
/// the invert bit is set from the `0x60` bits of that word (immediately when
/// both are set, else through one more callee and a re-read of the word).
/// The block bit is the first lookup's callee answer on `ped`.
///
/// The blend is `cur + (target - cur) * rate` in that operation order. When
/// the low byte of `flags` is non-zero the rate becomes 1.0, so the result is
/// the target. The result is stored to `this + 0x1f4` and returned in ST0.
///
/// Original: 0x00a25390 (thiscall, two stack words; upper bytes of the second
/// are unread).
lf_checker_rt::export!(thiscall, rw_00a25390(this: u32, ped: u32, flags: u32) -> f32 {
    unsafe {
        const RATE_DEFAULT: f32 = f32::from_bits(0x3e80_0000); // 0.25
        const RATE_ALT: f32 = f32::from_bits(0x3dcc_cccd); // 0.1
        const RATE_FULL: f32 = f32::from_bits(0x3f80_0000); // 1.0
        const TARGET_A: f32 = f32::from_bits(0x8000_0000); // -0.0
        const TARGET_B: f32 = f32::from_bits(0xbecc_cccd); // -0.4
        const TARGET_C: f32 = f32::from_bits(0xbe19_999a); // -0.15
        const PED_MGR: u32 = 0x224;
        const PED_FLAGS: u32 = 0xd68;
        const PED_MODE: u32 = 0xb80;
        const MGR_BIAS: u32 = 0x44;
        const LOOKUP_A: u32 = 0x41e;
        const LOOKUP_B: u32 = 0x414;
        const VALUE_OFF: u32 = 0x1f4;
        const BUSY_MASK: u32 = 0x18;
        const INV_MASK: u32 = 0x60;
        const INV_SET: u32 = 0x20;
        const FIND: u32 = 1;
        const CHECK1: u32 = 2;
        const CHECK2: u32 = 3;
        const BLOCK_OF: u32 = 4;
        const INV_OF: u32 = 5;
        const DUCK: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let mut rate = RATE_DEFAULT;
        let mut target = 0.0f32;
        if ped != 0 {
            let mgr = rd32(ped.wrapping_add(PED_MGR)).wrapping_add(MGR_BIAS);
            let first = lf_checker_rt::callee_thiscall!(FIND, u32, mgr, LOOKUP_A);
            let mgr2 = rd32(ped.wrapping_add(PED_MGR)).wrapping_add(MGR_BIAS);
            let second = lf_checker_rt::callee_thiscall!(FIND, u32, mgr2, LOOKUP_B);
            let flag: u8 = if lf_checker_rt::callee_cdecl!(CHECK1, u32, 1) as u8 != 0 {
                1
            } else if lf_checker_rt::callee_cdecl!(CHECK2, u32,) as u8 != 0 {
                1
            } else {
                0
            };
            let blocked: u8 = if first != 0
                && lf_checker_rt::callee_thiscall!(BLOCK_OF, u32, first, ped) as u8 != 0
            {
                1
            } else {
                0
            };
            let mut busy: u8 = 0;
            if second != 0 {
                let words = rd32(ped.wrapping_add(PED_FLAGS));
                if words != 0 && rd32(words) & BUSY_MASK != BUSY_MASK {
                    busy = 1;
                }
            }
            let mut invert: u8 = 0;
            if second != 0 {
                let words = rd32(ped.wrapping_add(PED_FLAGS));
                if words != 0 {
                    if rd32(words) & INV_MASK == INV_MASK {
                        invert = 1;
                    } else {
                        let ans = lf_checker_rt::callee_thiscall!(INV_OF, u32, second, ped) as u8;
                        let fresh = rd32(ped.wrapping_add(PED_FLAGS));
                        if ans == 0 {
                            if (fresh as *const u8).read() & INV_MASK as u8 == 0 {
                                invert = 1;
                            }
                        } else if rd32(fresh) & INV_MASK == INV_SET {
                            invert = 1;
                        }
                    }
                }
            }
            let ducking = lf_checker_rt::callee_thiscall!(DUCK, u32, ped) as u8;
            if (ducking != 0 || busy != 0) && blocked == 0 && invert == 0 {
                let words = rd32(ped.wrapping_add(PED_FLAGS));
                if words != 0 && flag != invert {
                    rate = RATE_ALT;
                    target = TARGET_A;
                } else if rd32(ped.wrapping_add(PED_MODE)) == 1 {
                    target = TARGET_B;
                } else {
                    target = TARGET_C;
                }
            }
        }
        if flags & 0xff != 0 {
            rate = RATE_FULL;
        }
        let cur = rdf(this.wrapping_add(VALUE_OFF));
        let out = add(mul(sub(target, cur), rate), cur);
        wrf(this.wrapping_add(VALUE_OFF), out);
        out
    }
});
