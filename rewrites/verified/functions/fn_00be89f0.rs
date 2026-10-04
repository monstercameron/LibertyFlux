// original: 0x00BE89F0 WIND_FIRE_LOOP (symbols)
/// Run one wind/fire loop tick for an entity, reporting success.
///
/// `a1` is the entity id, `a2` a token and `a3` a parameter block. A
/// negative id returns 0 at once. Otherwise the context callee (cdecl,
/// 0) must return nonzero; the two validation callees (cdecl, id) must
/// both answer nonzero; the combine callee then runs (thiscall on the
/// fixed loop object, with token, context, a3, id and the two answers)
/// and the context's head word must be nonzero. The name callee (cdecl,
/// "WIND_FIRE_LOOP", 0) resolves the loop token: when it equals `a2`
/// the tune callee runs (thiscall on the head with the -100.0 weight).
/// The settle callee always runs (thiscall on the head with 0, 0, 0),
/// then the commit callee (cdecl, context, id), and the function returns
/// 1. Any failed test returns 0. Callers pass three words (verified from
/// four call sites, each cleaning 12 bytes).
///
/// Original: 0x00BE89F0 (cdecl, three stack words, byte result).
lf_checker_rt::export!(cdecl, rw_00BE89F0(a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const LOOP_OBJECT: u32 = 0x012142B8;
        const LOOP_NAME: u32 = 0x00EB99E4;
        const TUNE_WEIGHT: u32 = 0xc2c8_0000;
        const CONTEXT: u32 = 1;
        const VALID_A: u32 = 2;
        const VALID_B: u32 = 3;
        const COMBINE: u32 = 4;
        const RESOLVE: u32 = 5;
        const TUNE: u32 = 6;
        const SETTLE: u32 = 7;
        const COMMIT: u32 = 8;
        if (a1 as i32) < 0 {
            return 0;
        }
        let ctx: u32 = lf_checker_rt::callee_cdecl!(CONTEXT, u32, 0);
        if ctx == 0 {
            return 0;
        }
        let va: u32 = lf_checker_rt::callee_cdecl!(VALID_A, u32, a1);
        let vb: u32 = lf_checker_rt::callee_cdecl!(VALID_B, u32, a1);
        if va == 0 || vb == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            COMBINE, u32, lf_checker_rt::relocated(LOOP_OBJECT), a2, ctx, a3,
            a1, va, vb
        );
        let head = (ctx as *const u32).read_unaligned();
        if head == 0 {
            return 0;
        }
        let tok: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE, u32, lf_checker_rt::relocated(LOOP_NAME), 0
        );
        if tok == a2 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TUNE, u32, head, TUNE_WEIGHT);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, head, 0, 0, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(COMMIT, u32, a1, ctx);
        1
    }
});

