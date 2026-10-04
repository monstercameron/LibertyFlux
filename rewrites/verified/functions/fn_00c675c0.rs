// original: 0x00c675c0 cutscene_store_slot_guarded
/// Stores `value` into slot `index` of the 10-slot table at 0x2b8.
///
/// The original compares the index signed against 10 and stores for any
/// smaller value, including negatives (which land just below the table).
export!(thiscall, rw_c675c0(this: u32, index: u32, value: u32) -> u32 {
    let index = index as i32;
    if index < 10 {
        unsafe {
            (this as *mut u32)
                .byte_add(0x2b8)
                .offset(index as isize)
                .write(value);
        }
    }
    value
});
