// original: 0x00963080 slot_table_store
/// Store a pointer into one of two 0x818-entry slot tables.
///
/// Ignores null pointers, negative indexes and indexes past the table end.
/// Otherwise writes the pointer and the index's high word as a tag into the
/// selected table (0x1208970 when the selector is 1, else 0x12142D0).
/// Returns nothing observable (early exits leak entry EAX).
export!(cdecl, rw_00963080(which: u32, index: u32, ptr: u32) -> () {
    unsafe {
        if ptr == 0 {
            return;
        }
        if (index as i32) < 0 {
            return;
        }
        if (index as u16) >= 0x818 {
            return;
        }
        let base = if which == 1 {
            relocated(0x1208970)
        } else {
            relocated(0x12142D0)
        };
        let row = (index & 0xFFFF) as usize;
        *((base as *mut u32).add(row * 2)) = ptr;
        *((base as *mut u16).add(row * 4 + 2)) = (index >> 16) as u16;
    }
});
