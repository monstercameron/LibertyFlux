// original: 0x00908e90 BLIP_PICK
/// Pick the blip under the cursor.
///
/// Runs a picker call over a zeroed three-word block, resolves the picked
/// handle through a lookup call, then clears per-state fields on the picked
/// entry while its active flag is set, notifies the blip system, and marks
/// the entry picked.
///
/// The contract skips the picker call's block address and snapshots its three
/// (zero) words; all other call arguments are compared by value.
export!(cdecl, rw_00908e90() -> u32 {
    unsafe {
        let blk = [0u32; 3];
        let picked = callee_cdecl!(1, u32, 1, 8, 0xffffffff, blk.as_ptr() as u32,
                                   2, relocated(0xe84b98));
        let current = global::<u32>(0x1034494);
        *current = picked;
        let index = callee_cdecl!(2, u32, picked);
        *current = index;
        let table = relocated(0x118f6f8) as *const u32;
        let mut id = *current;
        let mut entry = *table.add(id as usize);
        if *((entry + 8) as *const u8) != 0 {
            *((entry + 0x44) as *mut u32) = 0;
            id = *current;
            entry = *table.add(id as usize);
        }
        if *((entry + 8) as *const u8) != 0 {
            *((entry + 0x50) as *mut u32) = 0x3ecccccd;
            id = *current;
            entry = *table.add(id as usize);
        }
        if *((entry + 8) as *const u8) != 0 {
            *((entry + 0x54) as *mut u32) = 0x37;
            id = *current;
        }
        callee_cdecl!(3, u32, id, relocated(0xe84ba4));
        id = *current;
        entry = *table.add(id as usize);
        let flags = (entry + 0x20) as *mut u16;
        *flags |= 8;
        entry
    }
});
