// original: 0x005e7420 SET_OBJECT_USED_IN_POOL_GAME
/// Script native `SET_OBJECT_USED_IN_POOL_GAME` (hash 0x07B23203).
///
/// Resolves the object handle in arg0 through the object pool: the low byte is
/// the slot tag, the remaining bits the slot index. If the tag matches and the
/// slot is occupied, sets (arg1 != 0) or clears (arg1 == 0) the in-pool-game
/// flag (bit 21) of the object's word at offset 0x210. Mismatched or empty
/// slots leave everything untouched. No engine call is made.
export!(cdecl, rw_005e7420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
                let handle = *args;
                let set = *args.add(1) != 0;
                // Object pool header: [0] entry base, [1] tag array, [3] entry stride.
                let pool = *(global::<u32>(0x1632c60) as *const u32);
                let tags = *((pool + 4) as *const u32);
                let index = (((handle as i32) >> 8) as u32);
                let tag = *((tags.wrapping_add(index)) as *const u8);
                if tag != (handle & 0xFF) as u8 {
                    return tags;
                }
                let stride = *((pool + 12) as *const u32);
                let entry_base = *(pool as *const u32);
                let entry = entry_base.wrapping_add(index.wrapping_mul(stride));
                if entry == 0 {
                    return tags;
                }
                // Set or clear the in-pool-game flag (bit 21) of the object's word at +0x210.
                const FLAG: u32 = 0x200000;
                const CELL_OFF: u32 = 0x210;
                let cell = (entry + CELL_OFF) as *mut u32;
                let old = *cell;
                let want = if set { FLAG } else { 0 };
                let diff = (want ^ old) & FLAG;
                *cell = old ^ diff;
                diff
    }
});
