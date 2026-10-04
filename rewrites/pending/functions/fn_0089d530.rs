// original: 0x0089d530 audio_table_call_handler
/// Audio table forward: resolve the entry selected by the object's row/column
/// bytes and invoke the handler (thiscall/1) on it with the extra argument.
/// A column byte of 0xFF, or an empty entry, returns without calling.
/// cdecl/2; the defined result is the low byte, upper bytes pass through.
export!(cdecl, rw_0089d530(obj: u32, arg: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const TABLE_OFF: u32 = 0x6F10;
        const ROW_IDX: u32 = 0x40;
        const COL_IDX: u32 = 0x48;
        const STRIDE: u32 = 0x115D964;
        const TABLE: u32 = 0x115D988;
        const ABSENT: u32 = 0xFF;
        let col = *((obj + COL_IDX) as *const u8) as u32;
        if col == ABSENT {
            return obj & 0xFFFF_FF00;
        }
        let row = *((obj + ROW_IDX) as *const u8) as u32;
        let stride = *global::<u32>(STRIDE);
        let table = *global::<u32>(TABLE);
        let slot = table
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_OFF);
        let entry = (slot as *const u32).read_unaligned();
        let ptr = col.wrapping_mul(stride).wrapping_add(entry);
        if ptr == 0 {
            return table & 0xFFFF_FF00;
        }
        let target: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        target(ptr, arg)
    }
});
