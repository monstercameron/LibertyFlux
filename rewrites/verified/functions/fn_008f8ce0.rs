// original: 0x008F8CE0 stream_request_dispatch_16arg
/// Validate two request codes, then dispatch all sixteen words.
///
/// The first code (second stack word) must be -1, 0 or 6 and the
/// second code (fourth word) likewise when the middle word is
/// nonzero; anything else returns the first code unchanged. The
/// valid request is forwarded, with the whole sixteen-word frame, to
/// the pool core on the first global pool, or on the second pool
/// when both the middle word is zero and the first code is not 6.
/// Cdecl, sixteen stack arguments; returns the call's result.
export!(cdecl, rw_008f8ce0(a0: u32, a1: u32, a2: u32, a3: u32,
        a4: u32, a5: u32, a6: u32, a7: u32,
        a8: u32, a9: u32, a10: u32, a11: u32,
        a12: u32, a13: u32, a14: u32, a15: u32) -> u32 {
    unsafe {
        const POOL_A: u32 = 0x118e7c0;
        const POOL_B: u32 = 0x118e7c8;
        const SPECIAL: u32 = 6;
        let ai = a1 as i32;
        if ai < -1 {
            return a1;
        }
        let dl = if ai > 0 {
            if a1 != SPECIAL {
                return a1;
            }
            true
        } else {
            false
        };
        let path_a = if a2 == 0 {
            dl
        } else {
            let ci = a3 as i32;
            if ci < -1 {
                return a1;
            }
            if ci > 0 && a3 != SPECIAL {
                return a1;
            }
            true
        };
        if path_a {
            callee_thiscall!(1, u32, relocated(POOL_A),
                a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15)
        } else {
            callee_thiscall!(2, u32, relocated(POOL_B),
                a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15)
        }
    }
});
