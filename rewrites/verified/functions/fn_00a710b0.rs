// original: 0x00A710B0 CTaskComplexPlayerPlaceCarBomb::vf19

/// Build the place-car-bomb task replacement, fast path or slow path.
///
/// `this` is the task (`+0x14` a token, `+0x26` a flag byte), `arg` a game
/// object or null. A null token, or a false answer from the comparator
/// (cdecl: token, `this + 0x30`), returns null at once.
///
/// Fast path (non-null `arg` passing a checker predicate on `arg + 0x2B0`):
/// a give call runs (thiscall: 0, 0, 1, 0, result ignored), a block is
/// allocated from the pool behind a data global (null yields a null
/// result), and the block is made with argument `0x1300`.
///
/// Slow path (null `arg` or failed predicate): a first block is allocated
/// (null yields null); a second block is allocated and, unless null (which
/// contributes a zero), feeds a ten-word stdcall probe (2, `this + 0x30`,
/// `0.2f`, `1.0f`, -1, 1, 0, 0, 0, 1) whose x87 float result the original
/// stores into dead stack scratch, then a zero-argument constructor; the
/// first block joins them through an attach call (thiscall on it: that
/// value, 0, 0, 0).
///
/// Unless flag bit 0 of `+0x26` is set, a finalizer runs (thiscall on
/// `this + 0x1C`: `0x18`, result ignored) and the built block is returned.
///
/// Original: 0x00A710B0 (thiscall, one stack word). Returns a pointer or
/// null in `eax`; predicate answers use `al` only.
lf_checker_rt::export!(thiscall, rw_00A710B0(this: u32, arg: u32) -> u32 {
    unsafe {
        const TOKEN: u32 = 0x14;
        const FLAGB: u32 = 0x26;
        const CMP_OFF: u32 = 0x30;
        const PROBE_OFF: u32 = 0x2b0;
        const FIN_OFF: u32 = 0x1c;
        const MAKE_ARG: u32 = 0x1300;
        const FIN_ARG: u32 = 0x18;
        const F20: u32 = 0x3e4ccccd;
        const F1: u32 = 0x3f800000;
        const POOL_GLOBAL: u32 = 0x167e2a0;
        const C_CMP: u32 = 1;
        const C_CHECK: u32 = 2;
        const C_GIVE: u32 = 3;
        const C_ALLOC1: u32 = 4;
        const C_MAKE: u32 = 5;
        const C_ALLOC2: u32 = 6;
        const C_ALLOC3: u32 = 7;
        const C_PROBE: u32 = 8;
        const C_CTOR: u32 = 9;
        const C_ATTACH: u32 = 10;
        const C_FIN: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn low_set(ans: u32) -> bool {
            ans & 0xff != 0
        }

        let token = rd32(this + TOKEN);
        if token == 0 {
            return 0;
        }
        let cmp_base = this.wrapping_add(CMP_OFF);
        if !low_set(lf_checker_rt::callee_cdecl!(C_CMP, u32, token, cmp_base)) {
            return 0;
        }
        let mut built = 0u32;
        let mut probe = 0u32;
        if arg != 0 {
            probe = arg.wrapping_add(PROBE_OFF);
        }
        if arg != 0
            && low_set(lf_checker_rt::callee_thiscall!(C_CHECK, u32, probe, 1))
        {
            lf_checker_rt::callee_thiscall!(C_GIVE, u32, probe, 0, 0, 1, 0);
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let block = lf_checker_rt::callee_thiscall!(C_ALLOC1, u32, pool);
            if block != 0 {
                built = lf_checker_rt::callee_thiscall!(C_MAKE, u32, block, MAKE_ARG);
            }
        } else {
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let first = lf_checker_rt::callee_thiscall!(C_ALLOC2, u32, pool);
            if first != 0 {
                let second = lf_checker_rt::callee_thiscall!(C_ALLOC3, u32, pool);
                let mut inner = 0u32;
                if second != 0 {
                    let _dropped: f32 = lf_checker_rt::callee_stdcall!(
                        C_PROBE, f32, 2, cmp_base, F20, F1, 0xffff_ffff, 1, 0, 0, 0, 1
                    );
                    inner = lf_checker_rt::callee_thiscall!(C_CTOR, u32, second);
                }
                built = lf_checker_rt::callee_thiscall!(C_ATTACH, u32, first, inner, 0, 0, 0);
            }
        }
        if rd8(this + FLAGB) & 1 == 0 {
            lf_checker_rt::callee_thiscall!(
                C_FIN,
                u32,
                this.wrapping_add(FIN_OFF),
                FIN_ARG
            );
        }
        built
    }
});
