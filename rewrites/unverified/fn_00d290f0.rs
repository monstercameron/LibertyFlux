// original: 0x00D290F0 CPedTargetting::vf7

/// Decide whether a ped is an acceptable targeting candidate (pure predicate).
///
/// `this` is the targeting object, `ped` the candidate. The candidate passes
/// only if every gate below accepts it; any rejection returns 0, acceptance
/// returns 1 (in `al`; the upper bytes of `eax` are whatever the last callee
/// left, so only `al` is meaningful).
///
/// Gates in order:
/// 1. `ped.TYPE` (`+0x28`, masked with `TYPE_MASK`) must equal `TYPE_PED`.
/// 2. If byte `ped+0x210` is set, the word at `ped+0xA74` must be neither 1
///    nor 2.
/// 3. The pointer at `ped+0x224` must be non-null.
/// 4. The candidate must not be the owner's own ped (`this+0x24C`).
/// 5. If byte `ped+0x211` is set, callee 1 (asked of the owner's ped) must
///    answer nonzero.
/// 6. Callee 2 (asked of the owner's relation object at `owner+0x224`, with
///    the candidate) must answer zero.
/// 7. Callee 3 (asked of the owner's ped) yields an object or null; when
///    non-null, callee 4 (asked of that object `+8`, with the candidate)
///    must answer zero.
/// 8. Callee 5 (asked of the owner's relation object, with the candidate
///    and 1) decides: nonzero accepts at once, zero falls through to 9.
/// 9. Fallback: callee 6 (asked of `ped+0x224` advanced by `0x2E0`) yields a
///    ped or null. Null rejects, a non-ped type rejects, the owner's own ped
///    accepts, otherwise callee 2 is asked again with that ped and nonzero
///    accepts.
///
/// The function writes no memory; all six callees are intercepted by the
/// checker and scripted per trial. Original is thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_00D290F0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TYPE_OFF: u32 = 0x28;
        const TYPE_MASK: u32 = 0x3c0;
        const TYPE_PED: u32 = 0xc0;
        const FLAG_A: u32 = 0x210;
        const KIND_OFF: u32 = 0xa74;
        const REL_OFF: u32 = 0x224;
        const OWNER_OFF: u32 = 0x24c;
        const FLAG_B: u32 = 0x211;
        const FALLBACK_BIAS: u32 = 0x2e0;
        const GROUP_MEMBER_BIAS: u32 = 8;

        const CAL_A: u32 = 1;
        const CAL_B: u32 = 2;
        const CAL_C: u32 = 3;
        const CAL_D: u32 = 4;
        const CAL_E: u32 = 5;
        const CAL_F: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        if rd32(ped.wrapping_add(TYPE_OFF)) & TYPE_MASK != TYPE_PED {
            return 0;
        }
        if rd8(ped.wrapping_add(FLAG_A)) != 0 {
            let kind = rd32(ped.wrapping_add(KIND_OFF));
            if kind == 1 || kind == 2 {
                return 0;
            }
        }
        if rd32(ped.wrapping_add(REL_OFF)) == 0 {
            return 0;
        }
        let owner = rd32(this.wrapping_add(OWNER_OFF));
        if ped == owner {
            return 0;
        }
        if rd8(ped.wrapping_add(FLAG_B)) != 0 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(CAL_A, u32, owner);
            if ok & 0xff == 0 {
                return 0;
            }
        }
        let rel = rd32(owner.wrapping_add(REL_OFF));
        let veto: u32 = lf_checker_rt::callee_thiscall!(CAL_B, u32, rel, ped);
        if veto & 0xff != 0 {
            return 0;
        }
        let group: u32 = lf_checker_rt::callee_thiscall!(CAL_C, u32, owner);
        if group != 0 {
            let member: u32 =
                lf_checker_rt::callee_thiscall!(CAL_D, u32, group.wrapping_add(GROUP_MEMBER_BIAS), ped);
            if member & 0xff != 0 {
                return 0;
            }
        }
        let direct: u32 = lf_checker_rt::callee_thiscall!(CAL_E, u32, rel, ped, 1);
        if direct & 0xff == 0 {
            let alt: u32 = lf_checker_rt::callee_thiscall!(
                CAL_F,
                u32,
                rd32(ped.wrapping_add(REL_OFF)).wrapping_add(FALLBACK_BIAS)
            );
            if alt == 0 {
                return 0;
            }
            if rd32(alt.wrapping_add(TYPE_OFF)) & TYPE_MASK != TYPE_PED {
                return 0;
            }
            if alt == owner {
                return 1;
            }
            let second: u32 =
                lf_checker_rt::callee_thiscall!(CAL_B, u32, rd32(owner.wrapping_add(REL_OFF)), alt);
            if second & 0xff == 0 {
                return 0;
            }
        }
        1
    }
});
