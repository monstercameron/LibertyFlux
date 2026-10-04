// original: 0x008d20e0 Player_GetPedByInfoOrCurrent
/// Player ped lookup: returns the ped field of the given info object, or of
/// the current player info (taken from the player table via the current
/// index) when passed null. Null table entries and index -1 yield null.
export!(cdecl, rw_008d20e0(arg: u32) -> u32 {
    unsafe {
        const CUR_INDEX: u32 = 0x0103_6F14;
        const TABLE: u32 = 0x011A_8808;
        const PED_FIELD: u32 = 0x598;
        let mut obj = arg;
        if obj == 0 {
            let index = *global::<i32>(CUR_INDEX);
            if index == -1 {
                return 0;
            }
            let entry = *global::<u32>(TABLE.wrapping_add((index as u32).wrapping_mul(4)));
            if entry == 0 {
                return 0;
            }
            obj = entry;
        }
        *((obj + PED_FIELD) as *const u32)
    }
});
