// original: 0x00CD28D0 read_bit6_byte_at_f0
/// Returns bit 6 of the byte stored at offset `0xF0` from the receiver.
lf_checker_rt::export!(thiscall, rw_fn_00cd28d0(this_ptr: u32) -> u32 {
    unsafe { (((this_ptr as *const u8).add(0xF0).read() >> 6) & 1) as u32 }
});
