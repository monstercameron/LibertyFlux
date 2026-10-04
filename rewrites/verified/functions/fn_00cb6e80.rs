// original: 0x00cb6e80 resolver_tail_call
/// Local dword at +0x24, or the resolver's answer for [this+0x20]+8.
export!(thiscall, rw_cb6e80(this_ptr: u32) -> u32 {
    unsafe {
        let holder_flag = ((this_ptr.wrapping_add(0xB4)) as *const u8).read();
        if holder_flag & 1 == 0 {
            ((this_ptr.wrapping_add(0x24)) as *const u32).read()
        } else {
            let inner = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
            lf_checker_rt::callee_thiscall!(1, u32, inner.wrapping_add(8))
        }
    }
});
