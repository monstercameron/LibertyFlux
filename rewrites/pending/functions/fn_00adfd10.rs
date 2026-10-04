// original: 0x00adfd10 ui_resolve_table_handle
/// Resolve a handle through the shared object table.
///
/// Selects a record with the u16 at 0x1a; a live secondary pointer wins and
/// yields the word at 0xb4, otherwise the record's fallback at 0xc is
/// dereferenced when non-null, else the result is null.
export!(thiscall, rw_00adfd10(this: *const u8) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01295CD8;
        let sel = *(this.add(0x1a) as *const u16) as u32;
        let record = *(global::<u32>(TABLE).add(sel as usize));
        let live = *((record + 8) as *const u32);
        if live != 0 {
            *((live + 0xb4) as *const u32)
        } else {
            let fallback = *((record + 0x0c) as *const u32);
            if fallback != 0 {
                *(fallback as *const u32)
            } else {
                0
            }
        }
    }
});
