// original: 0x00db1980 UILayoutFrame::vf24
/// Store an integer slot and a float slot into the frame object.
///
/// Both words are copied bit-exactly; the float is moved, never rounded
/// or converted.
export!(thiscall, rw_00db1980(this_ptr: u32, slot: u32, fraction_bits: u32) -> u32 {
    unsafe {
        const INT_SLOT: usize = 0x14;
        const FLOAT_SLOT: usize = 0x18;
        let base = this_ptr as *mut u8;
        *(base.add(INT_SLOT) as *mut u32) = slot;
        *(base.add(FLOAT_SLOT) as *mut u32) = fraction_bits;
        0
    }
});
