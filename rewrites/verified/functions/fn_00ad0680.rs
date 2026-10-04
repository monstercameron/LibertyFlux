// original: 0x00ad0680 rmcInstanceWheelData::vf7
/// Copy a four-dword vector into the slots at 0x50..0x5c.
///
/// Virtual method that stores the caller's 16 bytes into the object.
/// Returns the last dword copied, as left in place by the original.
export!(thiscall, rw_00ad0680(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let s = src as *const u32;
        let d = this.add(0x50) as *mut u32;
        *d = *s;
        *d.add(1) = *s.add(1);
        *d.add(2) = *s.add(2);
        let last = *s.add(3);
        *d.add(3) = last;
        last
    }
});
