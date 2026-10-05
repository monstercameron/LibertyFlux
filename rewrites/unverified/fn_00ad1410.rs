// original: 0x00AD1410 audio_bank_refresh (proposed)

/// Refresh the audio bank set when the selection changes.
///
/// `count`, `rate`, `mode` and `format` describe the wanted bank set. When
/// the probe (id 1) reports idle and the four stored words already equal
/// the arguments, nothing happens. Otherwise the words are stored, every
/// live bank object in the `count`-long table at `BANK_TAB` plus the two
/// side slots is released (each drops its reference word; one falling to
/// zero with a kept type runs virtual slot 0 (id 2) with argument 1) and
/// every slot is cleared. A twelve-word descriptor is then built (unit
/// bytes, the extension word, the extension-present flag, a mode word):
/// the frame call (id 3, thiscall, pops nothing) runs first and the bytes
/// are set after it, exactly like the original. Three banks are created
/// through the factory (id 7) with mode 9, 6, 5, 2 or 0 selected by
/// `format` (the 0x20 case asks probe id 4; any other value keeps the
/// blank word), then a second mode word is chosen (probe id 5 under the
/// override flag, else probe id 6 with the extension word, else 2) and two
/// more banks are created, the second with a forced mode word of 0xe.
/// When the extension word is nonzero a link block runs (ids 8-13, two
/// thiscall rounds on created banks, raw words through helper calls). A
/// sixth bank is created with a descriptor whose first word is a second
/// blank, then a three-deep virtual chain (ids 14-16) finalises it, and
/// the neighbour finaliser (id 17) ends the refresh. Two helper arguments
/// pass the blank frame word (see the contract's stack fill).
///
/// Original: 0x00AD1410 (cdecl, four stack words; no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ad1410(count: u32, rate: u32, mode: u32, format: u32) -> u32 {
    unsafe {
        const PROBE_THIS: u32 = 0x0118_D7F0;
        const ST_COUNT: u32 = 0x0103_F434;
        const ST_RATE: u32 = 0x0154_E168;
        const ST_MODE: u32 = 0x0154_E16C;
        const ST_FORMAT: u32 = 0x0103_F438;
        const BANK_TAB: u32 = 0x0154_E170;
        const SIDE_A: u32 = 0x0154_E184;
        const SIDE_B: u32 = 0x0154_E180;
        const SLOT_C: u32 = 0x0154_E17C;
        const FACTORY: u32 = 0x017F_5630;
        const EXT: u32 = 0x018D_C2AC;
        const OVERRIDE: u32 = 0x0104_5538;
        const CHAIN_PTR: u32 = 0x0154_E188;
        const OBJ_TYPE: u32 = 8;
        const OBJ_REF: u32 = 0x0a;
        const RELEASE_SLOT: u32 = 0;
        const CREATE_SLOT: u32 = 0x38;
        const FINAL_SLOT: u32 = 0x50;
        const CHAIN_SLOT0: u32 = 0x10;
        const CHAIN_SLOT1: u32 = 0x48;
        const PROBE: u32 = 1;
        const RELEASE: u32 = 2;
        const FRAME: u32 = 3;
        const PROBE5: u32 = 4;
        const PROBE4: u32 = 5;
        const PROBE3: u32 = 6;
        const CREATE: u32 = 7;
        const LINK0: u32 = 8;
        const LINK1: u32 = 9;
        const ROUND: u32 = 10;
        const LINK2: u32 = 11;
        const LINK3: u32 = 12;
        const FINAL: u32 = 13;
        const CH0: u32 = 14;
        const CH1: u32 = 15;
        const CH2: u32 = 16;
        const FIN: u32 = 17;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }

        let xb = lf_checker_rt::xbase();
        let g = |va: u32| va.wrapping_sub(0x400000).wrapping_add(xb);
        let probe = lf_checker_rt::callee_thiscall!(
            PROBE,
            u32,
            lf_checker_rt::relocated(PROBE_THIS)
        );
        if (probe as u8) == 0
            && rd32(g(ST_COUNT)) == count
            && rd32(g(ST_RATE)) == rate
            && rd32(g(ST_MODE)) == mode
            && rd32(g(ST_FORMAT)) == format
        {
            return 0;
        }
        wr32(g(ST_COUNT), count);
        wr32(g(ST_RATE), rate);
        wr32(g(ST_MODE), mode);
        wr32(g(ST_FORMAT), format);
        unsafe fn release_one(obj: u32) {
            unsafe {
                let rc = rd16(obj.wrapping_add(0x0a));
                if rc == 0 {
                    return;
                }
                let ty = rd8(obj.wrapping_add(8));
                wr16(obj.wrapping_add(0x0a), rc.wrapping_sub(1) & 0xffff);
                if rc.wrapping_sub(1) & 0xffff != 0 {
                    return;
                }
                if ty != 2 && ty != 4 {
                    return;
                }
                let vt = rd32(obj);
                let target = rd32(vt.wrapping_add(0));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(target as usize) };
                f(obj, 1);
            }
        }
        // The original runs i = 0 while i < count (signed), after a
        // signed non-positive skip.
        if (count as i32) > 0 {
            let mut j = 0u32;
            while (j as i32) < (count as i32) {
                let slot = g(BANK_TAB).wrapping_add(j.wrapping_mul(4));
                let obj = rd32(slot);
                if obj != 0 {
                    release_one(obj);
                }
                wr32(slot, 0);
                j = j.wrapping_add(1);
            }
        }
        for slot_va in [SIDE_A, SIDE_B] {
            let slot = g(slot_va);
            let obj = rd32(slot);
            if obj != 0 {
                release_one(obj);
            }
            wr32(slot, 0);
        }
        let factory = rd32(g(FACTORY));
        let fvt = rd32(factory);
        let create_target = rd32(fvt.wrapping_add(CREATE_SLOT));
        let create_fn: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(create_target as usize) };
        let mut desc = [0u32; 12];
        let dp = (&desc[0] as *const u32) as u32;
        lf_checker_rt::callee_thiscall!(FRAME, u32, dp, 0);
        let ext = rd32(g(EXT));
        let setne = if ext != 0 { 1u32 } else { 0u32 };
        desc[0] = 1;
        desc[1] = ext;
        desc[2] = 1;
        desc[4] = 0x0001_0101;
        desc[5] = 0;
        desc[9] = setne;
        let mode1 = if format == 0x80 {
            9u32
        } else if format == 0x40 {
            6u32
        } else if format == 0x20 {
            if (lf_checker_rt::callee_cdecl!(PROBE5, u32, 5) as u8) != 0 {
                5
            } else {
                2
            }
        } else {
            0
        };
        desc[11] = mode1;
        wr32(g(BANK_TAB), create_fn(factory, lf_checker_rt::relocated(0x00EA6900), 3, rate, mode, format, dp));
        wr32(
            g(BANK_TAB).wrapping_add(4),
            create_fn(factory, lf_checker_rt::relocated(0x00EA6918), 3, rate, mode, format, dp),
        );
        wr32(
            g(BANK_TAB).wrapping_add(8),
            create_fn(factory, lf_checker_rt::relocated(0x00EA6930), 3, rate, mode, format, dp),
        );
        let (mode2, dx2) = if rd32(g(OVERRIDE)) == 1 {
            wr32(g(ST_FORMAT), 0x20);
            if (lf_checker_rt::callee_cdecl!(PROBE4, u32, 4) as u8) != 0 {
                (4u32, 0x20u32)
            } else {
                (2u32, 0x20u32)
            }
        } else if (lf_checker_rt::callee_cdecl!(PROBE3, u32, 3) as u8) != 0 && ext == 0 {
            wr32(g(ST_FORMAT), 0x10);
            (3u32, 0x10u32)
        } else {
            wr32(g(ST_FORMAT), 0x20);
            (2u32, 0x20u32)
        };
        desc[11] = mode2;
        wr32(g(SIDE_B), create_fn(factory, lf_checker_rt::relocated(0x00EA6948), 3, rate, mode, dx2, dp));
        desc[11] = 0x0e;
        wr32(g(SLOT_C), create_fn(factory, lf_checker_rt::relocated(0x00EA695C), 3, rate, mode, 0x20, dp));
        if setne == 1 {
            lf_checker_rt::callee_stdcall!(LINK0, u32, 0);
            let z = [0u32; 1];
            let zp = (&z[0] as *const u32) as u32;
            lf_checker_rt::callee_cdecl!(LINK1, u32, zp);
            let r1 = lf_checker_rt::callee_thiscall!(ROUND, u32, rd32(g(BANK_TAB)));
            lf_checker_rt::callee_cdecl!(LINK2, u32, 0, r1);
            let r2 = lf_checker_rt::callee_thiscall!(ROUND, u32, rd32(g(SLOT_C)));
            lf_checker_rt::callee_cdecl!(LINK3, u32, r2);
            let fvt2 = rd32(factory);
            let t50 = rd32(fvt2.wrapping_add(FINAL_SLOT));
            let f50: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(t50 as usize) };
            f50(factory, rd32(g(SLOT_C)));
            lf_checker_rt::callee_cdecl!(LINK2, u32, 0, 0);
            lf_checker_rt::callee_cdecl!(LINK3, u32, 0);
        }
        let mut t = [0u32; 13];
        t[1] = desc[0];
        t[2] = desc[1];
        t[3] = desc[2];
        t[4] = desc[3];
        t[5] = desc[4];
        t[6] = desc[5];
        t[7] = desc[6];
        t[8] = desc[7];
        t[9] = desc[8];
        t[10] = desc[9];
        t[11] = desc[10];
        let tp = (&t[0] as *const u32) as u32;
        wr32(g(SIDE_A), create_fn(factory, lf_checker_rt::relocated(0x00EA6974), 3, rate, mode, 0x20, tp));
        let pa = rd32(g(SIDE_A));
        let pvt = rd32(pa);
        let t10 = rd32(pvt.wrapping_add(CHAIN_SLOT0));
        let f10: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(t10 as usize) };
        f10(pa);
        let t48 = rd32(pvt.wrapping_add(CHAIN_SLOT1));
        let f48: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(t48 as usize) };
        let q = f48(pa);
        let qvt = rd32(q);
        let t482 = rd32(qvt.wrapping_add(CHAIN_SLOT1));
        let f482: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(t482 as usize) };
        f482(qvt, q, 0, lf_checker_rt::relocated(0x0154E188));
        lf_checker_rt::callee_cdecl!(LINK3, u32, rd32(g(CHAIN_PTR)));
        lf_checker_rt::callee_cdecl!(FIN, u32,);
        0
    }
});
