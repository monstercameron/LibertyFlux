// original: 0x00c2d780 store_handle_refresh
/// Conditional handle store, cached-value refresh, marker clear.
///
/// Stores the first argument at +0x2C only when the second argument's
/// low byte is nonzero, then refreshes +0x28 from +0x1C, clears the
/// marker byte and returns the refreshed value.
export!(thiscall, rw_00c2d780(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        if (b as u8) != 0 {
            *(this.add(0x2C) as *mut u32) = a;
        }
        let v = *(this.add(0x1C) as *const u32);
        *(this.add(0x28) as *mut u32) = v;
        *this.add(0x30) = 0;
        v
    }
});
