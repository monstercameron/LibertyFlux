// original: 0x00b6fed0 JACKING_GENERIC

/// Build a generic vehicle-jacking task for the given actors and seat.
///
/// `a0` is the animation object, `a1` the vehicle, `a2` the ped, `a3` the
/// wanted seat id (-1 skips the seat check). Layout read: `a0+0x218`/`+0x219`
/// busy bytes and `a0+0xb30` the current vehicle (must equal `a1` to reuse
/// the door path); `a1+0x2e` a 16-bit model word indexing the seat table and
/// `a1+0x1304` the vehicle state; `a2+0x211` the in-vehicle flag and its
/// vtable slot `+0x128` the jacking hook. Four globals hold one-time init
/// state: a flag word (bits for the three lazily resolved handles) and the
/// three handles; a fifth global region holds the seat table (entries point
/// at seat records with the accepted seat id at `+0xc4`).
///
/// Behaviour: resolve any missing handles first (recording which were made),
/// then pick the ped's handle by the hook result and the door probe. The
/// bulk is a decision tree over the hook, the busy bytes, the seat check,
/// the vehicle state (1 = seated path returning 1 when already inside, 0 =
/// probe more, anything else = build directly) and two ped predicates,
/// ending in exactly one of twelve builder calls (thiscall on `a0`/`a1`/`a2`
/// `+0x570`, ten words: a name pointer, shape flags, the ped and handle or
/// zeros, 1.0, zeros) whose value is returned. Null `a1`/`a2` return
/// immediately; the value there is whatever the caller left, so the proof
/// never passes null (see narrowed).
///
/// Original: 0x00b6fed0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00b6fed0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const HOOK_SLOT: u32 = 0x128;
        const MODEL_WORD: u32 = 0x2e;
        const VEH_STATE: u32 = 0x1304;
        const IN_VEH_FLAG: u32 = 0x211;
        const BUSY0: u32 = 0x218;
        const BUSY1: u32 = 0x219;
        const CUR_VEH: u32 = 0xb30;
        const BUILDER_OFF: u32 = 0x570;
        const PROBE_OFF: u32 = 0x210;
        const DOOR_OFF: u32 = 0x2b0;
        const SEAT_ID_OFF: u32 = 0xc4;
        const G_HANDLE0: u32 = 0x1670ce8;
        const G_FLAGS: u32 = 0x1670cec;
        const G_HANDLE1: u32 = 0x1670cf0;
        const G_HANDLE2: u32 = 0x1670cf4;
        const SEAT_TABLE: u32 = 0x1295cd8;
        const CAL_RESOLVE: u32 = 0;
        const CAL_PROBE1: u32 = 2;
        const CAL_PROBE2: u32 = 3;
        const CAL_PROBE3: u32 = 4;
        const CAL_CHECK1: u32 = 5;
        const CAL_MODEL: u32 = 6;
        const CAL_BUILD: u32 = 7;
        const CAL_CHECK2: u32 = 8;
        const CAL_DOOR: u32 = 9;
        const CAL_PRED_A: u32 = 10;
        const CAL_PRED_B: u32 = 11;
        const CAL_CHECK3: u32 = 12;
        const NAME_RES0: u32 = 0xeb18c8;
        const NAME_RES1: u32 = 0xeb18d8;
        const NAME_RES2: u32 = 0xeb18f4;
        const NAME_TMP: u32 = 0xeb1940;
        const ONE_F: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn glob(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wrglob(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(v) }
        }
        /// The jacking hook: vtable slot 0x128 on the ped.
        #[inline(always)]
        unsafe fn hook(a2: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(a2) + HOOK_SLOT) as usize);
                f(a2)
            }
        }
        /// A builder call carrying ped and handle.
        #[inline(always)]
        unsafe fn build_full(obj: u32, name: u32, ped: u32, handle: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    CAL_BUILD, u32, obj.wrapping_add(BUILDER_OFF),
                    lf_checker_rt::relocated(name),
                    1, 0, 0, 0xffff_ffff, ped, handle, ONE_F.to_bits(), 0, 0
                )
            }
        }
        /// A builder call carrying zeros.
        #[inline(always)]
        unsafe fn build_zero(obj: u32, name: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    CAL_BUILD, u32, obj.wrapping_add(BUILDER_OFF),
                    lf_checker_rt::relocated(name),
                    0, 0, 0, 0xffff_ffff, 0, 0, ONE_F.to_bits(), 0, 0
                )
            }
        }

        if a2 == 0 || a1 == 0 {
            // Unreached in the proof (see narrowed); return a fixed value.
            return 0;
        }
        // Lazily resolve the three handles.
        let mut flags = glob(G_FLAGS);
        if flags & 1 == 0 {
            flags |= 1;
            wrglob(G_FLAGS, flags);
            let r = lf_checker_rt::callee_cdecl!(
                CAL_RESOLVE, u32, lf_checker_rt::relocated(NAME_RES0), 0
            );
            wrglob(G_HANDLE0, r);
            flags = glob(G_FLAGS);
        }
        if flags & 2 == 0 {
            flags |= 2;
            wrglob(G_FLAGS, flags);
            let r = lf_checker_rt::callee_cdecl!(
                CAL_RESOLVE, u32, lf_checker_rt::relocated(NAME_RES1), 0
            );
            wrglob(G_HANDLE1, r);
            flags = glob(G_FLAGS);
        }
        if flags & 4 == 0 {
            flags |= 4;
            wrglob(G_FLAGS, flags);
            let r = lf_checker_rt::callee_cdecl!(
                CAL_RESOLVE, u32, lf_checker_rt::relocated(NAME_RES2), 0
            );
            wrglob(G_HANDLE2, r);
        }
        let mut handle = glob(G_HANDLE0);
        if hook(a2) as u8 != 0 {
            if rd32(a1.wrapping_add(VEH_STATE)) == 0 {
                let p = lf_checker_rt::callee_thiscall!(
                    CAL_PROBE1, u32, a1.wrapping_add(PROBE_OFF)
                );
                handle = glob(G_HANDLE1);
                if p as u8 == 0 {
                    handle = glob(G_HANDLE2);
                }
            } else {
                handle = glob(G_HANDLE2);
            }
        }
        if lf_checker_rt::callee_thiscall!(CAL_CHECK1, u32, a1, a2) as u8 == 0 {
            let model = rd16s(a1.wrapping_add(MODEL_WORD));
            if lf_checker_rt::callee_cdecl!(CAL_MODEL, u32, 0x18, model) as u8 != 0 {
                return build_full(a0, 0xeb1904, a2, handle);
            }
        }
        // From here a0 is the working object.
        if rd8(a0.wrapping_add(BUSY0)) == 0
            && rd8(a0.wrapping_add(BUSY1)) == 0
            && rd32(a0.wrapping_add(CUR_VEH)) == a1
        {
            if rd32(a1.wrapping_add(VEH_STATE)) == 0 {
                let p = lf_checker_rt::callee_thiscall!(
                    CAL_PROBE2, u32, a1.wrapping_add(PROBE_OFF)
                );
                if p as u8 != 0 {
                    return build_full(a0, 0xeb1914, a2, handle);
                }
            }
            return build_full(a0, 0xeb1928, a2, handle);
        }
        if a3 != 0xffff_ffff {
            let idx = rd16s(a1.wrapping_add(MODEL_WORD));
            let entry = rd32(
                lf_checker_rt::relocated(SEAT_TABLE)
                    .wrapping_add((idx as i32).wrapping_mul(4) as u32),
            );
            if rd32(entry.wrapping_add(SEAT_ID_OFF)) != a3
                && lf_checker_rt::callee_thiscall!(CAL_DOOR, u32, a0.wrapping_add(DOOR_OFF), 1) as u8 != 0
                && rd8(a2.wrapping_add(IN_VEH_FLAG)) == 0
            {
                let tmp = lf_checker_rt::callee_cdecl!(
                    CAL_RESOLVE, u32, lf_checker_rt::relocated(NAME_TMP), 0
                );
                return build_full(a0, 0xeb194c, a2, tmp);
            }
        }
        let state = rd32(a1.wrapping_add(VEH_STATE));
        if state == 1 {
            if rd8(a2.wrapping_add(IN_VEH_FLAG)) != 0 {
                return 1;
            }
            return build_zero(a0, 0xeb195c);
        }
        if state == 0 {
            let p = lf_checker_rt::callee_thiscall!(
                CAL_PROBE3, u32, a1.wrapping_add(PROBE_OFF)
            );
            if p as u8 != 0 {
                let d = lf_checker_rt::callee_thiscall!(CAL_CHECK2, u32, a1, a2);
                if d as u8 != 0 {
                    if rd8(a2.wrapping_add(IN_VEH_FLAG)) == 0 {
                        let e = lf_checker_rt::callee_thiscall!(CAL_PRED_A, u32, a2);
                        if e as u8 != 0 {
                            return build_full(a0, 0xeb1994, a2, handle);
                        }
                        let f = lf_checker_rt::callee_thiscall!(CAL_PRED_B, u32, a2);
                        if f as u8 != 0 {
                            return build_full(a0, 0xeb19a8, a2, handle);
                        }
                        return build_full(a0, 0xeb19bc, a2, handle);
                    }
                } else if rd8(a2.wrapping_add(IN_VEH_FLAG)) == 0 {
                    return build_zero(a2, 0xeb196c);
                }
                return build_zero(a0, 0xeb197c);
            }
        }
        if lf_checker_rt::callee_thiscall!(CAL_CHECK3, u32, a1, a2) as u8 != 0 {
            build_full(a0, 0xeb19dc, a2, handle)
        } else {
            build_zero(a2, 0xeb19cc)
        }
    }
});
