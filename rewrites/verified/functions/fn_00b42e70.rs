// original: 0x00b42e70 take_slot_and_tail_dispatch
/// Take a pending slot index and hand it to the dispatcher.
///
/// Reads the object's kind field: kinds 4 and 2 surrender the index stored
/// at their slot (clearing it to 0xFFFF); other kinds return the kind
/// itself. An index at or past 0x200 is returned as is, otherwise the
/// dispatcher is tail-called with it.
export!(cdecl, rw_b42e70(obj: u32) -> u32 {
    unsafe {
        let kind = (((obj as *const u32).byte_add(0x28).read() >> 6) & 0xf);
        let idx: u32;
        if kind == 4 {
            let slot = (obj as *mut u16).byte_add(0x220);
            idx = slot.read() as u32;
            slot.write(0xFFFF);
        } else if kind == 2 {
            let slot = (obj as *mut u16).byte_add(0x1074);
            idx = slot.read() as u32;
            slot.write(0xFFFF);
        } else {
            return kind;
        }
        if idx >= 0x200 {
            return idx;
        }
        callee_cdecl!(2, u32, idx)
    }
});
