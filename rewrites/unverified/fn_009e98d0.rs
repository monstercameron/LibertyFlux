// original: 0x009e98d0 ped_mode_bits_set
/// True when the mode word at `+0x20` of the resolved entry (key
/// at `+0x18`) has both bit 5 and bit 12 set. (thiscall; low byte.)
lf_checker_rt::export!(thiscall, rw_009e98d0(this_ptr: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x18;
        const MODE_OFF: u32 = 0x20;
        let key = (this_ptr.wrapping_add(KEY_OFF) as *const u32).read_unaligned();
        let entry: u32 = lf_checker_rt::callee_cdecl!(1, u32, key);
        let mode = (entry.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
        if mode & (1 << 5) != 0 && mode & (1 << 12) != 0 { 1 } else { 0 }
    }
});
