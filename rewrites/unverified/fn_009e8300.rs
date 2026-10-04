// original: 0x009e8300 ped_flagged_subobject_or_null
/// Pointer to the sub-object at `+0xE20` when the flag byte at
/// `+0xE10` is non-zero, else null. (thiscall, no stack arguments.)
lf_checker_rt::export!(thiscall, rw_009e8300(this_ptr: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0xE10;
        const SUB_OFF: u32 = 0xE20;
        if (this_ptr.wrapping_add(FLAG_OFF) as *const u8).read() != 0 {
            this_ptr.wrapping_add(SUB_OFF)
        } else {
            0
        }
    }
});
