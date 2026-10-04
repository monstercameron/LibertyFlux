// original: 0x00db1aa0 UILayoutFrame::vf54
/// Install a pointer into the frame's cache slot if it is still empty.
///
/// A non-null slot is left untouched, so the first installed value wins.
export!(thiscall, rw_00db1aa0(this_ptr: u32, value: u32) -> u32 {
    unsafe {
        const CACHE_SLOT: usize = 0xec;
        let slot = (this_ptr as *mut u8).add(CACHE_SLOT) as *mut u32;
        if *slot == 0 {
            *slot = value;
        }
        0
    }
});
