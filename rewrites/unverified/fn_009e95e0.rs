// original: 0x009e95e0 ped_has_big_flag_a
/// True when the flag query (`0x20000000`, mode 1) on
/// `[this+0x78]` reports set. (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e95e0(this_ptr: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x78;
        const QUERY_MASK: u32 = 0x20000000;
        const QUERY_MODE: u32 = 1;
        let inner = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, inner, QUERY_MASK, QUERY_MODE);
        if r == 0 { 0 } else { 1 }
    }
});
