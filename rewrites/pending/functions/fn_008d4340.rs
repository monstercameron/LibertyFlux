// original: 0x008D4340 emit_quads_record_loop
/// Quad emitter, record-loop variant.
///
/// Runs the shared setup sequence (two plain calls, then two object calls
/// with the receiver and mode taken from globals), opens a 3-by-`6*n`
/// primitive, and walks `n` 36-byte records at `rec`. Each record holds a
/// 2x2 grid — x in words 0/2, y in words 1/3, U in words 4/6, V in words
/// 5/7, key in word 8 — emitted as six vertices in the order
/// (x0,y0), (x0,y1), (x1,y0), (x1,y0), (x0,y1), (x1,y1): the middle two
/// vertices repeat an earlier one each, exactly as the original does. Every
/// vertex also carries three zero words and the constant float -1.0. It
/// then runs the two teardown calls and tail-calls the final object call
/// with the same two arguments, whose result is returned.
lf_checker_rt::export!(cdecl, rb92_fn3(rec: u32, n: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_SETUP: u32 = 1; // 0x432C20 setup (cdecl/2)
    const CAL_STATE: u32 = 2; // state helper (cdecl/1)
    const CAL_CFG3: u32 = 3; // configure, 3 args (thiscall/3)
    const CAL_CFG1: u32 = 4; // configure, 1 arg (thiscall/1)
    const CAL_BEGIN: u32 = 5; // begin primitive (cdecl/2)
    const CAL_VERT: u32 = 6; // emit one vertex (cdecl/9)
    const CAL_MID: u32 = 7; // end primitive (cdecl/0)
    const CAL_FIN0: u32 = 8; // finish (thiscall/0)
    const CAL_TAIL: u32 = 9; // final call, tail-called (thiscall/2)

    const G_THIS: u32 = 0x0117_36C8; // object receiver for the object calls
    const G_MODE: u32 = 0x0117_36CC; // mode word forwarded to the configure call
    const NEG_ONE_BITS: u32 = 0xBF80_0000; // -1.0f, fixed vertex field
    const REC_LEN: u32 = 0x24; // bytes per record

    #[inline(always)]
    fn g32(file_va: u32) -> u32 {
        unsafe { *(lf_checker_rt::global::<u32>(file_va) as *const u32) }
    }

    #[inline(always)]
    fn w(base: u32, word: u32) -> u32 {
        unsafe { *((base.wrapping_add(word.wrapping_mul(4))) as *const u32) }
    }

    #[inline(always)]
    fn vert(bx: u32, by: u32, key: u32, u: u32, v: u32) {
        unsafe {
            lf_checker_rt::callee_cdecl!(
                CAL_VERT, u32, bx, by, 0u32, 0u32, 0u32, NEG_ONE_BITS, key, u, v
            );
        }
    }

    unsafe {
        lf_checker_rt::callee_cdecl!(CAL_SETUP, u32, 0u32, 0u32);
        lf_checker_rt::callee_cdecl!(CAL_STATE, u32, 0u32);
        let this = g32(G_THIS);
        let mode = g32(G_MODE);
        lf_checker_rt::callee_thiscall!(CAL_CFG3, u32, this, 2u32, 0u32, mode);
        lf_checker_rt::callee_thiscall!(CAL_CFG1, u32, this, 0u32);
        lf_checker_rt::callee_cdecl!(CAL_BEGIN, u32, 3u32, n.wrapping_mul(6));
        let mut i: u32 = 0;
        while i < n {
            let base = rec.wrapping_add(i.wrapping_mul(REC_LEN));
            let (x0, y0, x1, y1) = (w(base, 0), w(base, 1), w(base, 2), w(base, 3));
            let (u0, v0, u1, v1) = (w(base, 4), w(base, 5), w(base, 6), w(base, 7));
            let key = w(base, 8);
            vert(x0, y0, key, u0, v0);
            vert(x0, y1, key, u0, v1);
            vert(x1, y0, key, u1, v0);
            vert(x1, y0, key, u1, v0);
            vert(x0, y1, key, u0, v1);
            vert(x1, y1, key, u1, v1);
            i = i.wrapping_add(1);
        }
        lf_checker_rt::callee_cdecl!(CAL_MID, u32,);
        lf_checker_rt::callee_thiscall!(CAL_FIN0, u32, this);
        lf_checker_rt::callee_thiscall!(CAL_TAIL, u32, this, rec, n)
    }
});
