// original: 0x00ab9a60 object_slot_search_field8

/// Object wrapper that searches the slot table reachable from an object.
///
/// Loads the table pointer stored 0x21c bytes into the object, steps past a
/// 0x80-byte header, and runs the two-record key search over it.
export!(cdecl, rs64_ab9a60(obj: *const u8, key: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x21c;
        const HEADER: u32 = 0x80;
        let table = *(((obj as u32).wrapping_add(TABLE_OFF)) as *const u32);
        let this = table.wrapping_add(HEADER);
        callee_thiscall!(0, u32, this, key)
    }
});
