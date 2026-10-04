// original: 0x00a64840 record_copy_stamp
/// Copies a four-word record into the object at +0x290.
///
/// The first three words after the leading dword move as float bits; the
/// copy is bitwise either way. The trailing field at +0x2A0 is stamped with
/// the shared global dword. Returns nothing.
export!(thiscall, rw_00a64840(this: u32, src: u32) -> u32 {
    unsafe {
        *((this + 0x290) as *mut u32) = *(src as *const u32);
        *((this + 0x294) as *mut u32) = *((src + 4) as *const u32);
        *((this + 0x298) as *mut u32) = *((src + 8) as *const u32);
        *((this + 0x29C) as *mut u32) = *((src + 0xC) as *const u32);
        *((this + 0x2A0) as *mut u32) = *global::<u32>(0x11735B4);
    }
    0
});
