// original: 0x006961f0 channel_adopt_count
/// Adopt an element count, allocating the list on first use.
///
/// The first call records the count and allocates; later calls only update
/// the live count. Returns nothing meaningful.
export!(thiscall, rs80_6961f0(this: *mut u8, n: u32) -> u32 {
    unsafe {
        let w = n as u16;
        if *((this).add(6) as *const u16) == 0 {
            *((this).add(6) as *mut u16) = w;
            if n != 0 {
                let p: u32 = callee_stdcall!(1, u32, n);
                *((this).add(4) as *mut u16) = w;
                *((this).add(0) as *mut u32) = p;
            } else {
                *((this).add(4) as *mut u16) = 0;
                *((this).add(0) as *mut u32) = 0;
            }
        } else {
            *((this).add(4) as *mut u16) = w;
        }
        0 // unchecked: the original leaks entry-EAX residue on one path
    }
});
