// original: 0x008D4990 emit_quad_grid_by_value
/// Quad emitter, by-value grid variant.
///
/// Runs the shared setup sequence (two plain calls, then two object calls
/// with the receiver and mode taken from globals), opens a 4-by-4 primitive,
/// and emits the four corners of the axis-aligned grid spanned by `x0`/`x1`
/// and `y0`/`y1` at height `z`, in (x0,y0), (x0,y1), (x1,y0), (x1,y1) order.
/// Every vertex carries the key dword `*k`, two zero words, the constant
/// float -1.0 and a (0,0) UV. It then runs the two teardown calls and
/// tail-calls the final object call with the same six arguments, whose
/// result is returned.
lf_checker_rt::export!(cdecl, rb92_fn5(x0: f32, y0: f32, x1: f32, y1: f32, z: f32, k: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_SETUP: u32 = 1; // 0x432C20 setup (cdecl/2)
    const CAL_STATE: u32 = 2; // state helper (cdecl/1)
    const CAL_CFG3: u32 = 3; // configure, 3 args (thiscall/3)
    const CAL_CFG1: u32 = 4; // configure, 1 arg (thiscall/1)
    const CAL_BEGIN: u32 = 5; // begin primitive (cdecl/2)
    const CAL_VERT: u32 = 6; // emit one vertex (cdecl/9)
    const CAL_MID: u32 = 7; // end primitive (cdecl/0)
    const CAL_FIN0: u32 = 8; // finish (thiscall/0)
    const CAL_TAIL: u32 = 9; // final call, tail-called (thiscall/6)

    const G_THIS: u32 = 0x0117_36C8; // object receiver for the object calls
    const G_MODE: u32 = 0x0117_36D0; // mode word forwarded to the configure call
    const NEG_ONE_BITS: u32 = 0xBF80_0000; // -1.0f, fixed vertex field

    #[inline(always)]
    fn g32(file_va: u32) -> u32 {
        unsafe { *(lf_checker_rt::global::<u32>(file_va) as *const u32) }
    }

    unsafe {
        lf_checker_rt::callee_cdecl!(CAL_SETUP, u32, 0u32, 0u32);
        lf_checker_rt::callee_cdecl!(CAL_STATE, u32, 0u32);
        let this = g32(G_THIS);
        let mode = g32(G_MODE);
        lf_checker_rt::callee_thiscall!(CAL_CFG3, u32, this, 2u32, 0u32, mode);
        lf_checker_rt::callee_thiscall!(CAL_CFG1, u32, this, 0u32);
        lf_checker_rt::callee_cdecl!(CAL_BEGIN, u32, 4u32, 4u32);
        let key = *(k as *const u32);
        let (xb0, yb0, xb1, yb1, zb) = (x0.to_bits(), y0.to_bits(), x1.to_bits(), y1.to_bits(), z.to_bits());
        let corners = [(xb0, yb0), (xb0, yb1), (xb1, yb0), (xb1, yb1)];
        let mut i = 0;
        while i < 4 {
            let (bx, by) = corners[i];
            lf_checker_rt::callee_cdecl!(
                CAL_VERT, u32, bx, by, zb, 0u32, 0u32, NEG_ONE_BITS, key, 0u32, 0u32
            );
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(CAL_MID, u32,);
        lf_checker_rt::callee_thiscall!(CAL_FIN0, u32, this);
        lf_checker_rt::callee_thiscall!(CAL_TAIL, u32, this, xb0, yb0, xb1, yb1, zb, k)
    }
});
