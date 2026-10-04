// original: 0x00b6eb60 vehicle_door_table_lookup (proposed)
/// Look up a door/animation word from a vehicle-side table.
///
/// `this` points to the task: dword at `+0x24` points to the table record,
/// dword at `+0x28` is the selector. When the selector is 5 the answer is the
/// word at `+0xf50` of the record. Otherwise the index mapper (callee 1,
/// cdecl/2: record pointer, selector) translates the selector and the answer
/// is the word at `+0xf54 + index*4`.
///
/// Original: 0x00b6eb60 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00b6eb60(this: u32) -> u32 {
    unsafe {
        const RECORD_OFF: u32 = 0x24;
        const SELECTOR_OFF: u32 = 0x28;
        const DIRECT_SEL: u32 = 5;
        const DIRECT_SLOT: u32 = 0xf50;
        const TABLE_BASE: u32 = 0xf54;
        let rec = ((this + RECORD_OFF) as *const u32).read_unaligned();
        let sel = ((this + SELECTOR_OFF) as *const u32).read_unaligned();
        if sel == DIRECT_SEL {
            return ((rec + DIRECT_SLOT) as *const u32).read_unaligned();
        }
        let idx: u32 = lf_checker_rt::callee_cdecl!(1, u32, rec, sel);
        ((rec + TABLE_BASE + idx.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});

