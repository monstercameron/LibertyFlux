// original: 0x008D30B0 NativeImpl_GET_VEHICLE_PLAYER_WOULD_ENTER
/// Searches for the entry the player would use: cached handle, alternate
/// object, then a spatial query over a clamped bounding box.
///
/// `a0` is the search context, `a1` a three-float query point and `a2` a
/// tag byte. A skip bit returns null; a live cached handle is probed and
/// returned when it answers; otherwise a linked alternate is admitted by
/// its mode and generation words. When neither answers, a bounding box is
/// built from the context position widened by a global radius and clamped
/// to global limits, and a spatial query over it yields a candidate that
/// is probed before being returned. Returns the accepted object or null.
lf_checker_rt::export!(cdecl, rb109_fn2(a0: u32, a1: u32, a2: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_PROBE: u32 = 1; // liveness probe (thiscall/0, al result)
    const CAL_ADMIT: u32 = 2; // alternate admission (thiscall/1: context)
    const CAL_QUERY: u32 = 3; // spatial query (cdecl/5: bbox, key, params, 4, 13)

    // Object layout.
    const OFF_SKIP: u32 = 0x268; // skip-flag word
    const SKIP_BIT: u32 = 0x2000;
    const OFF_INNER: u32 = 0x228; // cached-handle owner link
    const INNER_HANDLE: u32 = 0x5A0; // cached handle, 0 when empty
    const OFF_ALT: u32 = 0xAB0; // alternate object link
    const ALT_MODE: u32 = 0x28; // alternate mode word
    const ALT_MODE_MASK: u32 = 0x3C0;
    const ALT_MODE_WANT: u32 = 0x80;
    const ALT_GEN: u32 = 0x1304; // alternate generation word
    const ALT_GEN_WANT: u32 = 2;
    const OFF_POS: u32 = 0x20; // position record link

    // Globals (file VAs; resolved through the worker's image base).
    const G_RADIUS: u32 = 0x01032360; // box half-width (f32)
    const G_HI: u32 = 0x00FE8D18; // upper clamp (f32)
    const G_LO: u32 = 0x00FE8E1C; // lower clamp (f32)

    // Query key: a code address pushed as an immediate. The immediate cell
    // is in the reloc table, so the original pushes the relocated address;
    // derive it the same way (Verified: worker replay + live call log).
    const QUERY_KEY_FILE_VA: u32 = 0x008D1920;

    #[inline(always)]
    unsafe fn load(base: u32, off: u32) -> u32 {
        *((base.wrapping_add(off)) as *const u32)
    }
    #[inline(always)]
    unsafe fn fload(base: u32, off: u32) -> f32 {
        *((base.wrapping_add(off)) as *const f32)
    }

    unsafe {
        if (load(a0, OFF_SKIP) & SKIP_BIT) != 0 {
            return 0;
        }
        let inner = load(a0, OFF_INNER);
        let handle = load(inner, INNER_HANDLE);
        if handle != 0 {
            let ok = lf_checker_rt::callee_thiscall!(CAL_PROBE, u32, handle);
            if (ok & 0xFF) == 0 {
                return 0;
            }
            return handle;
        }
        let alt = load(a0, OFF_ALT);
        if alt != 0
            && (load(alt, ALT_MODE) & ALT_MODE_MASK) == ALT_MODE_WANT
            && load(alt, ALT_GEN) == ALT_GEN_WANT
        {
            let ok = lf_checker_rt::callee_thiscall!(CAL_ADMIT, u32, alt, a0);
            if (ok & 0xFF) != 0 {
                return alt;
            }
        }
        // Bounding box from the context position +- radius, clamped. Each
        // select below mirrors one `comiss` + conditional jump exactly: the
        // comparison uses only `>`, so unordered (NaN) inputs take the same
        // fall-through side as the hardware jump does.
        let pos = load(a0, OFF_POS);
        let px = fload(pos, 0x30);
        let py = fload(pos, 0x34);
        let pz = fload(pos, 0x38);
        let rad = *(lf_checker_rt::global::<f32>(G_RADIUS));
        let hi = *(lf_checker_rt::global::<f32>(G_HI));
        let glo = *(lf_checker_rt::global::<f32>(G_LO));
        let hi_x = rad + px;
        let lo_x = px - rad;
        let hi_y = rad + py;
        let lo_y = py - rad;
        let mut min_x = if hi_x > hi { hi } else { hi_x };
        let mut min_y = if hi_y > hi { hi } else { hi_y };
        let mut min_z = if pz > hi { hi } else { pz };
        let mut max_x = if glo > hi_x { glo } else { hi_x };
        let mut max_y = if glo > hi_y { glo } else { hi_y };
        let mut max_z = if glo > pz { glo } else { pz };
        min_x = if lo_x > min_x { min_x } else { lo_x };
        min_y = if lo_y > min_y { min_y } else { lo_y };
        min_z = if pz > min_z { min_z } else { pz };
        max_x = if max_x > lo_x { max_x } else { lo_x };
        max_y = if max_y > lo_y { max_y } else { lo_y };
        max_z = if max_z > pz { max_z } else { pz };
        // Box word 3 is a scratch slot the original never initializes; the
        // contract defines uninitialized stack as 0 (`stack_fill`), so both
        // sides observe 0.0 here.
        let bbox = [min_x, min_y, min_z, 0.0f32, max_x, max_y, max_z];
        // Query parameter block: context link, the candidate out-slot
        // (zeroed before the call, written by the callee), a zero word, a
        // scratch word (also defined-0), the query point, a trailing scratch
        // word, and the tag byte in a zero word.
        let qx = *(a1 as *const f32);
        let qy = *((a1.wrapping_add(4)) as *const f32);
        let qz = *((a1.wrapping_add(8)) as *const f32);
        let mut params = [0u32; 9];
        params[0] = a0;
        params[4] = qx.to_bits();
        params[5] = qy.to_bits();
        params[6] = qz.to_bits();
        params[8] = a2 & 0xFF;
        lf_checker_rt::callee_cdecl!(
            CAL_QUERY, u32,
            bbox.as_ptr() as u32,
            lf_checker_rt::relocated(QUERY_KEY_FILE_VA),
            params.as_ptr() as u32,
            4,
            0x0D
        );
        let out = params[1];
        if out == 0 {
            return 0;
        }
        let ok = lf_checker_rt::callee_thiscall!(CAL_PROBE, u32, out);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        out
    }
});
