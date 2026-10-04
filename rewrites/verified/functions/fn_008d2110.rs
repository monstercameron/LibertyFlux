// original: 0x008d2110 Impl_GetMissionTrain
/// Mission-train lookup: resolves an object the same way as
/// `Player_GetPedByInfoOrCurrent`, then returns its train field only when the
/// flag word has bit 2 set; otherwise null. (With a null argument and index
/// -1 the original reads field 0x598 of address zero and faults; the rewrite
/// does the same read so the fault matches.)
export!(cdecl, rw_008d2110(arg: u32) -> u32 {
    unsafe {
        const CUR_INDEX: u32 = 0x0103_6F14;
        const TABLE: u32 = 0x011A_8808;
        let mut obj = arg;
        if obj == 0 {
            let index = *global::<i32>(CUR_INDEX);
            obj = if index == -1 {
                0
            } else {
                *global::<u32>(TABLE.wrapping_add((index as u32).wrapping_mul(4)))
            };
        }
        let inner = *((obj + 0x598) as *const u32);
        if inner == 0 {
            return 0;
        }
        if *((inner + 0x26c) as *const u8) & 4 == 0 {
            return 0;
        }
        *((inner + 0xb30) as *const u32)
    }
});
