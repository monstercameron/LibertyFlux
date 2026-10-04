// original: 0x00e5ff00 zero_region_019f2178_200
/// Zero a fixed scratch region through the shared fill routine.
///
/// Forwards the region address, the zero byte and the length;
/// returns the fill routine's result.
export!(cdecl, rw_00e5ff00() -> u32 {
    unsafe {
        /// Start of the region to zero.
        const REGION: u32 = 0x019F2178;
        /// Length in bytes.
        const LEN: u32 = 0x200;
        callee_cdecl!(1, u32, relocated(REGION), 0, LEN)
    }
});
