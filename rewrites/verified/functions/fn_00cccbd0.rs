// original: 0x00CCCBD0 task_factory_dispatch (proposed)

/// Create a ped task object selected by a small integer task id.
///
/// `obj` is the requesting ped (or order) object, `id` the task id. The id is
/// compared signed and selects one of seven builders; every other id,
/// including 0x516, returns null. Each builder first fetches the shared task
/// factory from the `TASK_FACTORY_SLOT` global and bails out with null when
/// it is null, then invokes one constructor-like callee and returns its
/// result:
///
/// | id    | constructor args (this = factory result)              |
/// |-------|-------------------------------------------|
/// | 0xCF  | (0x2C, 0xBE, 0x1388)                      |
/// | 0xCD  | (-1, -1, 1)                               |
/// | 0xD9  | (0, 0, 0x2C, 0xBE, 4.0, 0, 1)             |
/// | 0x38F | (hint, 0, 10000.0, 0x186A0, K_B, K_A, 0)  |
/// | 0x845 | (2, flag ? 0x66 : 0x67)                   |
/// | 0x83F | (0x1388, 0x3A98, 0, 0, 0), then maybe tag |
/// | 0x838 | (0x3E8, 0x2710, -1.0, -1.0)               |
///
/// The 0x38F branch first asks a helper for a hint object, then replaces it
/// with the word at `+0x44` of a manager lookup (manager at the constant
/// address `PED_MGR_THIS`) when bit 21 of the word at `obj+0x28` is set and
/// the lookup yields a non-null word. `K_A`/`K_B` are two read-only float
/// constants. The 0x845 branch reads the flag byte at `obj+0x219`.
/// The 0x83F branch afterwards checks the `MODEL_FLAG` byte: when set it
/// sign-extends the word at `obj+0x2E`, indexes the `PED_MODEL_TABLE` with
/// it, compares the byte at entry `+0xEF` (unsigned) against the
/// `MODEL_LIMIT` dword (signed `jge`), and writes 1 at result `+0x4C` when
/// the byte is below the limit.
///
/// Original: 0x00CCCBD0 (stdcall, two stack words; ECX is scratch).
lf_checker_rt::export!(stdcall, rw_00cccbd0(obj: u32, id: u32) -> u32 {
    unsafe {
        const TASK_FACTORY_SLOT: u32 = 0x0167E2A0;
        const PED_MODEL_TABLE: u32 = 0x01295CD8;
        const MODEL_LIMIT: u32 = 0x010516AC;
        const MODEL_FLAG: u32 = 0x010521A0;
        const CONST_FLOAT_A: u32 = 0x00EEF94C;
        const CONST_FLOAT_B: u32 = 0x00EEF948;
        const PED_MGR_THIS: u32 = 0x012E2420;
        const HINT_BIT: u32 = 21;
        const TAG_OFFSET: u32 = 0x4C;
        const GETTER: u32 = 1;
        const B1_CTOR: u32 = 2;
        const B2_CTOR: u32 = 3;
        const B3_CTOR: u32 = 4;
        const HINT_HELPER: u32 = 5;
        const MGR_LOOKUP: u32 = 6;
        const B4_CTOR: u32 = 7;
        const B5_CTOR: u32 = 8;
        const B6_CTOR: u32 = 9;
        const B7_CTOR: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn factory() -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(TASK_FACTORY_SLOT)) }
        }

        let id = id as i32;
        if id > 0x516 {
            let t = (id as u32).wrapping_sub(0x838);
            if t == 0 {
                // 0x838.
                let f = factory();
                let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
                if v == 0 {
                    return 0;
                }
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    B7_CTOR, u32, v, 0x3E8, 0x2710, 0xBF80_0000, 0xBF80_0000
                );
                return r;
            }
            let t2 = t.wrapping_sub(7);
            if t2 == 0 {
                // 0x83F.
                let f = factory();
                let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
                let r = if v == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        B6_CTOR, u32, v, 0x1388, 0x3A98, 0, 0, 0
                    )
                };
                if rd8(lf_checker_rt::relocated(MODEL_FLAG)) == 0 {
                    return r;
                }
                let idx = (obj.wrapping_add(0x2E) as *const i16).read_unaligned() as i32;
                let entry = rd32(
                    lf_checker_rt::relocated(PED_MODEL_TABLE)
                        .wrapping_add((idx as u32).wrapping_mul(4)),
                );
                let level = rd8(entry.wrapping_add(0xEF)) as i32;
                let limit = rd32(lf_checker_rt::relocated(MODEL_LIMIT)) as i32;
                if level < limit {
                    ((r as *mut u32).wrapping_byte_add(TAG_OFFSET as usize)).write_unaligned(1);
                }
                return r;
            }
            if t2.wrapping_sub(6) != 0 {
                return 0;
            }
            // 0x845.
            let flag = rd8(obj.wrapping_add(0x219));
            let kind = if flag != 0 { 0x66 } else { 0x67 };
            let f = factory();
            let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
            if v == 0 {
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(B5_CTOR, u32, v, 2, kind);
            return r;
        }
        if id == 0x516 {
            return 0;
        }
        if id > 0xD9 {
            if id != 0x38F {
                return 0;
            }
            // 0x38F.
            let mut hint: u32 = lf_checker_rt::callee_cdecl!(HINT_HELPER, u32,);
            let bits = rd32(obj.wrapping_add(0x28));
            if (bits >> HINT_BIT) & 1 != 0 {
                let found: u32 = lf_checker_rt::callee_thiscall!(
                    MGR_LOOKUP, u32,
                    lf_checker_rt::relocated(PED_MGR_THIS), obj
                );
                if found != 0 {
                    let alt = rd32(found.wrapping_add(0x44));
                    if alt != 0 {
                        hint = alt;
                    }
                }
            }
            let f = factory();
            let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
            if v == 0 {
                return 0;
            }
            let ka = rd32(lf_checker_rt::relocated(CONST_FLOAT_A));
            let kb = rd32(lf_checker_rt::relocated(CONST_FLOAT_B));
            let r: u32 = lf_checker_rt::callee_thiscall!(
                B4_CTOR, u32, v, hint, 0, 0x447A_0000, 0x186A0, kb, ka, 0
            );
            return r;
        }
        if id == 0xD9 {
            // 0xD9.
            let f = factory();
            let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
            if v == 0 {
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                B3_CTOR, u32, v, 0, 0, 0x2C, 0xBE, 0x4080_0000, 0, 1
            );
            return r;
        }
        let t = (id as u32).wrapping_sub(0xCD);
        if t == 0 {
            // 0xCD.
            let f = factory();
            let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
            if v == 0 {
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                B2_CTOR, u32, v, 0xFFFF_FFFF, 0xFFFF_FFFF, 1
            );
            return r;
        }
        if t.wrapping_sub(2) != 0 {
            return 0;
        }
        // 0xCF.
        let f = factory();
        let v: u32 = lf_checker_rt::callee_thiscall!(GETTER, u32, f);
        if v == 0 {
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(B1_CTOR, u32, v, 0x2C, 0xBE, 0x1388);
        r
    }
});
