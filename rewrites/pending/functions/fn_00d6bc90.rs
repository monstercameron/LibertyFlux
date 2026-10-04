// original: 0x00d6bc90 progress_state_zero_init
/// Zero the progress-state fields of the object at `this_ptr` and return it.
///
/// Writes zeros to six dwords at +0x00..+0x17, a word at +0x20, dwords at
/// +0x24/+0x28 and four qwords at +0x2C/+0x34/+0x4C/+0x54. All other bytes
/// (including +0x1C, +0x22 and +0x3C..+0x4B) are left untouched.
lf_checker_rt::export!(thiscall, rw_00d6bc90(this_ptr: u32) -> u32 {
    unsafe {
        let b = this_ptr as *mut u8;
        *((b.add(0x00)) as *mut u32) = 0;
        *((b.add(0x04)) as *mut u32) = 0;
        *((b.add(0x08)) as *mut u32) = 0;
        *((b.add(0x0c)) as *mut u32) = 0;
        *((b.add(0x10)) as *mut u32) = 0;
        *((b.add(0x14)) as *mut u32) = 0;
        *((b.add(0x20)) as *mut u16) = 0;
        *((b.add(0x24)) as *mut u32) = 0;
        *((b.add(0x28)) as *mut u32) = 0;
        *((b.add(0x2c)) as *mut u64) = 0;
        *((b.add(0x34)) as *mut u64) = 0;
        *((b.add(0x4c)) as *mut u64) = 0;
        *((b.add(0x54)) as *mut u64) = 0;
        this_ptr
    }
});
