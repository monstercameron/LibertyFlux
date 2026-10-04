// original: 0x00908b60 blip_set_float_mode
/// Store a float into a handle-selected blip record.
///
/// Resolves `handle` to a blip id through the engine lookup, then copies the
/// float bits to +0x50 when `mode` is 0 or to +0x40 when `mode` is 0x10,
/// provided the record exists and has own data. Returns the resolved record
/// pointer, 0 for a null entry, or the lookup answer when negative.
export!(cdecl, rw_00908B60(mode: u32, handle: u32, fbits: u32) -> u32 {
    unsafe {
        let a = callee_cdecl!(1, u32, handle) as i32;
        if a < 0 {
            return a as u32;
        }
        let entry = blip(a as u32);
        if entry.is_null() {
            return 0;
        }
        if mode == 0 {
            if *entry.add(0x08) == 0 {
                return entry as u32;
            }
            *(entry.add(0x50) as *mut u32) = fbits;
        } else if mode == 0x10 {
            if *entry.add(0x08) == 0 {
                return entry as u32;
            }
            *(entry.add(0x40) as *mut u32) = fbits;
        }
        entry as u32
    }
});
