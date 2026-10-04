// original: 0x009fa7e0 playstat_state_memset_init
/// Clear the three play-stat state regions.
///
/// Zeroes three fixed-size state blocks in global storage through the CRT
/// fill routine, and returns that routine's final result.
export!(cdecl, rw_009fa7e0() -> u32 {
    unsafe {
        const REGIONS: [(u32, u32); 3] = [
            (0x012b9190, 0x3f4),
            (0x012b9588, 0x630),
            (0x012b9bb8, 0x84),
        ];
        let mut answer = 0;
        for (dst, len) in REGIONS {
            answer = callee_cdecl!(1, u32, relocated(dst), 0, len);
        }
        answer
    }
});
