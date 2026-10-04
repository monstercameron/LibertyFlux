// original: 0x00db1970 UILayoutFrame::vf77
/// Store a flag byte into the frame object.
///
/// Only the low byte of the argument is kept; the rest of the word is
/// ignored. Part of a family of one-byte setters on this object.
export!(thiscall, rw_00db1970(this_ptr: u32, value: u32) -> u32 {
    unsafe {
        const FLAG: usize = 0xc9;
        *((this_ptr as *mut u8).add(FLAG)) = value as u8;
        0
    }
});
