// original: 0x009aa180 CONV_GANG_STATE
/// Classify `id` into a gang state by three ordered membership tests.
///
/// Each stage asks the membership oracle (stubbed, cdecl/5) whether
/// `id` belongs to that stage's gang tag (a constant address), with a
/// stack out-word the oracle may fill but this function never reads.
/// The first stage hit returns 3, the second 2, the third returns 1
/// for a hit and 0 for a miss. Stdcall, one stack word, byte result.
export!(stdcall, rw_009AA180(id: u32) -> u32 {
    unsafe {
        const TAG_A: u32 = 0xe91ce0;
        const TAG_B: u32 = 0xe91cf0;
        const TAG_C: u32 = 0xe91cfc;
        let mut scratch: u32 = 0;
        let out = &mut scratch as *mut u32 as u32;
        let a: u32 = callee_cdecl!(1, u32, id, relocated(TAG_A), out, 0, 0);
        if a as u8 != 0 {
            return 3;
        }
        let b: u32 = callee_cdecl!(2, u32, id, relocated(TAG_B), out, 0, 0);
        if b as u8 != 0 {
            return 2;
        }
        let c: u32 = callee_cdecl!(3, u32, id, relocated(TAG_C), out, 0, 0);
        (c as u8 != 0) as u32
    }
});
