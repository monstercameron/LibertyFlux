// original: 0x009e95a0 ped_is_flagged_or_typed
/// True when the flag bit `0x20` at `+0x50` is set or the type word
/// at `+0x48` is anything but -1. (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e95a0(this_ptr: u32) -> u32 {
    unsafe {
        const TYPE_OFF: u32 = 0x48;
        const FLAG_OFF: u32 = 0x50;
        const FLAG_BIT: u8 = 0x20;
        if (this_ptr.wrapping_add(FLAG_OFF) as *const u8).read() & FLAG_BIT != 0 {
            1
        } else if (this_ptr.wrapping_add(TYPE_OFF) as *const u32).read_unaligned() != 0xFFFFFFFF {
            1
        } else {
            0
        }
    }
});
