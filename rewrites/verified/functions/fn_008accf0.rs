// original: 0x008accf0 audOcclusionPool_build_and_enable
/// Builds the occlusion pool (next function) then stores the enable-helper's
/// answer for input 1 into the publish slot; returns that answer.
export!(cdecl, rw_008accf0() -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        let r = callee_cdecl!(2, u32, 1);
        *global::<u32>(0x115fd58) = r;
        r
    }
});

