// original: 0x008D51C0 emit_quad_lazy_state
/// Quad emitter, lazy-state variant.
///
/// Emits one quad from lazily-initialized global state. Two cached float
/// pairs are initialized on first use (tracked by flag bits, one pair to
/// zero and one to a constant); when the transform flag is set, each pair
/// is passed through a transform call that replaces it in place, otherwise
/// the cached values are used directly. After two setup calls it opens a
/// 4-by-4 primitive, records the key in a global, and emits the four
/// corners (x0,y0), (x0,y1), (x1,y0), (x1,y1). Every vertex carries the key,
/// a zero word, three global float words and a global UV pair. It then runs
/// the end primitive and tail-calls the shared tail fragment with the key,
/// whose result is returned.
lf_checker_rt::export!(cdecl, rb92_fn7(key: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_XF: u32 = 1; // transform one pair in place (cdecl/1, out-param)
    const CAL_A: u32 = 2; // setup (cdecl/1)
    const CAL_B: u32 = 3; // setup (cdecl/0)
    const CAL_BEGIN: u32 = 4; // begin primitive (cdecl/2)
    const CAL_VERT: u32 = 5; // emit one vertex (cdecl/9)
    const CAL_MID: u32 = 6; // end primitive (cdecl/0)
    const CAL_TAIL: u32 = 7; // shared tail fragment (cdecl/1)

    const G_FLAGS: u32 = 0x0117_3734; // init bits: 1 = pair0 ready, 2 = pair1 ready
    const G_X0: u32 = 0x0117_372C; // cached pair0
    const G_Y0: u32 = 0x0117_3730;
    const G_X1: u32 = 0x0117_3738; // cached pair1
    const G_Y1: u32 = 0x0117_373C;
    const G_C1: u32 = 0x00FE_88E8; // pair1 init constant (pristine)
    const G_BR: u32 = 0x0103_2774; // nonzero = transform the pairs
    const G_F0: u32 = 0x017F_59F0; // vertex UV second word
    const G_F4: u32 = 0x017F_59F4; // vertex UV first word
    const G_EC: u32 = 0x017F_59EC; // vertex middle words
    const G_F8: u32 = 0x017F_59F8;
    const G_FC: u32 = 0x017F_59FC;
    const G_KEY: u32 = 0x0110_DFB0; // last emitted key

    #[inline(always)]
    fn g32(file_va: u32) -> u32 {
        unsafe { *(lf_checker_rt::global::<u32>(file_va) as *const u32) }
    }

    #[inline(always)]
    fn set32(file_va: u32, v: u32) {
        unsafe {
            *(lf_checker_rt::global::<u32>(file_va) as *mut u32) = v;
        }
    }

    unsafe {
        let mut f = g32(G_FLAGS);
        let (x0, y0) = if f & 1 == 0 {
            f |= 1;
            set32(G_FLAGS, f);
            set32(G_X0, 0);
            set32(G_Y0, 0);
            (0u32, 0u32)
        } else {
            (g32(G_X0), g32(G_Y0))
        };
        let (x1, y1) = if f & 2 == 0 {
            f |= 2;
            set32(G_FLAGS, f);
            let c = g32(G_C1);
            set32(G_X1, c);
            set32(G_Y1, c);
            (c, c)
        } else {
            (g32(G_X1), g32(G_Y1))
        };
        let br = *(lf_checker_rt::global::<u8>(G_BR) as *const u8);
        let (px0, py0, px1, py1) = if br != 0 {
            let mut p1 = [x0, y0];
            lf_checker_rt::callee_cdecl!(CAL_XF, u32, p1.as_mut_ptr() as u32);
            let mut p2 = [x1, y1];
            lf_checker_rt::callee_cdecl!(CAL_XF, u32, p2.as_mut_ptr() as u32);
            (p1[0], p1[1], p2[0], p2[1])
        } else {
            (x0, y0, x1, y1)
        };
        lf_checker_rt::callee_cdecl!(CAL_A, u32, 0u32);
        lf_checker_rt::callee_cdecl!(CAL_B, u32,);
        lf_checker_rt::callee_cdecl!(CAL_BEGIN, u32, 4u32, 4u32);
        set32(G_KEY, key);
        let (f0, f4, ec, f8, fc) = (g32(G_F0), g32(G_F4), g32(G_EC), g32(G_F8), g32(G_FC));
        let corners = [(px0, py0), (px0, py1), (px1, py0), (px1, py1)];
        let mut i = 0;
        while i < 4 {
            let (bx, by) = corners[i];
            lf_checker_rt::callee_cdecl!(
                CAL_VERT, u32, bx, by, 0u32, ec, f8, fc, key, f4, f0
            );
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(CAL_MID, u32,);
        lf_checker_rt::callee_cdecl!(CAL_TAIL, u32, key)
    }
});
