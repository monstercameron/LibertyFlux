// original: 0x00A711C0 CTaskComplexPlayerSettingsTask::vf19

/// Refresh a player settings task from the owner's state word.
///
/// `this` is the task (`+0x18`/`+0x1C` variant words), `arg` a game object
/// whose state word at `[[arg + 0x21C] + 0x12C]` selects `+0x18`: `0x1E`
/// when it is 1 or `0x14`, else `0x1F`.
///
/// The direct path needs a checker predicate (thiscall on `arg + 0x2B0`, 1)
/// and a global gate (no arguments) both true; a variant probe (thiscall
/// on the task) then sets `+0x1C` to `0xD7` (true) or `0xDA` (false). A
/// block is allocated from the pool behind a data global and built through
/// a maker call (thiscall on the block: words `+0x18`/`+0x1C`, `8.0f`, `0`,
/// `1.0f`, `0`), whose result is returned; a null block returns null.
///
/// When either predicate fails, a second try of the gate decides: true
/// resets `+0x1C` to `0xDD` and rejoins the allocation above; false sets
/// `+0x1C` to -1, allocates a plain block (null returns null), runs a
/// zero-argument initializer on it, stamps the vtable constant `0xE9EDBC`
/// with zero words at `+0x14`/`+0x18`, a zero half-word at `+0x1C` and
/// `0x1388` at `+0x20`, and returns the block.
///
/// Original: 0x00A711C0 (thiscall, one stack word). Returns a pointer or
/// null in `eax`; predicate answers use `al` only.
lf_checker_rt::export!(thiscall, rw_00A711C0(this: u32, arg: u32) -> u32 {
    unsafe {
        const WORD18: u32 = 0x18;
        const SEL: u32 = 0x1c;
        const STATE1: u32 = 0x21c;
        const STATE2: u32 = 0x12c;
        const PROBE_OFF: u32 = 0x2b0;
        const MATCH_A: u32 = 0x14;
        const MATCH_B: u32 = 0x01;
        const V_MATCH: u32 = 0x1e;
        const V_OTHER: u32 = 0x1f;
        const SEL_T: u32 = 0xd7;
        const SEL_F: u32 = 0xda;
        const SEL_RETRY: u32 = 0xdd;
        const VT_CONST: u32 = 0xe9edbc; // file VA: relocated at load
        const TAIL_CONST: u32 = 0x1388;
        const POOL_GLOBAL: u32 = 0x167e2a0;
        const C_CHECK: u32 = 1;
        const C_GATE1: u32 = 2;
        const C_PROBE: u32 = 3;
        const C_ALLOC1: u32 = 4;
        const C_MAKE: u32 = 5;
        const C_GATE2: u32 = 6;
        const C_ALLOC2: u32 = 7;
        const C_INIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn low_set(ans: u32) -> bool {
            ans & 0xff != 0
        }

        let state = rd32(rd32(arg + STATE1) + STATE2);
        wr32(
            this + WORD18,
            if state == MATCH_A || state == MATCH_B {
                V_MATCH
            } else {
                V_OTHER
            },
        );
        let direct = low_set(lf_checker_rt::callee_thiscall!(
            C_CHECK,
            u32,
            arg.wrapping_add(PROBE_OFF),
            1
        )) && low_set(lf_checker_rt::callee_cdecl!(C_GATE1, u32,));
        if direct {
            let picked = low_set(lf_checker_rt::callee_thiscall!(C_PROBE, u32, this));
            wr32(this + SEL, if picked { SEL_T } else { SEL_F });
        } else if low_set(lf_checker_rt::callee_cdecl!(C_GATE2, u32,)) {
            wr32(this + SEL, SEL_RETRY);
        } else {
            wr32(this + SEL, 0xffff_ffff);
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let block = lf_checker_rt::callee_thiscall!(C_ALLOC2, u32, pool);
            if block == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(C_INIT, u32, block);
            wr32(block, lf_checker_rt::relocated(VT_CONST));
            wr32(block + 0x14, 0);
            wr32(block + 0x18, 0);
            ((block + 0x1c) as *mut u16).write_unaligned(0);
            wr32(block + 0x20, TAIL_CONST);
            return block;
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let block = lf_checker_rt::callee_thiscall!(C_ALLOC1, u32, pool);
        if block == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            C_MAKE,
            u32,
            block,
            rd32(this + WORD18),
            rd32(this + SEL),
            0x4100_0000,
            0,
            0x3f80_0000,
            0
        )
    }
});
